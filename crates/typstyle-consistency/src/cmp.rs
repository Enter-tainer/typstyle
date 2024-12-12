use anyhow::{bail, Context};
use itertools::Itertools;
use reflexo_typst::error::diag_from_std;
use std::env;
use typst::{
    foundations::Smart::{Auto, Custom},
    layout::Page,
    World,
};
use typst_pdf::{PdfOptions, PdfStandards};

use crate::CompilationResult;

pub fn compare_docs(
    doc: CompilationResult,
    formatted_doc: CompilationResult,
) -> anyhow::Result<()> {
    match (doc, formatted_doc) {
        (Ok(doc), Ok(formatted_doc)) => {
            pretty_assertions::assert_eq!(
                doc.pages.len(),
                formatted_doc.pages.len(),
                "The page counts are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.info.title,
                formatted_doc.info.title,
                "The titles are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.info.author,
                formatted_doc.info.author,
                "The authors are not consistent"
            );
            pretty_assertions::assert_eq!(
                doc.info.keywords,
                formatted_doc.info.keywords,
                "The keywords are not consistent"
            );

            for (i, (doc, formatted_doc)) in
                doc.pages.iter().zip(formatted_doc.pages.iter()).enumerate()
            {
                let png = typst_render::render(
                    &Page {
                        frame: doc.frame.clone(),
                        fill: typst::foundations::Smart::Auto,
                        numbering: None,
                        number: i,
                    },
                    2.0,
                );
                let formatted_png = typst_render::render(
                    &Page {
                        frame: formatted_doc.frame.clone(),
                        fill: typst::foundations::Smart::Auto,
                        numbering: None,
                        number: i,
                    },
                    2.0,
                );
                if png != formatted_png {
                    // save both to tmp path and report error
                    let tmp_dir = env::temp_dir();
                    let png_path = tmp_dir.join(format!("{}-{}.png", i, "original"));
                    let formatted_png_path = tmp_dir.join(format!("{}-{}.png", i, "formatted"));
                    png.save_png(&png_path).unwrap();
                    formatted_png.save_png(&formatted_png_path).unwrap();
                    bail!(
                        "The output are not consistent for page {}, original png path: {}, formatted png path: {}",
                        i, png_path.display(), formatted_png_path.display()
                    );
                }
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
            bail!("One of the documents failed to compile: {res1:#?} {res2:#?}");
        }
    }
    Ok(())
}

pub fn compare_docs_full(
    name: &str,
    doc: CompilationResult,
    world: &dyn World,
    formatted_doc: CompilationResult,
    formatted_world: &dyn World,
) -> anyhow::Result<()> {
    match (doc, formatted_doc) {
        (Ok(doc), Ok(formatted_doc)) => {
            let pdf = typst_pdf::pdf(
                &doc,
                &PdfOptions {
                    ident: Custom("original"),
                    timestamp: None,
                    page_ranges: None,
                    standards: PdfStandards::default(),
                },
            );
            let formatted_pdf = typst_pdf::pdf(
                &formatted_doc,
                &PdfOptions {
                    ident: Custom("formatted"),
                    timestamp: None,
                    page_ranges: None,
                    standards: PdfStandards::default(),
                },
            );
            // write both pdf to tmp path
            let tmp_dir = env::temp_dir();
            let pdf_path = tmp_dir.join(format!("{name}-{}.pdf", "original"));
            let formatted_pdf_path = tmp_dir.join(format!("{name}-{}.pdf", "formatted"));
            std::fs::write(&pdf_path, pdf.unwrap()).context("failed to write pdf")?;
            std::fs::write(&formatted_pdf_path, formatted_pdf.unwrap())
                .context("failed to write formatted pdf")?;
            let message = format!(
                "The pdfs are written to \"{}\" and \"{}\"",
                pdf_path.display(),
                formatted_pdf_path.display()
            );
            pretty_assertions::assert_eq!(
                doc.pages.len(),
                formatted_doc.pages.len(),
                "The page counts are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.title,
                formatted_doc.info.title,
                "The titles are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.author,
                formatted_doc.info.author,
                "The authors are not consistent. {message}"
            );
            pretty_assertions::assert_eq!(
                doc.info.keywords,
                formatted_doc.info.keywords,
                "The keywords are not consistent. {message}"
            );

            for (i, (doc, formatted_doc)) in
                doc.pages.iter().zip(formatted_doc.pages.iter()).enumerate()
            {
                let png = typst_render::render(
                    &Page {
                        frame: doc.frame.clone(),
                        fill: Auto,
                        numbering: None,
                        number: i,
                    },
                    2.0,
                );
                let formatted_png = typst_render::render(
                    &Page {
                        frame: formatted_doc.frame.clone(),
                        fill: Auto,
                        numbering: None,
                        number: i,
                    },
                    2.0,
                );
                if png != formatted_png {
                    // save both to tmp path and report error
                    let tmp_dir = env::temp_dir();
                    let png_path = tmp_dir.join(format!("{name}-{}-{}.png", i, "original"));
                    let formatted_png_path =
                        tmp_dir.join(format!("{name}-{}-{}.png", i, "formatted"));
                    png.save_png(&png_path).unwrap();
                    formatted_png.save_png(&formatted_png_path).unwrap();
                    bail!(
                        "The output are not consistent for page {}, original png path: \"{}\", formatted png path: \"{}\"",
                        i, png_path.display(), formatted_png_path.display()
                    );
                }
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
        (_, Err(res2)) => {
            let diag = res2
                .into_iter()
                .map(|e| diag_from_std(e, Some(formatted_world)))
                .collect_vec();
            bail!("Formatted doc failed to compile: {:#?}", diag);
        }
        (Err(res1), _) => {
            let diag = res1
                .into_iter()
                .map(|e| diag_from_std(e, Some(world)))
                .collect_vec();
            bail!("Original doc failed to compile: {:#?}", diag);
        }
    }
    Ok(())
}
