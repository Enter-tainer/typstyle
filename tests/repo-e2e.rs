use std::{
    borrow::Cow,
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{bail, Context};
use itertools::Itertools;
use libtest_mimic::{Arguments, Failed, Trial};
use reflexo_typst::{
    config::CompileOpts, error::diag_from_std, foundations::Smart, typst::prelude::EcoVec,
    world::EntryOpts, CompileDriver, ShadowApi, TypstDocument, TypstSystemUniverse, TypstWorld,
};
use typst::diag::SourceDiagnostic;
use typstyle_core::Typstyle;

fn main() -> anyhow::Result<()> {
    let args = Arguments::from_args();
    let tests = collect_tests()?;
    libtest_mimic::run(&args, tests).exit();
}

#[derive(Debug, Clone)]
struct Testcase {
    name: Cow<'static, str>,
    repo_url: Cow<'static, str>,
    revision: Cow<'static, str>,
    entrypoint: Cow<'static, str>,
    blacklist: HashSet<String>,
}

impl Testcase {
    fn new(
        name: impl Into<Cow<'static, str>>,
        repo_url: impl Into<Cow<'static, str>>,
        revision: impl Into<Cow<'static, str>>,
        entrypoint: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            name: name.into(),
            repo_url: repo_url.into(),
            revision: revision.into(),
            entrypoint: entrypoint.into(),
            blacklist: HashSet::new(),
        }
    }

    fn with_blacklist(mut self, blacklist: impl Iterator<Item = impl Into<String>>) -> Self {
        self.blacklist = blacklist.map(Into::into).collect();
        self
    }
}

fn collect_tests() -> anyhow::Result<Vec<Trial>> {
    let cases = [
        Testcase::new(
            "tutorial",
            "https://github.com/typst-doc-cn/tutorial",
            "06e69d94c6ecd42a9af76c252a0a0eb47d93b6b8",
            "src/ebook.typ",
        ),
        Testcase::new(
            "uniquecv",
            "https://github.com/gaoachao/uniquecv-typst",
            "38fd15c5156e683f989fcf5a04b119b8a1d22f2e",
            "main.typ",
        ),
        Testcase::new(
            "cetz-manual",
            "https://github.com/cetz-package/cetz",
            "f78e2a0382442ac45584d1bed39665db6fbebc5b",
            "manual.typ",
        ),
        Testcase::new(
            "HomotopyHistory",
            "https://github.com/trebor-Huang/HomotopyHistory/",
            "5ee6ff5f9b3e1dccae3b84cfb093cf5d649bc12c",
            "paper.typ",
        ),
        Testcase::new(
            "typst-talk",
            "https://github.com/OrangeX4/typst-talk",
            "742a0c614c0163dee557b101fb8e4e4063d51fd3",
            "main.typ",
        )
        .with_blacklist(["chicv.typ"].into_iter()),
        Testcase::new(
            "touying-example",
            "https://github.com/touying-typ/touying",
            "9b6dc9db5a15837696c5cbfdcc62b21f8d4f3f83",
            "examples/example.typ",
        ),
        Testcase::new(
            "tablex-test",
            "https://github.com/PgBiel/typst-tablex",
            "940d13c570f241a8a9de7512c453deaee29952e5",
            "tablex-test.typ",
        ),
        Testcase::new(
            "fletcher-manual",
            "https://github.com/Jollywatt/typst-fletcher",
            "1859ac08998a14083ac57b73ee684c9dfa3375f7",
            "docs/manual.typ",
        )
        // tidy has weird behavior when parsing typ source code
        .with_blacklist(["main.typ", "marks.typ"].into_iter()),
        Testcase::new(
            "nju-thesis-typst",
            "https://github.com/nju-lug/nju-thesis-typst",
            "8b481bddd3bfa683a5af2e22922e222d8d5d0f81",
            "thesis.typ",
        ),
        Testcase::new(
            "physica",
            "https://github.com/Leedehai/typst-physics",
            "1d683a041d2361fc4078e419d17cb9ada60dadd7",
            "physica-manual.typ",
        ),
        Testcase::new(
            "lovelace",
            "https://github.com/andreasKroepelin/lovelace",
            "a83b64662b1a6f78593b8e028e9a8162f1793d4c",
            "examples/doc.typ",
        ),
        Testcase::new(
            "quill",
            "https://github.com/Mc-Zen/quill",
            "3cd5f656c3c6845e267621d1d118d6c8f7731f37",
            "docs/guide/quill-guide.typ",
        )
        // these files are included as-is and should not be formatted
        .with_blacklist(
            [
                "shor-nine-qubit-code.typ",
                "teleportation.typ",
                "phase-estimation.typ",
                "qft.typ",
                "fault-tolerant-measurement.typ",
                "fault-tolerant-pi8.typ",
                "fault-tolerant-toffoli1.typ",
                "fault-tolerant-toffoli2.typ",
                "quill-guide.typ",
            ]
            .into_iter(),
        ),
        Testcase::new(
            "curryst",
            "https://github.com/pauladam94/curryst",
            "0f8f1330d6a73acbf0f0d03f2e9073af0dcee173",
            "examples/natural-deduction.typ",
        ),
    ];
    Ok(cases
        .into_iter()
        .map(|case| {
            Trial::test(format!("{} - e2e", case.name.clone()), move || {
                run_test_case(case.clone()).map_err(|e| Failed::from(e.to_string()))
            })
        })
        .collect())
}

fn run_test_case(testcase: Testcase) -> anyhow::Result<()> {
    clone_test_case(&testcase)?;
    compile_and_format_test_case(&testcase)?;
    let testcase_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/e2e")
        .join(&*testcase.name);
    let _ = fs::remove_dir_all(testcase_dir);
    Ok(())
}

fn clone_test_case(testcase: &Testcase) -> anyhow::Result<()> {
    // clone the repo
    // checkout the revision
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // do mkdir -p project_root/tests/e2e
    let e2e_dir = project_root.join("tests/e2e");
    let testcase_dir = e2e_dir.join(&*testcase.name);
    let _ = fs::create_dir_all(&e2e_dir);
    // clean the testcase_dir
    let _ = fs::remove_dir_all(&testcase_dir);
    // do git clone with submodule
    // do git checkout testcase.revision
    std::process::Command::new("git")
        .arg("clone")
        .arg(&*testcase.repo_url)
        .arg(&testcase_dir)
        .arg("--recurse-submodules")
        .output()
        .context("failed to clone repo")?;
    std::process::Command::new("git")
        .arg("checkout")
        .arg(&*testcase.revision)
        .current_dir(&testcase_dir)
        .output()
        .context("failed to checkout revision")?;
    Ok(())
}

fn compile_and_format_test_case(testcase: &Testcase) -> anyhow::Result<()> {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let e2e_dir = project_root.join("tests/e2e");
    let testcase_dir = e2e_dir.join(&*testcase.name);
    let entrypoint = testcase_dir.join(&*testcase.entrypoint);
    let root = if cfg!(windows) {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    };
    let make_world = || -> anyhow::Result<TypstSystemUniverse> {
        Ok(TypstSystemUniverse::new(CompileOpts {
            entry: EntryOpts::new_rooted(root.clone(), Some(entrypoint.clone())),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..Default::default()
        })?)
    };
    let mut world = make_world()?;
    let mut formatted_world = make_world()?;
    // map all files within the testcase_dir
    for entry in walkdir::WalkDir::new(&testcase_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            let rel_path = path.strip_prefix(&testcase_dir)?;
            let content = fs::read(path)?;
            world.map_shadow(&root.join(rel_path), content.clone().into())?;
            formatted_world.map_shadow(
                &root.join(rel_path),
                if path.extension() == Some("typ".as_ref())
                    && !testcase.blacklist.contains(
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .as_str(),
                    )
                {
                    let content = String::from_utf8(content)?;
                    let doc = Typstyle::new_with_content(content, 80).pretty_print();
                    let second_format = Typstyle::new_with_content(doc.clone(), 80).pretty_print();
                    pretty_assertions::assert_eq!(
                        doc,
                        second_format,
                        "The file {} is not converging after formatting",
                        rel_path.display()
                    );
                    doc.as_bytes().into()
                } else {
                    content.into()
                },
            )?;
        }
    }
    let entry_file = root.join(
        entrypoint
            .strip_prefix(&testcase_dir)
            .context("entrypoint is not within the testcase_dir")?,
    );
    let c = std::marker::PhantomData;
    let world = world.with_entry_file(entry_file.clone());
    let mut driver = CompileDriver::new(c, world);
    let formatted_world = formatted_world.with_entry_file(entry_file);
    let mut formatted_driver = CompileDriver::new(c, formatted_world);
    let doc = driver.compile(&mut Default::default());
    let formatted_doc = formatted_driver.compile(&mut Default::default());
    compare_docs(
        &testcase.name,
        doc,
        &driver.snapshot(),
        formatted_doc,
        &formatted_driver.snapshot(),
    )?;
    Ok(())
}

fn compare_docs(
    name: &str,
    doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
    world: &dyn TypstWorld,
    formatted_doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
    formatted_world: &dyn TypstWorld,
) -> anyhow::Result<()> {
    match (doc, formatted_doc) {
        (Ok(doc), Ok(formatted_doc)) => {
            let pdf = typst_pdf::pdf(&doc, Smart::Custom("original"), None, None);
            let formatted_pdf =
                typst_pdf::pdf(&formatted_doc, Smart::Custom("formatted"), None, None);
            // write both pdf to tmp path
            let tmp_dir = env::temp_dir();
            let pdf_path = tmp_dir.join(format!("{name}-{}.pdf", "original"));
            let formatted_pdf_path = tmp_dir.join(format!("{name}-{}.pdf", "formatted"));
            std::fs::write(&pdf_path, pdf).context("failed to write pdf")?;
            std::fs::write(&formatted_pdf_path, formatted_pdf)
                .context("failed to write formatted pdf")?;
            let message = format!(
                "The pdfs are written to \"{}\" and \"{}\"",
                pdf_path.display(),
                formatted_pdf_path.display()
            );
            pretty_assertions::assert_eq!(
                doc.pages.len(),
                formatted_doc.pages.len(),
                "The page counts are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.title,
                formatted_doc.info.title,
                "The titles are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.author,
                formatted_doc.info.author,
                "The authors are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.keywords,
                formatted_doc.info.keywords,
                "The keywords are not consistent. {message}"
            );

            for (i, (doc, formatted_doc)) in
                doc.pages.iter().zip(formatted_doc.pages.iter()).enumerate()
            {
                let png = typst_render::render(doc, 2.0);
                let formatted_png = typst_render::render(formatted_doc, 2.0);
                if png != formatted_png {
                    // save both to tmp path and report error
                    let tmp_dir = env::temp_dir();
                    let png_path = tmp_dir.join(format!("{name}-{}-{}.png", i, "original"));
                    let formatted_png_path =
                        tmp_dir.join(format!("{name}-{}-{}.png", i, "formatted"));
                    png.save_png(&png_path).unwrap();
                    formatted_png.save_png(&formatted_png_path).unwrap();
                    bail!(
                        "The output are not consistent for page {}, original png path: \"{}\", formatted png path: \"{}\"",
                        i, png_path.display(), formatted_png_path.display()
                    );
                }
            }
        }
        (Err(e1), Err(e2)) => {
            pretty_assertions::assert_eq!(
                e1.len(),
                e2.len(),
                "The error counts are not consistent"
            );
            for (e1, e2) in e1.iter().zip(e2.iter()) {
                pretty_assertions::assert_eq!(
                    e1.message,
                    e2.message,
                    "The error messages are not consistent after formatting"
                );
            }
        }
        (_, Err(res2)) => {
            let diag = res2
                .into_iter()
                .map(|e| diag_from_std(e, Some(formatted_world)))
                .collect_vec();
            bail!("Formatted doc failed to compile: {:#?}", diag);
        }
        (Err(res1), _) => {
            let diag = res1
                .into_iter()
                .map(|e| diag_from_std(e, Some(world)))
                .collect_vec();
            bail!("Original doc failed to compile: {:#?}", diag);
        }
    }
    Ok(())
}
