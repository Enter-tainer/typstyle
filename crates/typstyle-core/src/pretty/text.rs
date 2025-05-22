use typst_syntax::{ast::*, SyntaxNode};

use super::{prelude::*, PrettyPrinter};
use crate::ext::StrExt;

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_text(&'a self, text: Text<'a>) -> ArenaDoc<'a> {
        // `Text` only consists of words joined by single spaces
        self.convert_verbatim(text)
    }

    pub(super) fn convert_text_wrapped(&'a self, text: Text<'a>) -> ArenaDoc<'a> {
        wrap_text(&self.arena, text.get())
    }

    pub(super) fn convert_space(&'a self, space: Space) -> ArenaDoc<'a> {
        self.convert_space_untyped(space.to_untyped())
    }

    pub(super) fn convert_space_untyped(&'a self, node: &SyntaxNode) -> ArenaDoc<'a> {
        if node.text().has_linebreak() {
            self.arena.hardline()
        } else {
            self.arena.space()
        }
    }

    pub(super) fn convert_parbreak(&'a self, parbreak: Parbreak) -> ArenaDoc<'a> {
        let newline_count = parbreak.to_untyped().text().count_linebreaks();
        self.arena.hardline().repeat_n(newline_count)
    }
}

fn wrap_text<'a>(arena: &'a Arena<'a>, text: &'a str) -> ArenaDoc<'a> {
    arena.intersperse(text.split_ascii_whitespace(), arena.softline())
        + if text.ends_with(' ') {
            // special case when a link follows the text
            arena.softline()
        } else {
            arena.nil()
        }
}
