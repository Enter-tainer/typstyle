extern crate libtest_mimic;

use libtest_mimic::{Arguments, Failed, Trial};
use typst_geshihua::PrettyPrinter;

use std::{env, error::Error, ffi::OsStr, fmt::format, fs, path::Path};

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
                    let test_120 = Trial::test(format!("{} - 120char", name), move || {
                        check_file(&path, 120)
                    })
                    .with_kind("typst");
                    tests.extend([test_40, test_80, test_120]);
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
    let current_dir = env::current_dir()?;
    visit_dir(&current_dir, &mut tests)?;

    Ok(tests)
}

/// Performs a couple of tidy tests.
fn check_file(path: &Path, width: usize) -> Result<(), Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;

    let printer = PrettyPrinter::default();
    let root = typst_syntax::parse(&content);
    let markup = root.cast().unwrap();
    let doc = printer.convert_markup(markup);
    let doc_string = doc.pretty(width).to_string();
    let filename = path.file_name().unwrap().to_str().unwrap();
    insta::with_settings!({
        snapshot_suffix => format!("{}-{width}", filename),
        input_file => path,
    }, {
        insta::assert_snapshot!(doc_string);
    });
    Ok(())
}
