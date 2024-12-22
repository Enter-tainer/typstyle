use std::{borrow::Cow, collections::HashSet, fs, path::Path};

use anyhow::Context;
use libtest_mimic::{Failed, Trial};
use typst_syntax::Source;
use typstyle_consistency::{cmp::compare_docs, universe::make_universe_formatted};
use typstyle_core::Typstyle;

use crate::common::test_dir;

#[derive(Debug, Clone)]
struct Testcase {
    name: Cow<'static, str>,
    repo_url: Cow<'static, str>,
    revision: Cow<'static, str>,
    entrypoint: Cow<'static, str>,
    // these files are included as-is and should not be formatted
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

    fn with_blacklist(mut self, blacklist: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.blacklist = blacklist.into_iter().map(Into::into).collect();
        self
    }
}

pub(super) fn collect_tests() -> Vec<Trial> {
    let cases = [
        Testcase::new(
            "tutorial",
            "https://github.com/typst-doc-cn/tutorial",
            "3e1dcc83ca7abb314f84e985e84c15299790aaa8",
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
            "8ec63845f87941d8d6b90b5d0dd52ed4e74c3694",
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
        .with_blacklist(["chicv.typ"]),
        Testcase::new(
            "touying-example",
            "https://github.com/touying-typ/touying",
            "e2e20b7243733e14cfb303f3d22988e862655041",
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
            "a011539846850ad466f16fde715ab6f83a6512f4",
            "docs/manual.typ",
        )
        // tidy has weird behavior when parsing typ source code
        .with_blacklist(["main.typ", "marks.typ"]),
        Testcase::new(
            "nju-thesis-typst",
            "https://github.com/nju-lug/nju-thesis-typst",
            "8b481bddd3bfa683a5af2e22922e222d8d5d0f81",
            "thesis.typ",
        ),
        Testcase::new(
            "physica",
            "https://github.com/Leedehai/typst-physics",
            "443c963013b5cfea64818fa71990fefde9c93131",
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
        .with_blacklist([
            "shor-nine-qubit-code.typ",
            "teleportation.typ",
            "phase-estimation.typ",
            "qft.typ",
            "fault-tolerant-measurement.typ",
            "fault-tolerant-pi8.typ",
            "fault-tolerant-toffoli1.typ",
            "fault-tolerant-toffoli2.typ",
            "quill-guide.typ",
        ]),
        Testcase::new(
            "curryst",
            "https://github.com/pauladam94/curryst",
            "1ffd5f41a22cf3a2ea1d48f65c959c1883fe08b3",
            "examples/natural-deduction.typ",
        ),
        Testcase::new(
            "derive-it",
            "https://github.com/0rphee/derive-it",
            "e56a25def12082b8c9c6d54026c426434a5610e8",
            "examples/example.typ",
        ),
    ];
    cases
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
        .arg(&*testcase.repo_url)
        .arg(testcase_dir)
        .arg("--recurse-submodules")
        .output()
        .context("failed to clone repo")?;
    // do git checkout testcase.revision
    std::process::Command::new("git")
        .arg("checkout")
        .arg(&*testcase.revision)
        .current_dir(testcase_dir)
        .output()
        .context("failed to checkout revision")?;
    Ok(())
}

fn check_testcase(testcase: &Testcase, testcase_dir: &Path) -> anyhow::Result<()> {
    let entrypoint = testcase_dir.join(&*testcase.entrypoint);

    let (doc, formatted_doc) = make_universe_formatted(
        testcase_dir,
        &entrypoint,
        &testcase.blacklist,
        |content, rel_path| {
            let source = Source::detached(content);
            if source.root().erroneous() {
                return source.text().to_string();
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

    compare_docs(&testcase.name, doc, formatted_doc, true)
}
