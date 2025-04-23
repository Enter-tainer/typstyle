use std::env;

use anyhow::{anyhow, Result};
use typst::{
    foundations::Smart,
    layout::{Page, PagedDocument},
};

use crate::universe::Compiled;

pub struct DiffSink(Vec<String>);

impl DiffSink {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(&mut self, err: impl Into<String>) {
        self.0.push(err.into());
    }

    pub fn check(&mut self, condition: bool, message: impl FnOnce() -> String) {
        if !condition {
            self.push(message());
        }
    }

    pub fn is_ok(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for DiffSink {
    fn default() -> Self {
        Self::new()
    }
}

impl From<DiffSink> for Result<()> {
    fn from(value: DiffSink) -> Self {
        if value.is_ok() {
            Ok(())
        } else {
            Err(anyhow!("{value}"))
        }
    }
}

impl From<&DiffSink> for Result<()> {
    fn from(value: &DiffSink) -> Self {
        if value.0.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("{value}"))
        }
    }
}

impl std::fmt::Display for DiffSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} errors occurred:", self.0.len())?;
        for (i, e) in self.0.iter().enumerate() {
            let err_str = e.replace('\n', "\n    ");
            writeln!(f, "{i:4}: {err_str}")?;
        }
        Ok(())
    }
}

pub struct CompiledPair(Compiled, Compiled);

impl CompiledPair {
    pub fn new(before: Compiled, after: Compiled) -> Self {
        Self(before, after)
    }

    pub fn compare(&self, require_compile: bool, sink: &mut DiffSink) -> Result<()> {
        match (&self.0.result, &self.1.result) {
            (Ok(doc_bf), Ok(doc_af)) => {
                check_doc_meta(doc_bf, doc_af, sink);
                check_png(doc_bf, doc_af, &self.0.name, &self.1.name, sink)?;
            }
            (Err(e1), Err(e2)) => {
                if require_compile {
                    sink.push(format!("Both docs failed to compile: \n{:#?}", e1));
                    return Ok(());
                }

                sink.check(e1.len() == e2.len(), || {
                    "The error counts are not consistent".to_string()
                });
                for (e1, e2) in e1.iter().zip(e2.iter()) {
                    sink.check(e1.message == e2.message, || {
                        "The error messages are not consistent after formatting".to_string()
                    });
                }
            }
            (Err(res1), _) => {
                sink.push(format!("Original doc failed to compile: {:#?}", res1));
            }
            (_, Err(res2)) => {
                sink.push(format!("Formatted doc failed to compile: {:#?}", res2));
            }
        }
        Ok(())
    }
}

fn check_doc_meta(left: &PagedDocument, right: &PagedDocument, sink: &mut DiffSink) {
    sink.check(left.pages.len() == right.pages.len(), || {
        "The page counts are not consistent.".to_string()
    });
    sink.check(left.info.title == right.info.title, || {
        "The titles are not consistent.".to_string()
    });
    sink.check(left.info.author == right.info.author, || {
        "The authors are not consistent.".to_string()
    });
    sink.check(left.info.keywords == right.info.keywords, || {
        "The keywords are not consistent.".to_string()
    });
}

fn check_png(
    before: &PagedDocument,
    after: &PagedDocument,
    before_name: &str,
    after_name: &str,
    sink: &mut DiffSink,
) -> anyhow::Result<()> {
    let render_png = |page: &Page, number: usize| {
        typst_render::render(
            &Page {
                frame: page.frame.clone(),
                fill: Smart::Auto,
                numbering: None,
                supplement: Default::default(),
                number,
            },
            2.0,
        )
    };

    for (i, (page_bf, page_af)) in before.pages.iter().zip(after.pages.iter()).enumerate() {
        let png_bf = render_png(page_bf, i);
        let png_af = render_png(page_af, i);
        if png_bf == png_af {
            continue;
        }
        // save both to tmp path and report error
        let tmp_dir = env::temp_dir();
        let png_path_bf = tmp_dir.join(format!("{before_name}-{}.png", i));
        let png_path_af = tmp_dir.join(format!("{after_name}-{}.png", i));
        png_bf.save_png(&png_path_bf).unwrap();
        png_af.save_png(&png_path_af).unwrap();
        sink.push(format!(
            "The output are not consistent for page {}, original png path: \"{}\", formatted png path: \"{}\"",
            i, png_path_bf.display(), png_path_af.display()
        ));
    }

    Ok(())
}
