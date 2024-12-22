use std::{
    fs,
    path::{Path, PathBuf},
};

use libtest_mimic::Failed;
use typst_syntax::Source;

pub fn test_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn fixtures_dir() -> PathBuf {
    test_dir().join("fixtures")
}

pub fn read_source(path: &Path) -> Result<Source, Failed> {
    read_content(path).map(Source::detached)
}

pub fn read_content(path: &Path) -> Result<String, Failed> {
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
