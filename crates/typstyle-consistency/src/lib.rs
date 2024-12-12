pub mod cmp;
pub mod compile;

use ecow::EcoVec;
use std::sync::Arc;
use typst::{diag::SourceDiagnostic, model::Document};

pub type CompilationResult = Result<Arc<Document>, EcoVec<SourceDiagnostic>>;
