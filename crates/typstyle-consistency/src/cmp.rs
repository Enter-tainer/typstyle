use anyhow::Result;
use tinymist_world::SourceWorld;
use typst::{
    diag::SourceDiagnostic,
    ecow::EcoVec,
    foundations::Smart,
    layout::{Page, PagedDocument},
};

use crate::ErrorSink;

pub struct Compiled<'a> {
    pub name: String,
    pub world: &'a dyn SourceWorld,
    pub result: Result<PagedDocument, EcoVec<SourceDiagnostic>>,
}

pub fn compile_world(name: String, world: &dyn SourceWorld) -> Result<Compiled<'_>> {
    let result = typst::compile(world.as_world()).output;

    Ok(Compiled {
        name,
        world,
        result,
    })
}

pub fn compare_docs(
    before: &Compiled,
    after: &Compiled,
    require_compile: bool,
    sink: &mut ErrorSink,
) -> Result<()> {
    match (&before.result, &after.result) {
        (Ok(doc_bf), Ok(doc_af)) => {
            check_doc_meta(doc_bf, doc_af, sink);
            check_png(doc_bf, doc_af, sink)?;
        }
        (Err(e1), Err(e2)) => {
            if require_compile {
                sink.push("Both docs failed to compile.".to_string());
                print_diagnostics(before.world, e1.iter())?;
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
        (Err(e1), _) => {
            sink.push("Original doc failed to compile.".to_string());
            print_diagnostics(before.world, e1.iter())?;
        }
        (_, Err(e2)) => {
            sink.push("Formatted doc failed to compile.".to_string());
            print_diagnostics(after.world, e2.iter())?;
        }
    }
    Ok(())
}

fn check_doc_meta(left: &PagedDocument, right: &PagedDocument, sink: &mut ErrorSink) {
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
    sink: &mut ErrorSink,
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
        check_page(i, page_bf, page_af, sink);

        let png_bf = render_png(page_bf, i);
        let png_af = render_png(page_af, i);
        if png_bf == png_af {
            continue;
        }
        sink.push(format!("The output are not consistent for page {}.", i));
    }

    Ok(())
}

fn check_page(index: usize, before: &Page, after: &Page, sink: &mut ErrorSink) {
    sink.check(before.fill == after.fill, || {
        format!("The fills of page {index} are not consistent.")
    });
    sink.check(before.numbering == after.numbering, || {
        format!("The numberings of page {index} are not consistent.")
    });
    sink.check(before.supplement == after.supplement, || {
        format!("The supplements of page {index} are not consistent.")
    });
    sink.check(before.number == after.number, || {
        format!("The numbers of page {index} are not consistent.")
    });
}

fn print_diagnostics<'d, 'files>(
    world: &'files dyn SourceWorld,
    errors: impl Iterator<Item = &'d SourceDiagnostic>,
) -> Result<()> {
    Ok(tinymist_world::print_diagnostics(
        world,
        errors,
        tinymist_world::DiagnosticFormat::Human,
    )?)
}
