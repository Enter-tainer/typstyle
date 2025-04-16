use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{
    doc_ext::DocExt,
    layout::flow::FlowItem,
    trivia_strip_prefix,
    util::{is_comment_node, is_only_one_and},
    ArenaDoc, PrettyPrinter,
};
use crate::{ext::StrExt, pretty::mode::Mode};

#[derive(Debug, PartialEq, Eq)]
enum MarkupScope {
    /// The top-level markup.
    Document,
    /// Markup enclosed by `[]`.
    ContentBlock,
    /// Strong or Emph.
    Strong,
    /// ListItem, EnumItem, TermItem, Heading. Spaces without linebreaks can be stripped.
    Item,
}

impl<'a> PrettyPrinter<'a> {
    pub fn convert_markup(&'a self, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(markup, MarkupScope::Document)
    }

    pub(super) fn convert_content_block(&'a self, content_block: ContentBlock<'a>) -> ArenaDoc<'a> {
        let content = self
            .convert_markup_impl(content_block.body(), MarkupScope::ContentBlock)
            .nest(self.config.tab_spaces as isize);
        content.group().brackets()
    }

    pub(super) fn convert_strong(&'a self, strong: Strong<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup_impl(strong.body(), MarkupScope::Strong);
        body.enclose("*", "*")
    }

    pub(super) fn convert_emph(&'a self, emph: Emph<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup_impl(emph.body(), MarkupScope::Strong);
        body.enclose("_", "_")
    }

    pub(super) fn convert_raw(&'a self, raw: Raw<'a>) -> ArenaDoc<'a> {
        // no format multiline single backtick raw block
        if !raw.block() && raw.lines().count() > 1 {
            return self.format_disabled(raw.to_untyped());
        }

        let mut doc = self.arena.nil();
        for child in raw.to_untyped().children() {
            if let Some(delim) = child.cast::<RawDelim>() {
                doc += self.convert_verbatim(delim);
            } else if let Some(lang) = child.cast::<RawLang>() {
                doc += self.convert_verbatim(lang);
            } else if let Some(line) = child.cast::<Text>() {
                doc += self.convert_text(line);
            } else if child.kind() == SyntaxKind::RawTrimmed {
                if child.text().has_linebreak() {
                    doc += self.arena.hardline();
                } else {
                    doc += self.arena.space();
                }
            }
        }
        doc
    }

    pub(super) fn convert_ref(&'a self, reference: Ref<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("@") + self.arena.text(reference.target());
        if let Some(supplement) = reference.supplement() {
            doc += self.convert_content_block(supplement);
        }
        doc
    }

    pub(super) fn convert_heading(&'a self, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(heading.to_untyped(), |child| {
            if child.kind() == SyntaxKind::HeadingMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup_impl(markup, MarkupScope::Item))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_list_item(&'a self, list_item: ListItem<'a>) -> ArenaDoc<'a> {
        self.convert_list_item_like(list_item.to_untyped())
    }

    pub(super) fn convert_enum_item(&'a self, enum_item: EnumItem<'a>) -> ArenaDoc<'a> {
        self.convert_list_item_like(enum_item.to_untyped())
    }

    pub(super) fn convert_term_item(&'a self, term_item: TermItem<'a>) -> ArenaDoc<'a> {
        self.convert_list_item_like(term_item.to_untyped())
    }

    fn convert_list_item_like(&'a self, item: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.convert_flow_like(item, |child| match child.kind() {
            SyntaxKind::ListMarker | SyntaxKind::EnumMarker | SyntaxKind::TermMarker => {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            }
            SyntaxKind::Colon => FlowItem::tight_spaced(self.arena.text(child.text().as_str())),
            SyntaxKind::Space if child.text().has_linebreak() => {
                FlowItem::tight(self.arena.hardline())
            }
            SyntaxKind::Parbreak => FlowItem::tight(
                self.arena
                    .hardline()
                    .repeat_n(child.text().count_linebreaks()),
            ),
            SyntaxKind::Markup if child.children().next().is_some() => {
                // empty markup is ignored here
                FlowItem::spaced(
                    self.convert_markup_impl(child.cast().expect("markup"), MarkupScope::Item),
                )
            }
            _ => FlowItem::none(),
        })
        .nest(self.config.tab_spaces as isize)
    }

    fn convert_markup_impl(&'a self, markup: Markup<'a>, scope: MarkupScope) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Markup);

        if is_only_one_and(markup.to_untyped().children(), |node| {
            node.kind() == SyntaxKind::Space
        }) {
            return self.arena.space();
        }

        let items = collect_markup_items(markup);

        let mut doc = self.arena.nil();
        for MarkupItem { node, mixed_text } in items.items {
            if let Some(space) = node.cast::<Space>() {
                doc += self.convert_space(space);
                continue;
            }
            if let Some(pb) = node.cast::<Parbreak>() {
                doc += self.convert_parbreak(pb);
                continue;
            }
            doc += if let Some(expr) = node.cast::<Expr>() {
                if mixed_text {
                    let _g = self.suppress_breaks();
                    let doc = self.convert_expr(expr);
                    doc
                } else {
                    self.convert_expr(expr)
                }
            } else if is_comment_node(node) {
                self.convert_comment(node)
            } else {
                trivia_strip_prefix(&self.arena, node)
            };
        }

        // Add line or space (if any) to both sides.
        // Only turn space into, not the other way around.
        let has_line_break = self.attr_store.is_multiline(markup.to_untyped());
        let is_symmetric = items.start_bound != Boundary::Nil && items.end_bound != Boundary::Nil;
        let break_suppressed = self.is_break_suppressed();
        let get_delim = |bound: Boundary| {
            if scope == MarkupScope::Document || scope == MarkupScope::Item {
                // should not add extra lines to the document
                return if bound == Boundary::Break {
                    self.arena.hardline()
                } else {
                    self.arena.nil()
                };
            }
            match bound {
                Boundary::Nil => self.arena.nil(),
                Boundary::NilOrBreak => {
                    if scope == MarkupScope::Item
                        || !is_symmetric && !has_line_break
                        || break_suppressed
                    {
                        self.arena.nil()
                    } else {
                        self.arena.line_()
                    }
                }
                Boundary::SpaceOrBreak | Boundary::WeakSpaceOrBreak => {
                    if is_symmetric && !break_suppressed || has_line_break {
                        self.arena.line()
                    } else if scope == MarkupScope::Item {
                        // the space can be safely eaten
                        self.arena.nil()
                    } else {
                        self.arena.space()
                    }
                }
                Boundary::Break | Boundary::WeakBreak => self.arena.hardline(),
            }
        };
        doc.enclose(get_delim(items.start_bound), get_delim(items.end_bound))
    }
}

struct MarkupItem<'a> {
    node: &'a SyntaxNode,
    mixed_text: bool,
}

struct MarkupItems<'a> {
    items: Vec<MarkupItem<'a>>,
    start_bound: Boundary,
    end_bound: Boundary,
}

/// Markup boundary, deciding whether can break.
#[derive(Debug, PartialEq, Eq)]
enum Boundary {
    /// Should add no blank.
    Nil,
    /// Can add a space or linebreak when multiline.
    NilOrBreak,
    /// Can turn to a linebreak.
    SpaceOrBreak,
    /// Always breaks.
    Break,
    /// Can turn to a linebreak if not in document scope.
    WeakSpaceOrBreak,
    /// Always breaks if not in document scope.
    WeakBreak,
}

impl Boundary {
    fn from_space(space: &str) -> Self {
        if space.has_linebreak() {
            Self::Break
        } else {
            Self::SpaceOrBreak
        }
    }

    fn from_space_weak(space: &str) -> Self {
        if space.has_linebreak() {
            Self::WeakBreak
        } else {
            Self::WeakSpaceOrBreak
        }
    }

    fn strip_space(self) -> Self {
        match self {
            Self::SpaceOrBreak => Self::NilOrBreak,
            _ => self,
        }
    }
}

// Break markup into lines, split by stmt, parbreak, newline, multiline raw,
// equation if a line contains text, it will be skipped by the formatter
// to keep the original format.
fn collect_markup_items(markup: Markup<'_>) -> MarkupItems {
    let mut items = MarkupItems {
        items: vec![],
        start_bound: Boundary::Nil,
        end_bound: Boundary::Nil,
    };
    let mut cursor = 0;
    let mut current_line_has_text = false;
    for node in markup.to_untyped().children() {
        let mut break_line = false;
        if (node.kind() == SyntaxKind::Space || node.kind() == SyntaxKind::Parbreak)
            && node.text().has_linebreak()
            || node.kind().is_stmt()
        {
            break_line = true;
        } else if let Some(expr) = node.cast::<Expr>() {
            match expr {
                Expr::Text(_) | Expr::Strong(_) | Expr::Emph(_) => current_line_has_text = true,
                Expr::Raw(r) => {
                    if r.block() {
                        break_line = true;
                    } else {
                        current_line_has_text = true;
                    }
                }
                Expr::Code(_) => break_line = true,
                Expr::Equation(e) if e.block() => break_line = true,
                _ => (),
            }
        }
        if node.kind() == SyntaxKind::Space && items.items.is_empty() {
            // Discard leading space and mark it.
            items.start_bound = Boundary::from_space(node.text());
        } else {
            if items.items.is_empty() && is_block_elem_untyped(node) {
                items.start_bound = items.start_bound.strip_space();
            }
            items.items.push(MarkupItem {
                node,
                mixed_text: false,
            });
        }
        if break_line {
            if current_line_has_text {
                for item in &mut items.items[cursor..] {
                    item.mixed_text = true;
                }
            }
            cursor = items.items.len();
            current_line_has_text = false;
        }
    }
    if current_line_has_text {
        for item in &mut items.items[cursor..] {
            item.mixed_text = true;
        }
    }

    // Remove trailing spaces
    while let Some(last) = items.items.last() {
        if last.node.kind() != SyntaxKind::Space {
            if is_block_elem(last) {
                items.end_bound = items.end_bound.strip_space();
            }
            break;
        }
        items.end_bound = Boundary::from_space(last.node.text());
        items.items.pop();
    }

    // Check boundary through comments
    if items.start_bound == Boundary::Nil {
        match items.items.iter().find(|item| !is_comment_node(item.node)) {
            Some(it) if it.node.kind() == SyntaxKind::Space => {
                items.start_bound = Boundary::from_space_weak(it.node.text());
            }
            Some(it) if is_block_elem(it) => {
                items.start_bound = Boundary::NilOrBreak;
            }
            _ => {}
        }
    }
    if items.end_bound == Boundary::Nil {
        match (items.items.iter().rev()).find(|item| !is_comment_node(item.node)) {
            Some(it) if it.node.kind() == SyntaxKind::Space => {
                items.end_bound = Boundary::from_space_weak(it.node.text());
            }
            Some(it) if is_block_elem(it) => {
                items.end_bound = Boundary::NilOrBreak;
            }
            _ => {}
        }
    }

    items
}

fn is_block_elem(it: &MarkupItem<'_>) -> bool {
    is_block_elem_untyped(it.node)
}

fn is_block_elem_untyped(it: &'_ SyntaxNode) -> bool {
    matches!(
        it.kind(),
        SyntaxKind::ListItem | SyntaxKind::EnumItem | SyntaxKind::TermItem
    )
}
