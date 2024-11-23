use pretty::{Arena, DocAllocator};
use typst_syntax::{ast::*, SyntaxKind};

use super::{util::is_comment_node, ArenaDoc, PrettyPrinter};

// Handles arg-like expressions, where the comments can only appear in the middle.
pub struct ArgStylist<'a> {
    arena: &'a Arena<'a>,
    printer: &'a PrettyPrinter<'a>,
    doc: ArenaDoc<'a>,
    leading: bool,
}

impl<'a> ArgStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            arena: &printer.arena,
            doc: printer.arena.nil(),
            printer,
            leading: true,
        }
    }

    pub fn convert_named(mut self, named: Named<'a>) -> ArenaDoc<'a> {
        // We put a space only before expr and comment when not lead
        for node in named.to_untyped().children() {
            if let Some(expr) = node.cast() {
                self.push_leading_space();
                self.push_doc(self.printer.convert_expr(expr));
            } else if let Some(pattern) = node.cast() {
                self.push_leading_space();
                self.push_doc(self.printer.convert_pattern(pattern));
            } else if node.kind() == SyntaxKind::Colon {
                self.push_doc(self.arena.text(":"));
            } else if node.kind() == SyntaxKind::Hash {
                self.push_leading_space();
                self.push_doc(self.arena.text("#"));
                self.leading = true;
                continue;
            } else if is_comment_node(node) {
                self.push_leading_space();
                self.push_doc(self.printer.convert_comment(node));
                if node.kind() == SyntaxKind::LineComment {
                    self.push_doc(self.arena.hardline());
                    self.leading = true;
                    continue;
                }
            } else {
                // We ignore line break here.
                continue;
            }
            self.leading = false;
        }

        self.doc.group()
    }

    pub fn convert_keyed(mut self, keyed: Keyed<'a>) -> ArenaDoc<'a> {
        for node in keyed.to_untyped().children() {
            if let Some(expr) = node.cast() {
                self.push_leading_space();
                self.push_doc(self.printer.convert_expr(expr));
            } else if node.kind() == SyntaxKind::Colon {
                self.push_doc(self.arena.text(":"));
            } else if is_comment_node(node) {
                self.push_leading_space();
                self.push_doc(self.printer.convert_comment(node));
                if node.kind() == SyntaxKind::LineComment {
                    self.push_doc(self.arena.hardline());
                    self.leading = true;
                    continue;
                }
            } else {
                // We ignore line break here.
                continue;
            }
            self.leading = false;
        }

        self.doc.group()
    }

    pub fn convert_spread(mut self, spread: Spread<'a>) -> ArenaDoc<'a> {
        for node in spread.to_untyped().children() {
            if let Some(expr) = node.cast() {
                self.push_doc(self.printer.convert_expr(expr));
            } else if node.kind() == SyntaxKind::Dots {
                self.push_doc(self.arena.text(".."));
            } else if is_comment_node(node) {
                self.push_doc(self.printer.convert_comment(node));
                if node.kind() == SyntaxKind::LineComment {
                    self.push_leading_space();
                    self.push_doc(self.arena.hardline());
                    self.leading = true;
                    continue;
                }
            } else {
                // We ignore line break here.
                continue;
            }
            self.leading = false;
        }

        self.doc.group()
    }

    fn push_doc(&mut self, doc: ArenaDoc<'a>) {
        self.doc += doc;
    }

    /// Push a space if not leading.
    fn push_leading_space(&mut self) {
        if !self.leading {
            self.doc += self.arena.space();
        }
    }
}
