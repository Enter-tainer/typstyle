use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use tinymist_world::{
    config::CompileOpts, EntryOpts, EntryReader, ShadowApi, TaskInputs, TypstSystemUniverse,
    TypstSystemWorld,
};
use typst::{diag::SourceDiagnostic, ecow::EcoVec, foundations::Bytes, layout::PagedDocument};
use walkdir::WalkDir;

use crate::cmp::{CompiledPair, DiffSink};

type Formatter<'a> = Box<dyn Fn(&str) -> Result<String> + 'a>;

pub struct TypstyleUniverse<'a> {
    name: String,
    project_root: PathBuf,
    formatter: Formatter<'a>,
    orig_univ: TypstSystemUniverse,
    fmt_univ: TypstSystemUniverse,
    sink: DiffSink,
}

impl<'a> TypstyleUniverse<'a> {
    pub fn new(
        name: String,
        project_root: PathBuf,
        formatter: impl Fn(&str) -> Result<String> + 'a,
    ) -> Result<Self> {
        fn make_univ() -> Result<TypstSystemUniverse> {
            Ok(TypstSystemUniverse::new(CompileOpts {
                entry: EntryOpts::new_workspace(PathBuf::from(".")),
                with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
                ..Default::default()
            })?)
        }

        Ok(Self {
            name,
            project_root,
            formatter: Box::new(formatter),
            orig_univ: make_univ()?,
            fmt_univ: make_univ()?,
            sink: DiffSink::new(),
        })
    }

    pub fn add_all_files(
        &mut self,
        source_dir: &Path,
        blacklist: &HashSet<String>,
    ) -> Result<&mut Self> {
        // map all files within the testcase_dir
        let walk = WalkDir::new(source_dir)
            .into_iter()
            .filter_entry(|e| e.file_name().to_str() != Some(".git"));
        for entry in walk {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let vpath = path.strip_prefix(&self.project_root)?;

            let content = Bytes::new(fs::read(path)?);
            if path.extension() == Some("typ".as_ref())
                && !is_blacklisted(path, source_dir, blacklist)
            {
                self.add_fmt_file(vpath, content)?;
            } else {
                self.add_raw_file(vpath, content)?;
            }
        }

        Ok(self)
    }

    pub fn add_all_files_in_one(
        &mut self,
        one_path: &Path,
        source_dir: &Path,
    ) -> Result<&mut Self> {
        let mut entry_content = String::new();
        // map all files within the testcase_dir
        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            if !(entry.file_type().is_file() && entry.path().extension() == Some("typ".as_ref())) {
                continue;
            }

            let vpath = entry.path().strip_prefix(&self.project_root)?;
            let include_path = vpath.to_str().unwrap().replace('\\', "/");
            entry_content.push_str(&format!("#include \"{}\"\n", include_path));
        }
        self.add_source_file(one_path, entry_content)
            .with_context(|| format!("failed to add all-in-one file at {}", one_path.display()))?;

        Ok(self)
    }

    pub fn add_source_file(&mut self, path: &Path, content: String) -> Result<&mut Self> {
        self.add_fmt_file(path, Bytes::from_string(content))?;

        Ok(self)
    }

    fn add_fmt_file(&mut self, vpath: &Path, content: Bytes) -> Result<()> {
        let vpath = &vroot().join(vpath);

        let content_str = strip_trailing_whitespace(content.as_str()?);
        let formatted = (self.formatter)(&content_str);
        let fmt_content = Bytes::from_string(match formatted {
            Ok(res) => res,
            Err(err) => {
                self.sink.push(format!(
                    "failed to format file at `{}`: {}",
                    vpath.display(),
                    err
                ));
                content_str.clone()
            }
        });
        let orig_content = Bytes::from_string(content_str);

        self.orig_univ
            .map_shadow(vpath, orig_content)
            .with_context(|| {
                format!(
                    "failed to map file in the original world: {}",
                    vpath.display()
                )
            })?;
        self.fmt_univ
            .map_shadow(vpath, fmt_content)
            .with_context(|| {
                format!(
                    "failed to map file in the format world: {}",
                    vpath.display()
                )
            })?;

        Ok(())
    }

    fn add_raw_file(&mut self, vpath: &Path, content: Bytes) -> Result<()> {
        let vpath = &vroot().join(vpath);
        self.orig_univ.map_shadow(vpath, content.clone())?;
        self.fmt_univ.map_shadow(vpath, content)?;

        Ok(())
    }

    pub fn compile_with_entry(&self, entry: &Path) -> CompiledPair {
        let entry = &vroot().join(entry);

        fn compile_impl(univ: &TypstSystemUniverse, entry: &Path, name: String) -> Compiled {
            let world = univ.snapshot_with(Some(TaskInputs {
                entry: Some(univ.entry_state().select_in_workspace(entry)),
                ..Default::default()
            }));
            let result = typst::compile(&world).output;
            Compiled {
                name,
                world,
                result,
            }
        }

        CompiledPair::new(
            compile_impl(&self.orig_univ, entry, format!("{}-original", self.name)),
            compile_impl(&self.fmt_univ, entry, format!("{}-formatted", self.name)),
        )
    }

    pub fn sink(&self) -> &DiffSink {
        &self.sink
    }

    pub fn sink_mut(&mut self) -> &mut DiffSink {
        &mut self.sink
    }
}

fn is_blacklisted(path: &Path, source_dir: &Path, blacklist: &HashSet<String>) -> bool {
    // Get the relative path starting at source_dir.
    if let Ok(rel_path) = path.strip_prefix(source_dir) {
        // Return true if any component's name is in the blacklist.
        return rel_path
            .components()
            .filter_map(|comp| {
                if let std::path::Component::Normal(os_str) = comp {
                    os_str.to_str()
                } else {
                    None
                }
            })
            .any(|name| blacklist.contains(name));
    }
    false
}

pub struct Compiled {
    pub name: String,
    pub world: TypstSystemWorld,
    pub result: Result<PagedDocument, EcoVec<SourceDiagnostic>>,
}

fn vroot() -> &'static Path {
    Path::new(".")
}

fn strip_trailing_whitespace(s: &str) -> String {
    if s.is_empty() {
        return "\n".to_string();
    }
    let mut res = String::with_capacity(s.len());
    for line in s.lines() {
        res.push_str(line.trim_end());
        res.push('\n');
    }
    res
}
