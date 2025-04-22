use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use tinymist_world::{config::CompileOpts, EntryOpts, ShadowApi, TypstSystemUniverse};
use typst::foundations::Bytes;

pub fn make_universe(content: &str) -> anyhow::Result<TypstSystemUniverse> {
    let root = os_root();
    let mut univ = TypstSystemUniverse::new(CompileOpts {
        entry: EntryOpts::new_rooted(root.clone(), Some(PathBuf::from("/main.typ"))),
        with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
        ..Default::default()
    })?
    .with_entry_file(root.join("main.typ"));
    univ.map_shadow(
        &root.join("main.typ"),
        Bytes::from_string(content.to_string()),
    )?;
    Ok(univ)
}

pub fn make_universe_formatted(
    source_dir: &Path,
    entrypoint: &Path,
    blacklist: &HashSet<String>,
    formatter: fn(&str, &Path) -> String,
) -> anyhow::Result<(TypstSystemUniverse, TypstSystemUniverse)> {
    let root = os_root();
    let entry_file = root.join(
        entrypoint
            .strip_prefix(source_dir)
            .context("entrypoint is not within the testcase_dir")?,
    );

    let make_world = || -> anyhow::Result<TypstSystemUniverse> {
        let univ = TypstSystemUniverse::new(CompileOpts {
            entry: EntryOpts::new_rooted(root.clone(), Some(entrypoint.to_path_buf())),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..Default::default()
        })?
        .with_entry_file(entry_file.to_path_buf());
        Ok(univ)
    };

    let mut world = make_world()?;
    let mut formatted_world = make_world()?;

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

    // map all files within the testcase_dir
    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let rel_path = path.strip_prefix(source_dir)?;
        let full_path = root.join(rel_path);
        let content = Bytes::new(fs::read(path)?);
        if path.extension() == Some("typ".as_ref()) && !is_blacklisted(path, source_dir, blacklist)
        {
            let content = Bytes::new(strip_trailing_whitespace(content.as_str()?));
            world.map_shadow(&full_path, content.clone())?;
            formatted_world.map_shadow(
                &full_path,
                Bytes::new(formatter(content.as_str()?, rel_path)),
            )?;
        } else {
            world.map_shadow(&full_path, content.clone())?;
            formatted_world.map_shadow(&full_path, content)?;
        }
    }
    Ok((world, formatted_world))
}

fn os_root() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    }
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
