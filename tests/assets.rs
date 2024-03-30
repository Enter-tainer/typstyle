use libtest_mimic::{Arguments, Failed, Trial};
use std::{
    borrow::Cow,
    env,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use typst_ts_compiler::{
    service::{CompileDriver, Compiler},
    ShadowApi, TypstSystemWorld,
};
use typst_ts_core::{
    config::{compiler::EntryOpts, CompileOpts},
    diag::SourceDiagnostic,
    typst::prelude::EcoVec,
    TypstDocument,
};
use typstyle_core::pretty_print;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::from_args();
    let tests = collect_tests()?;
    libtest_mimic::run(&args, tests).exit();
}

/// Creates one test for each `.typ` file in the current directory or
/// sub-directories of the current directory.
fn collect_tests() -> Result<Vec<Trial>, Box<dyn Error>> {
    fn visit_dir(path: &Path, tests: &mut Vec<Trial>) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            // Handle files
            let path = entry.path();
            if file_type.is_file() {
                if path.extension() == Some(OsStr::new("typ")) {
                    let name = path
                        .strip_prefix(env::current_dir()?)?
                        .display()
                        .to_string();

                    let test_40 = {
                        let path: std::path::PathBuf = path.clone();
                        Trial::test(format!("{} - 40char", name), move || check_file(&path, 40))
                            .with_kind("typst")
                    };
                    let test_80 = {
                        let path = path.clone();
                        Trial::test(format!("{} - 80char", name), move || check_file(&path, 80))
                            .with_kind("typst")
                    };
                    let test_120 = {
                        let path = path.clone();
                        Trial::test(format!("{} - 120char", name), move || {
                            check_file(&path, 120)
                        })
                        .with_kind("typst")
                    };
                    let test_convergence = {
                        let path = path.clone();
                        Trial::test(format!("{} - convergence", name), move || {
                            check_convergence(&path, 80)
                        })
                    };
                    let test_output_consistency =
                        Trial::test(format!("{} - output consistency", name), move || {
                            check_output_consistency(&path, 80)
                        });
                    tests.extend([
                        test_40,
                        test_80,
                        test_120,
                        test_convergence,
                        test_output_consistency,
                    ]);
                }
            } else if file_type.is_dir() {
                // Handle directories
                visit_dir(&path, tests)?;
            }
        }

        Ok(())
    }

    // We recursively look for `.typ` files, starting from the current
    // directory.
    let mut tests = Vec::new();
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets");
    visit_dir(&current_dir, &mut tests)?;

    Ok(tests)
}

fn remove_crlf(content: String) -> String {
    if cfg!(windows) {
        content.replace("\r\n", "\n")
    } else {
        content
    }
}

/// Performs a couple of tidy tests.
fn check_file(path: &Path, width: usize) -> Result<(), Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;
    let content = remove_crlf(content);
    let rel_path = pathdiff::diff_paths(
        path,
        env::current_dir().unwrap().join("tests").join("assets"),
    )
    .unwrap();
    let doc_string = pretty_print(&content, width);
    let replaced_path = rel_path
        .to_str()
        .unwrap()
        .replace(std::path::MAIN_SEPARATOR, "-");
    insta::with_settings!({
        snapshot_suffix => format!("{}-{width}", replaced_path),
        input_file => path,
    }, {
        insta::assert_snapshot!(doc_string);
    });
    Ok(())
}

fn check_convergence(path: &Path, width: usize) -> Result<(), Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;
    let content = remove_crlf(content);
    let first_pass = pretty_print(&content, width);
    let second_pass = pretty_print(&first_pass, width);
    pretty_assertions::assert_str_eq!(
        first_pass,
        second_pass,
        "first pass and second pass are not the same!"
    );
    Ok(())
}

fn compile_typst_src(content: &str) -> Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>> {
    let root = if cfg!(windows) {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    };
    let world = TypstSystemWorld::new(CompileOpts {
        entry: EntryOpts::new_rooted(root.clone(), Some(PathBuf::from("/main.typ"))),
        with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
        ..Default::default()
    })
    .unwrap();
    world
        .map_shadow(&root.join("main.typ"), content.as_bytes().into())
        .unwrap();
    let mut driver = CompileDriver::new(world).with_entry_file(root.join("main.typ"));
    driver.compile(&mut Default::default())
}

fn check_output_consistency(path: &Path, width: usize) -> Result<(), Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;
    let content = remove_crlf(content);
    let formatted_src = pretty_print(&content, width);
    let doc = compile_typst_src(&content);
    let formatted_doc = compile_typst_src(&formatted_src);
    compare_docs(doc, formatted_doc)?;
    Ok(())
}

fn compare_docs(
    doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
    formatted_doc: Result<Arc<TypstDocument>, EcoVec<SourceDiagnostic>>,
) -> Result<(), Failed> {
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
        (res1, res2) => {
            return Err(Failed::from(format!(
                "One of the documents failed to compile: {:#?} {:#?}",
                res1, res2
            )));
        }
    }
    Ok(())
}
