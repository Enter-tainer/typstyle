use std::{env, error::Error, ffi::OsStr, fs, path::Path};

use libtest_mimic::{Failed, Trial};
use typstyle_core::{PrinterConfig, Typstyle};

/// Creates one test for each `.typ` file in the current directory or
/// sub-directories of the current directory.
pub fn collect_tests() -> Result<Vec<Trial>, Box<dyn Error>> {
    fn make_test(path: &Path, name: &str, width: usize) -> Trial {
        let path = path.to_path_buf();
        Trial::test(format!("{name} - {width}char"), move || {
            check_file(&path, width)
        })
        .with_kind("typst")
    }

    fn make_convergence_test(path: &Path, name: &str, width: usize) -> Trial {
        let path = path.to_path_buf();
        Trial::test(format!("{name} - convergence - {width}char"), move || {
            check_convergence(&path, width)
        })
    }

    #[cfg(feature = "consistency")]
    fn make_consistency_test(path: &Path, name: &str, width: usize) -> Trial {
        let path = path.to_path_buf();
        Trial::test(
            format!("{name} - output consistency - {width}char"),
            move || check_output_consistency(&path, width),
        )
    }

    fn visit_dir(path: &Path, tests: &mut Vec<Trial>) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_dir() {
                // Handle directories
                visit_dir(&path, tests)?;
                continue;
            } else if !file_type.is_file() {
                continue;
            }
            // Handle files
            if path.extension() == Some(OsStr::new("typ")) {
                let name = path
                    .strip_prefix(env::current_dir()?)?
                    .display()
                    .to_string();

                tests.extend([
                    make_test(&path, &name, 0),
                    make_test(&path, &name, 40),
                    make_test(&path, &name, 80),
                    make_test(&path, &name, 120),
                    make_convergence_test(&path, &name, 0),
                    make_convergence_test(&path, &name, 40),
                    make_convergence_test(&path, &name, 80),
                ]);
                #[cfg(feature = "consistency")]
                tests.extend([
                    make_consistency_test(&path, &name, 0),
                    make_consistency_test(&path, &name, 40),
                    make_consistency_test(&path, &name, 80),
                ]);
            }
        }

        Ok(())
    }

    // We recursively look for `.typ` files, starting from the current
    // directory.
    let mut tests = Vec::new();
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
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
    let content = read_content(path)?;

    let cfg = PrinterConfig::new_with_width(width);
    let formatted = Typstyle::new_with_content(content, cfg).pretty_print();
    let snap_name = format!("{}-{width}", path.file_name().unwrap().to_str().unwrap());

    insta::with_settings!({
        snapshot_path => path.parent().unwrap().join("snap"),
        prepend_module_to_snapshot => false,
        input_file => path,
        omit_expression => true,
    }, {
        insta::assert_snapshot!(snap_name, formatted);
    });
    Ok(())
}

fn check_convergence(path: &Path, width: usize) -> Result<(), Failed> {
    let content = read_content(path)?;

    let cfg = PrinterConfig::new_with_width(width);
    let first_pass = Typstyle::new_with_content(content, cfg.clone()).pretty_print();
    let second_pass = Typstyle::new_with_content(first_pass.clone(), cfg).pretty_print();
    pretty_assertions::assert_str_eq!(
        first_pass,
        second_pass,
        "first pass and second pass are not the same!"
    );
    Ok(())
}

#[cfg(feature = "consistency")]
fn check_output_consistency(path: &Path, width: usize) -> Result<(), Failed> {
    use typstyle_consistency::{cmp::compare_docs, universe::make_universe};

    let content = read_content(path)?;

    let cfg = PrinterConfig::new_with_width(width);
    let formatted_src = Typstyle::new_with_content(content.clone(), cfg).pretty_print();

    compare_docs(
        "",
        make_universe(&content)?,
        make_universe(&formatted_src)?,
        false,
    )?;

    Ok(())
}

fn read_content(path: &Path) -> Result<String, Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;
    let content = remove_crlf(content);

    Ok(content)
}
