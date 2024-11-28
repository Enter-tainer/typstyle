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
    item_count: usize,
    has_line_comment: bool,
    fold_style: FoldStyle,
}

struct ListStyle {
    /// The separator between items.
    separator: &'static str,
    /// The delimiter of the list.
    delim: (&'static str, &'static str),
    /// Whether to add an addition space inside the delimiters if the list is empty.
    add_space_if_empty: bool,
    /// Whether a trailing single-line separator is need if the list contains only one item.
    add_trailing_sep_single: bool,
    /// Whether can omit the delimiter if the list contains only one item.
    omit_delim_single: bool,
    /// Whether can omit the delimiter if the list is flat.
    omit_delim_flat: bool,
}

impl<'a> ListStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            arena: &printer.arena,
            printer,
            can_attach: false,
            free_comments: Default::default(),
            items: Default::default(),
            item_count: 0,
            has_line_comment: false,
            fold_style: FoldStyle::Fit,
        }
    }

    pub fn convert_array(mut self, array: Array<'a>) -> ArenaDoc<'a> {
        self.fold_style = self.printer.get_fold_style(array);
        self.process_list(array.to_untyped(), |node| {
            self.printer.convert_array_item(node)
        });

        self.pretty_commented_items(ListStyle {
            separator: ",",
            delim: ("(", ")"),
            add_space_if_empty: false,
            add_trailing_sep_single: true,
            omit_delim_single: false,
            omit_delim_flat: false,
        })
    }

    pub fn convert_dict(mut self, dict: Dict<'a>) -> ArenaDoc<'a> {
        let all_spread = dict.items().all(|item| matches!(item, DictItem::Spread(_)));

        self.process_list(dict.to_untyped(), |node| {
            self.printer.convert_dict_item(node)
        });

        self.pretty_commented_items(ListStyle {
            separator: ",",
            delim: (if all_spread { "(:" } else { "(" }, ")"),
            add_space_if_empty: false,
            add_trailing_sep_single: false,
            omit_delim_single: false,
            omit_delim_flat: false,
        })
    }

    pub fn convert_destructuring(mut self, destructuring: Destructuring<'a>) -> ArenaDoc<'a> {
        let only_one_pattern = is_only_one_and(destructuring.items(), |it| {
            matches!(*it, DestructuringItem::Pattern(_))
        });

        self.process_list(destructuring.to_untyped(), |node| {
            self.printer.convert_destructuring_item(node)
        });

        if only_one_pattern {
            self.fold_style = FoldStyle::Always;
        }

        self.pretty_commented_items(ListStyle {
            separator: ",",
            delim: ("(", ")"),
            add_space_if_empty: false,
            add_trailing_sep_single: only_one_pattern,
            omit_delim_single: false,
            omit_delim_flat: false,
        })
    }

    pub fn convert_params(mut self, params: Params<'a>, is_unnamed: bool) -> ArenaDoc<'a> {
        let is_single_simple = is_unnamed
            && is_only_one_and(params.children(), |it| {
                matches!(
                    *it,
                    Param::Pos(Pattern::Normal(_)) | Param::Pos(Pattern::Placeholder(_))
                )
            });

        self.process_list(params.to_untyped(), |node| self.printer.convert_param(node));

        if is_single_simple {
            self.fold_style = FoldStyle::Always;
        }

        self.pretty_commented_items(ListStyle {
            separator: ",",
            delim: ("(", ")"),
            add_space_if_empty: false,
            add_trailing_sep_single: false,
            omit_delim_single: is_single_simple,
            omit_delim_flat: false,
        })
    }

    pub fn convert_import_items(mut self, import_items: ImportItems<'a>) -> ArenaDoc<'a> {
        // Note that `ImportItem` does not implement `AstNode`.
        self.process_list_impl(import_items.to_untyped(), |child| match child.kind() {
            SyntaxKind::RenamedImportItem => child
                .cast()
                .map(|item| self.printer.convert_import_item_renamed(item)),
            SyntaxKind::ImportItemPath => child
                .cast()
                .map(|item| self.printer.convert_import_item_path(item)),
            _ => Option::None,
        });

        self.pretty_commented_items(ListStyle {
            separator: ",",
            delim: ("(", ")"),
            add_space_if_empty: false,
            add_trailing_sep_single: false,
            omit_delim_single: true,
            omit_delim_flat: true,
        })
    }

    /// Process a list of AstNodes.
    fn process_list<T: AstNode<'a>>(
        &mut self,
        list_node: &'a SyntaxNode,
        item_converter: impl Fn(T) -> ArenaDoc<'a>,
    ) {
        self.process_list_impl(list_node, |node| node.cast().map(&item_converter));
    }

    fn process_list_impl(
        &mut self,
        list_node: &'a SyntaxNode,
        item_checker: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
    ) {
        // Each item can be attached with comments at the front and back.
        // Can break line after front attachments.
        // If the back attachment appears before the comma, the comma is move to its front if multiline.

        self.fold_style = self.printer.get_fold_style_untyped(list_node);

        for node in list_node.children() {
            if let Some(item_body) = item_checker(node) {
                self.item_count += 1;
                let before = if self.free_comments.is_empty() {
                    self.arena.nil()
                } else {
                    self.arena
                        .intersperse(self.free_comments.drain(..), self.arena.line())
                        + self.arena.line()
                };
                self.items.push(Item::Commented {
                    body: (before + item_body).group(),
                    after: self.arena.nil(),
                });
                self.can_attach = true;
            } else if is_comment_node(node) {
                // Line comment cannot appear in single line block
                if node.kind() == SyntaxKind::LineComment {
                    self.has_line_comment = true;
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
        let multi = || {
            let mut inner = self.arena.nil();
            for item in self.items.iter() {
                match item {
                    Item::Comment(cmt) => inner += cmt.clone() + self.arena.hardline(),
                    Item::Commented { body, after } => {
                        inner +=
                            body.clone() + sty.separator + after.clone() + self.arena.hardline();
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
                            inner += self.arena.text(sty.separator) + self.arena.space();
                        } else if cnt == 1 && sty.add_trailing_sep_single {
                            // trailing comma for one-size array
                            inner += sty.separator;
                        }
                    }
                }
            }
            if cnt == 1 && sty.omit_delim_single || sty.omit_delim_flat {
                inner
            } else if sty.add_space_if_empty {
                inner.enclose(
                    self.arena.text(open) + self.arena.space(),
                    self.arena.space() + close,
                )
            } else {
                inner.enclose(open, close)
            }
        };
        let fold_style = if self.has_line_comment {
            FoldStyle::Never
        } else {
            self.fold_style
        };
        match fold_style {
            FoldStyle::Never => multi(),
            FoldStyle::Fit => multi().flat_alt(flat()).group(),
            FoldStyle::Always => flat(),
        }
    }
}

fn is_only_one_and<T>(mut iterator: impl Iterator<Item = T>, f: impl FnOnce(&T) -> bool) -> bool {
    iterator
        .next()
        .is_some_and(|first| f(&first) && iterator.next().is_none())
}
