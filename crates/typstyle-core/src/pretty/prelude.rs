pub use pretty::{Arena, DocAllocator, DocBuilder};

pub use super::doc_ext::*;

pub type ArenaDoc<'a> = DocBuilder<'a, Arena<'a>>;
