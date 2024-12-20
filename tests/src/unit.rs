use std::{env, error::Error, fs, path::Path};

use insta::internals::Content;
use libtest_mimic::{Failed, Trial};
use typst_syntax::Source;
use typstyle_core::{PrinterConfig, Typstyle};

/// Creates one test for each `.typ` file in the current directory or
/// sub-directories of the current directory.
pub fn collect_tests() -> Result<Vec<Trial>, Box<dyn Error>> {
    fn make_snapshot_test(path: &Path, name: &str, width: usize) -> Trial {
        let path = path.to_path_buf();
        Trial::test(format!("{name} - {width}char"), move || {
            check_snapshot(&path, width)
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
            } else if !(file_type.is_file() && path.extension() == Some("typ".as_ref())) {
                continue;
            }
            // Handle .typ files
            let name = path
                .strip_prefix(env::current_dir()?)?
                .display()
                .to_string();

            tests.extend([
                make_snapshot_test(&path, &name, 0),
                make_snapshot_test(&path, &name, 40),
                make_snapshot_test(&path, &name, 80),
                make_snapshot_test(&path, &name, 120),
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

        Ok(())
    }

    // We recursively look for `.typ` files, starting from the current directory.
    let mut tests = Vec::new();
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    visit_dir(&current_dir, &mut tests)?;

    Ok(tests)
}

fn check_snapshot(path: &Path, width: usize) -> Result<(), Failed> {
    let source = read_source(path)?;

    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.set_omit_expression(true);
    settings.set_snapshot_path(path.parent().unwrap().join("snap"));
    settings.set_input_file(path);
    if source.root().erroneous() {
        settings.set_raw_info(&Content::Map(vec![("erroneous".into(), true.into())]));
    }
    settings.bind(|| {
        let snap_name = format!("{}-{width}", path.file_name().unwrap().to_str().unwrap());
        if source.root().erroneous() {
            insta::assert_snapshot!(snap_name, "");
        } else {
            let cfg = PrinterConfig::new_with_width(width);
            let formatted = Typstyle::new_with_src(source, cfg).pretty_print().unwrap();

            insta::assert_snapshot!(snap_name, formatted);
        }
    });
    Ok(())
}

fn check_convergence(path: &Path, width: usize) -> Result<(), Failed> {
    let source = read_source(path)?;
    if source.root().erroneous() {
        return Ok(());
    }

    let cfg = PrinterConfig::new_with_width(width);
    let first_pass = Typstyle::new_with_src(source, cfg.clone()).pretty_print()?;
    let second_pass = Typstyle::new_with_content(first_pass.clone(), cfg).pretty_print()?;
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

    let source = read_source(path)?;
    if source.root().erroneous() {
        return Ok(());
    }

    let cfg = PrinterConfig::new_with_width(width);
    let formatted_src = Typstyle::new_with_src(source.clone(), cfg).pretty_print()?;

    compare_docs(
        "",
        make_universe(source.text())?,
        make_universe(&formatted_src)?,
        false,
    )?;

    Ok(())
}

fn read_source(path: &Path) -> Result<Source, Failed> {
    read_content(path).map(Source::detached)
}

fn read_content(path: &Path) -> Result<String, Failed> {
    let content = fs::read(path).map_err(|e| format!("Cannot read file: {e}"))?;

    // Check that the file is valid UTF-8
    let content = String::from_utf8(content)
        .map_err(|_| "The file's contents are not a valid UTF-8 string!")?;
    let content = remove_crlf(content);

    Ok(content)
}

fn remove_crlf(content: String) -> String {
    if cfg!(windows) {
        content.replace("\r\n", "\n")
    } else {
        content
    }
}
