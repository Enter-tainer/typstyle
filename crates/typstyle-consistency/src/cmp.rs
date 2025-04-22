use std::env;

use anyhow::{bail, Context};
use tinymist_world::TypstSystemUniverse;
use typst::{
    foundations::Smart,
    layout::{Page, PagedDocument},
};
use typst_pdf::{PdfOptions, PdfStandards};

pub fn compare_docs(
    name: &str,
    before: TypstSystemUniverse,
    after: TypstSystemUniverse,
    require_compile: bool,
    output_pdf: bool,
) -> anyhow::Result<()> {
    let compile_universe =
        |universe: TypstSystemUniverse| typst::compile(&universe.snapshot()).output;

    let doc_bf = compile_universe(before);
    let doc_af = compile_universe(after);

    match (doc_bf, doc_af) {
        (Ok(doc_bf), Ok(doc_af)) => {
            let message = if output_pdf {
                check_pdf(&doc_bf, &doc_af, name)?
            } else {
                String::new()
            };

            check_doc_meta(&doc_bf, &doc_af, &message);
            check_png(&doc_bf, &doc_af, name)?;
        }
        (Err(e1), Err(e2)) => {
            if require_compile {
                bail!("Both docs failed to compile: {:#?}", e1);
            }
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
        (Err(res1), _) => {
            bail!("Original doc failed to compile: {:#?}", res1);
        }
        (_, Err(res2)) => {
            bail!("Formatted doc failed to compile: {:#?}", res2);
        }
    }
    Ok(())
}

fn check_doc_meta(left: &PagedDocument, right: &PagedDocument, message: &str) {
    pretty_assertions::assert_eq!(
        left.pages.len(),
        right.pages.len(),
        "The page counts are not consistent. {message}"
    );
    pretty_assertions::assert_eq!(
        left.info.title,
        right.info.title,
        "The titles are not consistent. {message}"
    );
    pretty_assertions::assert_eq!(
        left.info.author,
        right.info.author,
        "The authors are not consistent. {message}"
    );
    pretty_assertions::assert_eq!(
        left.info.keywords,
        right.info.keywords,
        "The keywords are not consistent. {message}"
    );
}

fn check_pdf(before: &PagedDocument, after: &PagedDocument, name: &str) -> anyhow::Result<String> {
    let render_pdf = |doc: &PagedDocument, ident: &'static str| {
        typst_pdf::pdf(
            doc,
            &PdfOptions {
                ident: Smart::Custom(ident),
                timestamp: None,
                page_ranges: None,
                standards: PdfStandards::default(),
            },
        )
    };

    let pdf_bf = render_pdf(before, "original");
    let pdf_af = render_pdf(after, "formatted");
    // write both pdf to tmp path
    let tmp_dir = env::temp_dir();
    let pdf_path_bf = tmp_dir.join(format!("{name}-{}.pdf", "original"));
    let pdf_path_af = tmp_dir.join(format!("{name}-{}.pdf", "formatted"));
    std::fs::write(&pdf_path_bf, pdf_bf.unwrap()).context("failed to write pdf")?;
    std::fs::write(&pdf_path_af, pdf_af.unwrap()).context("failed to write formatted pdf")?;
    let message = format!(
        "The pdfs are written to \"{}\" and \"{}\"",
        pdf_path_bf.display(),
        pdf_path_af.display()
    );

    Ok(message)
}

fn check_png(before: &PagedDocument, after: &PagedDocument, name: &str) -> anyhow::Result<()> {
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
        if png_bf != png_af {
            // save both to tmp path and report error
            let tmp_dir = env::temp_dir();
            let png_path_bf = tmp_dir.join(format!("{name}-{}-{}.png", i, "original"));
            let png_path_af = tmp_dir.join(format!("{name}-{}-{}.png", i, "formatted"));
            png_bf.save_png(&png_path_bf).unwrap();
            png_af.save_png(&png_path_af).unwrap();
            bail!(
                "The output are not consistent for page {}, original png path: \"{}\", formatted png path: \"{}\"",
                i, png_path_bf.display(), png_path_af.display()
            );
        }
    }

    Ok(())
}
