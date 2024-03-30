#![allow(dead_code)]
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{bail, Context};
use itertools::Itertools;
use libtest_mimic::{Arguments, Failed, Trial};
use typst_ts_compiler::{
    service::{CompileDriver, Compiler},
    ShadowApi, TypstSystemWorld,
};
use typst_ts_core::{
    config::{compiler::EntryOpts, CompileOpts},
    diag::SourceDiagnostic,
    error::diag_from_std,
    typst::prelude::EcoVec,
    TypstDocument, TypstWorld,
};
use typstyle_core::pretty_print;

fn main() -> anyhow::Result<()> {
    let _args = Arguments::from_args();
    // let tests = collect_tests()?;
    // libtest_mimic::run(&args, tests).exit();
    Ok(())
}

#[derive(Debug, Clone)]
struct Testcase {
    name: Cow<'static, str>,
    repo_url: Cow<'static, str>,
    revision: Cow<'static, str>,
    entrypoint: Cow<'static, str>,
}

fn collect_tests() -> anyhow::Result<Vec<Trial>> {
    let cases = [
        Testcase {
            name: "tutorial".into(),
            repo_url: "https://github.com/typst-doc-cn/tutorial".into(),
            revision: "5ddb7edd309e2d7fb90486b9885e93b267aa464c".into(),
            entrypoint: "src/ebook.typ".into(),
        },
        Testcase {
            name: "uniquecv".into(),
            repo_url: "https://github.com/gaoachao/uniquecv-typst".into(),
            revision: "38fd15c5156e683f989fcf5a04b119b8a1d22f2e".into(),
            entrypoint: "main.typ".into(),
        },
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
    let make_world = || -> anyhow::Result<TypstSystemWorld> {
        Ok(TypstSystemWorld::new(CompileOpts {
            entry: EntryOpts::new_rooted(root.clone(), Some(entrypoint.clone())),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..Default::default()
        })?)
    };
    let world = make_world()?;
    let formatted_world = make_world()?;
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
                if path.extension() == Some("typ".as_ref()) {
                    let content = String::from_utf8(content)?;
                    let doc = pretty_print(&content, 80);
                    pretty_print(&doc, 80).into_bytes().into()
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
    let mut driver = CompileDriver::new(world).with_entry_file(entry_file.clone());
    let mut formatted_driver = CompileDriver::new(formatted_world).with_entry_file(entry_file);
    let doc = driver.compile(&mut Default::default());
    let formatted_doc = formatted_driver.compile(&mut Default::default());
    compare_docs(doc, &driver.world, formatted_doc, &formatted_driver.world)?;
    Ok(())
}

fn compare_docs(
    doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
    world: &dyn TypstWorld,
    formatted_doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
    formatted_world: &dyn TypstWorld,
) -> anyhow::Result<()> {
    match (doc, formatted_doc) {
        (Ok(doc), Ok(formatted_doc)) => {
            pretty_assertions::assert_eq!(
                doc.pages.len(),
                formatted_doc.pages.len(),
                "The page counts are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.title,
                formatted_doc.title,
                "The titles are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.author,
                formatted_doc.author,
                "The authors are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.keywords,
                formatted_doc.keywords,
                "The keywords are not consistent"
            );

            for (i, (doc, formatted_doc)) in
                doc.pages.iter().zip(formatted_doc.pages.iter()).enumerate()
            {
                let svg = typst_svg::svg(&doc.frame);
                let formatted_svg = typst_svg::svg(&formatted_doc.frame);
                pretty_assertions::assert_eq!(
                    svg,
                    formatted_svg,
                    "The output are not consistent for page {}",
                    i
                );
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
