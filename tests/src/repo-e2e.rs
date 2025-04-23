use std::{collections::HashSet, fs, path::Path};

use anyhow::{anyhow, bail, Context};
use libtest_mimic::{Failed, Trial};
use serde::Deserialize;
use typst_syntax::Source;
use typstyle_consistency::TypstyleUniverse;
use typstyle_core::{Config, Typstyle};

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
    if fs::exists(testcase_dir)? {
        return Ok(()); // for quick debugging
    }

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
    if testcase.entrypoint.is_none() && testcase.examples.is_none() {
        return Err(anyhow!(
            "The testcase `{}` does not have entrypoint or examples",
            testcase.name
        ));
    }

    let name = testcase.name.clone();
    let mut univ = TypstyleUniverse::new(name, testcase_dir.to_path_buf(), |content| {
        let source = Source::detached(content);
        if source.root().erroneous() {
            bail!("the file has syntax errors: {:?}", source.root().errors());
        }
        let config = Config {
            reorder_import_items: true,
            ..Default::default()
        };
        let first_pass = Typstyle::new(config.clone())
            .format_source(&source)
            .unwrap();
        let second_pass = Typstyle::new(config).format_content(&first_pass).unwrap();
        if first_pass != second_pass {
            bail!("the formatting does not converge")
        }
        Ok(first_pass)
    })
    .with_context(|| format!("failed to create universe: {}", testcase.name))?;
    univ.add_all_files(testcase_dir, &testcase.blacklist)
        .with_context(|| format!("failed to add all files in {}", testcase_dir.display()))?;

    if let Some(entrypoint) = testcase.entrypoint.as_ref() {
        let compiled = univ.compile_with_entry(Path::new(&entrypoint));
        compiled
            .compare(true, univ.sink_mut())
            .with_context(|| format!("failed to compare outputs with entry: {entrypoint}"))?;
    }
    if let Some(examples) = testcase.examples.as_ref() {
        let entry_vpath = Path::new("__examples__.typ");
        univ.add_all_files_in_one(entry_vpath, &testcase_dir.join(examples))?;
        let compiled = univ.compile_with_entry(entry_vpath);
        compiled
            .compare(true, univ.sink_mut())
            .with_context(|| format!("failed to compare outputs with examples: {examples}"))?;
    };

    univ.sink().into()
}
