use std::{
    fs,
    path::{Path, PathBuf},
};

use libtest_mimic::Failed;
use typst_syntax::Source;
use typstyle_core::Config;

pub fn test_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn fixtures_dir() -> PathBuf {
    test_dir().join("fixtures")
}

pub fn read_source_with_config(path: &Path) -> Result<(Source, Config), Failed> {
    let content = read_content(path)?;
    let config = parse_directives(&content)?;
    Ok((Source::detached(content), config))
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

/// Parses typstyle directives from the first line of a file
fn parse_directives(content: &str) -> Result<Config, Failed> {
    let mut config = Config::new();

    // Get the first line
    if let Some(first_line) = content.lines().next() {
        // Check if it starts with the directive marker
        if first_line.trim_start().starts_with("/// typstyle:") {
            // Extract the content after the marker
            let directive_content = first_line
                .trim_start()
                .strip_prefix("/// typstyle:")
                .unwrap_or("")
                .trim();

            // Split by spaces to get individual directives
            for directive in directive_content.split_whitespace() {
                // Check if it's a key-value pair
                let (key, value) = directive
                    .split_once('=')
                    .map(|(key, value)| (key.trim(), Some(value.trim())))
                    .unwrap_or((directive, None));
                match key {
                    "reorder-import-items" => config.reorder_import_items = value != Some("false"),
                    _ => return Err(format!("unknown directive: {key}").into()),
                }
            }
        }
    }

    Ok(config)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_directive() {
        let content = "/// typstyle: reorder-import-items\n#import \"module.typ\": a, b";
        let config = parse_directives(content).unwrap();

        assert_eq!(config.reorder_import_items, true);
    }
}
