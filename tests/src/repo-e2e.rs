use std::{collections::HashSet, fs, path::Path};

use anyhow::Context;
use libtest_mimic::{Failed, Trial};
use serde::Deserialize;
use typst_syntax::Source;
use typstyle_consistency::{cmp::compare_docs, universe::make_universe_formatted};
use typstyle_core::Typstyle;

use crate::common::{fixtures_dir, test_dir};

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
struct Testcase {
    name: String,
    git: String,
    #[serde(default)]
    rev: Option<String>,
    // these files are included as-is and should not be formatted
    #[serde(default)]
    entrypoint: Option<String>,
    #[serde(default)]
    examples: Option<String>,
    #[serde(default)]
    blacklist: HashSet<String>,
}

pub(super) fn collect_tests() -> Vec<Trial> {
    #[derive(Deserialize)]
    struct Config {
        testcase: Vec<Testcase>,
    }
    let config = toml::from_str::<Config>(
        &fs::read_to_string(fixtures_dir().join("e2e-repos.toml")).unwrap(),
    )
    .unwrap();
    config
        .testcase
        .into_iter()
        .map(|case| {
            Trial::test(format!("{} - e2e", case.name.clone()), move || {
                run_testcase(case.clone()).map_err(|e| Failed::from(e.to_string()))
            })
        })
        .collect()
}

fn run_testcase(testcase: Testcase) -> anyhow::Result<()> {
    let e2e_dir = test_dir().join("e2e");
    // do mkdir -p project_root/tests/e2e
    let _ = fs::create_dir_all(&e2e_dir);
    let testcase_dir = e2e_dir.join(&*testcase.name);

    clone_testcase_repo(&testcase, &testcase_dir)?;
    check_testcase(&testcase, &testcase_dir)?;

    let _ = fs::remove_dir_all(testcase_dir);
    Ok(())
}

fn clone_testcase_repo(testcase: &Testcase, testcase_dir: &Path) -> anyhow::Result<()> {
    // clean the testcase_dir
    let _ = fs::remove_dir_all(testcase_dir);
    // do git clone with submodule
    std::process::Command::new("git")
        .arg("clone")
        .arg(&testcase.git)
        .arg(testcase_dir)
        .arg("--recurse-submodules")
        .output()
        .context("failed to clone repo")?;
    if let Some(rev) = testcase.rev.as_ref() {
        // do git checkout testcase.revision
        std::process::Command::new("git")
            .arg("checkout")
            .arg(rev)
            .current_dir(testcase_dir)
            .output()
            .context("failed to checkout revision")?;
    }
    Ok(())
}

fn check_testcase(testcase: &Testcase, testcase_dir: &Path) -> anyhow::Result<()> {
    let Some(entrypoint) = testcase.entrypoint.as_ref() else {
        return Ok(());
    };

    let (doc, formatted_doc) = make_universe_formatted(
        testcase_dir,
        &testcase_dir.join(entrypoint),
        &testcase.blacklist,
        |content, rel_path| {
            let source = Source::detached(content);
            if source.root().erroneous() {
                panic!(
                    "The file {} has syntax errors: {:?}",
                    rel_path.display(),
                    source.root().errors()
                );
            }
            let doc = Typstyle::default().format_source(&source).unwrap();
            let second_format = Typstyle::default().format_content(&doc).unwrap();
            pretty_assertions::assert_eq!(
                doc,
                second_format,
                "The file {} is not converging after formatting",
                rel_path.display()
            );
            doc
        },
    )?;

    compare_docs(&testcase.name, doc, formatted_doc, true, true)
}
