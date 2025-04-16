use pretty::DocAllocator;
use typst_syntax::ast::*;

use super::{doc_ext::DocExt, ArenaDoc, PrettyPrinter};
use crate::ext::StrExt;

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_text(&'a self, text: Text<'a>) -> ArenaDoc<'a> {
        // `Text` only consists of words joined by single spaces
        self.convert_trivia(text)
    }

    pub(super) fn convert_space(&'a self, space: Space<'a>) -> ArenaDoc<'a> {
        let node = space.to_untyped();
        if node.text().has_linebreak() {
            self.arena.hardline()
        } else {
            self.arena.space()
        }
    }

    pub(super) fn convert_parbreak(&'a self, parbreak: Parbreak<'a>) -> ArenaDoc<'a> {
        let newline_count = parbreak.to_untyped().text().count_linebreaks();
        self.arena.hardline().repeat_n(newline_count)
    }
}
