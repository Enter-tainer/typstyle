use pretty::{Arena, DocAllocator};
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{style::FoldStyle, util::is_comment_node, ArenaDoc, PrettyPrinter};

enum Item<'a> {
    Comment(ArenaDoc<'a>),
    Commented {
        body: ArenaDoc<'a>,
        after: ArenaDoc<'a>,
    },
}

pub struct ListStylist<'a> {
    arena: &'a Arena<'a>,
    printer: &'a PrettyPrinter<'a>,
    can_attach: bool,
    free_comments: Vec<ArenaDoc<'a>>,
    items: Vec<Item<'a>>,
    fold_style: FoldStyle,
}

struct ListStyle {
    /// The separator used in single-line style.
    single_line_sep: &'static str,
    /// The separator used in multi-line style.
    multi_line_sep: &'static str,
    /// The delimiter of the list.
    delim: (&'static str, &'static str),
    /// Whether to add an addition space inside the delimiters if the list is empty.
    add_space_if_empty: bool,
}

impl<'a> ListStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            arena: &printer.arena,
            printer,
            can_attach: false,
            free_comments: Default::default(),
            items: Default::default(),
            fold_style: FoldStyle::Fit,
        }
    }

    pub fn convert_array(mut self, array: Array<'a>) -> ArenaDoc<'a> {
        self.fold_style = self.printer.get_fold_style(array);
        self.process_list(array.to_untyped(), |node| {
            self.printer.convert_array_item(node)
        });

        self.pretty_commented_items(ListStyle {
            single_line_sep: ",",
            multi_line_sep: ",",
            delim: ("(", ")"),
            add_space_if_empty: false,
        })
    }

    fn process_list<T: AstNode<'a>>(
        &mut self,
        list_node: &'a SyntaxNode,
        item_converter: impl Fn(T) -> ArenaDoc<'a>,
    ) {
        // Each item can be attached with comments at the front and back.
        // Can break line after front attachments.
        // If the back attachment appears before the comma, the comma is move to its front if multiline.

        self.fold_style = self.printer.get_fold_style_untyped(list_node);

        for node in list_node.children() {
            if let Some(item) = node.cast() {
                let before = if self.free_comments.is_empty() {
                    self.arena.nil()
                } else {
                    self.arena
                        .intersperse(self.free_comments.drain(..), self.arena.line())
                        + self.arena.line()
                };
                self.items.push(Item::Commented {
                    body: (before + item_converter(item)).group(),
                    after: self.arena.nil(),
                });
                self.can_attach = true;
            } else if is_comment_node(node) {
                // Line comment cannot appear in single line block
                if node.kind() == SyntaxKind::LineComment {
                    self.fold_style = FoldStyle::Never;
                }
                self.free_comments.push(self.printer.convert_comment(node));
            } else if node.kind() == SyntaxKind::Comma {
                self.try_attach_comments();
            } else if node.kind() == SyntaxKind::Space {
                let newline_cnt = node.text().chars().filter(|c| *c == '\n').count();
                if newline_cnt > 0 {
                    self.attach_or_detach_comments();
                    self.can_attach = false;
                }
            }
        }

        self.attach_or_detach_comments();
    }

    /// Try attaching free comments. If it fails, detach them.
    fn attach_or_detach_comments(&mut self) {
        if !self.try_attach_comments() {
            self.detach_comments();
        }
    }

    /// Attack free comments to the last item if possible.
    fn try_attach_comments(&mut self) -> bool {
        if self.can_attach && !self.free_comments.is_empty() {
            if let Some(Item::Commented { after: cmt, .. }) = self.items.last_mut() {
                *cmt += self.arena.space()
                    + self
                        .arena
                        .intersperse(self.free_comments.drain(..), self.arena.space());
                return true;
            }
        }
        false
    }

    /// Make all free comments detached.
    fn detach_comments(&mut self) {
        self.items
            .extend(self.free_comments.drain(..).map(Item::Comment));
    }

    fn pretty_commented_items(self, sty: ListStyle) -> ArenaDoc<'a> {
        let delim = sty.delim;
        if self.items.is_empty() {
            return if sty.add_space_if_empty {
                self.arena.text(delim.0) + self.arena.space() + delim.1
            } else {
                self.arena.text(delim.0) + delim.1
            };
        }
        let (open, close) = delim;
        let multi = {
            let mut inner = self.arena.nil();
            for item in self.items.iter() {
                match item {
                    Item::Comment(cmt) => inner += cmt.clone() + self.arena.hardline(),
                    Item::Commented { body, after } => {
                        inner += body.clone()
                            + sty.multi_line_sep
                            + after.clone()
                            + self.arena.hardline();
                    }
                }
            }
            (self.arena.hardline() + inner).nest(2).enclose(open, close)
        };
        let flat = || {
            let mut inner = self.arena.nil();
            let mut cnt = 0;
            for (i, item) in self.items.iter().enumerate() {
                match item {
                    Item::Comment(cmt) => inner += cmt.clone(),
                    Item::Commented { body, after } => {
                        cnt += 1;
                        inner += body.clone() + after.clone();
                        if i != self.items.len() - 1 {
                            inner += self.arena.text(sty.single_line_sep) + self.arena.space();
                        } else if cnt == 1 {
                            // trailing comma for one-size array
                            inner += sty.single_line_sep;
                        }
                    }
                }
            }
            if sty.add_space_if_empty {
                inner.enclose(
                    self.arena.text(open) + self.arena.space(),
                    self.arena.space() + close,
                )
            } else {
                inner.enclose(open, close)
            }
        };
        match self.fold_style {
            FoldStyle::Never => multi,
            FoldStyle::Fit => multi.clone().flat_alt(flat()).group(),
        }
    }
}
