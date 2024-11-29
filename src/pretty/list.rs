use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{style::FoldStyle, util::is_comment_node, ArenaDoc, PrettyPrinter};

enum Item<'a> {
    /// Detached comments that can be put on a line.
    Comment(ArenaDoc<'a>),
    /// List item with attached comments.
    Commented {
        /// The list item.
        body: ArenaDoc<'a>,
        /// Attached comments. Leading space included.
        after: Option<ArenaDoc<'a>>,
    },
}

pub struct ListStylist<'a> {
    printer: &'a PrettyPrinter<'a>,
    can_attach: bool,
    free_comments: Vec<ArenaDoc<'a>>,
    items: Vec<Item<'a>>,
    item_count: usize,
    has_line_comment: bool,
    fold_style: FoldStyle,
}

pub struct ListStyle {
    /// The separator between items.
    pub separator: &'static str,
    /// The delimiter of the list.
    pub delim: (&'static str, &'static str),
    /// Whether to add an addition space inside the delimiters if the list is empty. Currently not used.
    pub add_space_if_empty: bool,
    /// Whether a trailing single-line separator is need if the list contains only one item.
    pub add_trailing_sep_single: bool,
    /// Whether can omit the delimiter if the list contains only one item.
    pub omit_delim_single: bool,
    /// Whether can omit the delimiter if the list is flat.
    pub omit_delim_flat: bool,
}

impl<'a> ListStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            printer,
            can_attach: false,
            free_comments: Default::default(),
            items: Default::default(),
            item_count: 0,
            has_line_comment: false,
            fold_style: FoldStyle::Fit,
        }
    }

    pub fn always_fold_if(mut self, pred: impl FnOnce() -> bool) -> Self {
        if pred() {
            self.fold_style = FoldStyle::Always;
        }
        self
    }

    /// Process a list of `AstNode`'s.
    pub fn process_list<T: AstNode<'a>>(
        self,
        list_node: &'a SyntaxNode,
        item_converter: impl Fn(T) -> ArenaDoc<'a>,
    ) -> Self {
        self.process_list_impl(list_node, |node| node.cast().map(&item_converter))
    }

    /// Process a list of any nodes. Only use this when the node does not implement `AstNode`.
    pub fn process_list_impl(
        mut self,
        list_node: &'a SyntaxNode,
        item_checker: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
    ) -> Self {
        // Each item can be attached with comments at the front and back.
        // Can break line after front attachments.
        // If the back attachment appears before the comma, the comma is move to its front if multiline.

        self.fold_style = self.printer.get_fold_style_untyped(list_node);

        let arena = &self.printer.arena;

        for node in list_node.children() {
            if let Some(item_body) = item_checker(node) {
                self.item_count += 1;
                let before = if self.free_comments.is_empty() {
                    arena.nil()
                } else {
                    arena.intersperse(self.free_comments.drain(..), arena.line()) + arena.line()
                };
                self.items.push(Item::Commented {
                    body: (before + item_body).group(),
                    after: None,
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

        self
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
            let arena = &self.printer.arena;
            if let Some(Item::Commented { after, .. }) = self.items.last_mut() {
                let added =
                    arena.space() + arena.intersperse(self.free_comments.drain(..), arena.space());
                match after {
                    Some(cmt) => *cmt += added,
                    Option::None => *after = Some(added),
                }
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

    /// Create Doc from items in self.
    ///
    /// For attached comments:
    /// - break: `xxx, /* yyy */`, `xxx,`
    /// - flat: `xxx /* yyy */, `, `xxx, `
    pub fn print_doc(self, sty: ListStyle) -> ArenaDoc<'a> {
        let arena = &self.printer.arena;

        let delim = sty.delim;
        if self.items.is_empty() {
            return if sty.add_space_if_empty {
                arena.text(delim.0) + arena.space() + delim.1
            } else {
                arena.text(delim.0) + delim.1
            };
        }

        let is_single = self.item_count == 1;
        let sep = arena.text(sty.separator);
        let fold_style = if self.has_line_comment {
            FoldStyle::Never
        } else {
            self.fold_style
        };
        match fold_style {
            FoldStyle::Never => {
                let mut inner = arena.nil();
                for item in self.items.into_iter() {
                    match item {
                        Item::Comment(cmt) => inner += cmt + arena.hardline(),
                        Item::Commented { body, after } => {
                            inner += body + sep.clone() + after + arena.hardline();
                        }
                    }
                }
                (arena.hardline() + inner).nest(2).enclose(delim.0, delim.1)
            }
            FoldStyle::Always => {
                let mut inner = arena.nil();
                for (i, item) in self.items.into_iter().enumerate() {
                    match item {
                        Item::Comment(cmt) => inner += cmt,
                        Item::Commented { body, after } => {
                            inner += body + after;
                            if i != self.item_count - 1 {
                                inner += sep.clone() + arena.space();
                            } else if is_single && sty.add_trailing_sep_single {
                                // trailing comma for one-size array
                                inner += sep.clone();
                            }
                        }
                    }
                }
                if is_single && sty.omit_delim_single || sty.omit_delim_flat {
                    inner
                } else {
                    inner.enclose(delim.0, delim.1)
                }
            }
            FoldStyle::Fit => {
                let mut inner = arena.nil();
                for (i, item) in self.items.into_iter().enumerate() {
                    let is_last = i == self.item_count - 1;
                    match item {
                        Item::Comment(cmt) => inner += cmt + arena.line(),
                        Item::Commented {
                            body,
                            after: Option::None,
                        } => {
                            let follow = if is_single && sty.add_trailing_sep_single || !is_last {
                                sep.clone()
                            } else {
                                sep.clone().flat_alt(arena.nil())
                            };
                            let ln = if is_last { arena.line_() } else { arena.line() };
                            inner += body + follow + ln;
                        }
                        Item::Commented {
                            body,
                            after: Some(after),
                        } => {
                            let follow_break = sep.clone() + after.clone();
                            let follow_flat =
                                if !is_last || is_single && sty.add_trailing_sep_single {
                                    after + sep.clone()
                                } else {
                                    after
                                };
                            let ln = if is_last { arena.line_() } else { arena.line() };
                            inner += body + follow_break.flat_alt(follow_flat) + ln;
                        }
                    }
                }
                if is_single && sty.omit_delim_single {
                    inner
                } else {
                    inner = (arena.line_() + inner).nest(2);
                    if sty.omit_delim_flat {
                        inner
                            .enclose(
                                arena.text(delim.0).flat_alt(arena.nil()),
                                arena.text(delim.1).flat_alt(arena.nil()),
                            )
                            .group()
                    } else {
                        inner.group().enclose(delim.0, delim.1)
                    }
                }
            }
        }
    }
}
