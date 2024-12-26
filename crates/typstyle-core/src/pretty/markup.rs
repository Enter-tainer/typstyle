use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use crate::{ext::StrExt, pretty::mode::Mode};

use super::{
    flow::FlowItem,
    trivia_strip_prefix,
    util::{is_comment_node, is_only_one_and},
    ArenaDoc, PrettyPrinter,
};

#[derive(Default)]
struct MarkupStyle {
    /// Whether this markup is top-level.
    pub is_top_level: bool,
    /// Whether the spaces can be safely stripped.
    pub can_strip_space: bool,
}

impl<'a> PrettyPrinter<'a> {
    pub fn convert_markup(&'a self, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(
            markup,
            MarkupStyle {
                is_top_level: true,
                ..Default::default()
            },
        )
    }

    pub fn convert_markup_in_block(&'a self, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(markup, Default::default())
    }

    fn convert_markup_stripped(&'a self, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(
            markup,
            MarkupStyle {
                can_strip_space: true,
                ..Default::default()
            },
        )
    }

    fn convert_markup_impl(&'a self, markup: Markup<'a>, sty: MarkupStyle) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Markup);

        if is_only_one_and(markup.to_untyped().children(), |node| {
            node.kind() == SyntaxKind::Space
        }) {
            return self.arena.space();
        }

        let items = collect_markup_items(markup, sty.is_top_level);

        let mut doc = self.arena.nil();
        for MarkupItem {
            node,
            format_disabled,
        } in items.items
        {
            if let Some(space) = node.cast::<Space>() {
                doc += self.convert_space(space);
                continue;
            }
            if let Some(pb) = node.cast::<Parbreak>() {
                doc += self.convert_parbreak(pb);
                continue;
            }
            doc += if format_disabled {
                self.format_disabled(node)
            } else if let Some(expr) = node.cast::<Expr>() {
                self.convert_expr(expr)
            } else if is_comment_node(node) {
                self.convert_comment(node)
            } else {
                trivia_strip_prefix(&self.arena, node)
            };
        }

        // Add line or space (if any) to both sides.
        // Only turn space into, not the other way around.
        let has_line_break = self.attr_store.is_multiline(markup.to_untyped());
        if items.has_leading_space && items.has_trailing_space {
            if sty.can_strip_space {
                doc
            } else if has_line_break {
                doc.enclose(self.arena.hardline(), self.arena.hardline())
            } else {
                doc.enclose(self.arena.line(), self.arena.line())
            }
        } else {
            // The asymmetric case
            let get_delim = |has_space: bool| {
                if !has_space || sty.can_strip_space {
                    self.arena.nil()
                } else if has_line_break {
                    self.arena.hardline()
                } else {
                    self.arena.space()
                }
            };
            doc.enclose(
                get_delim(items.has_leading_space),
                get_delim(items.has_trailing_space),
            )
        }
    }

    pub(super) fn convert_heading(&'a self, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(heading.to_untyped(), |child| {
            if child.kind() == SyntaxKind::HeadingMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup_stripped(markup))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_list_item(&'a self, list_item: ListItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(list_item.to_untyped(), |child| {
            if child.kind() == SyntaxKind::ListMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(
                    self.convert_markup_stripped(markup)
                        .nest(self.config.tab_spaces as isize),
                )
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_enum_item(&'a self, enum_item: EnumItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(enum_item.to_untyped(), |child| {
            if child.kind() == SyntaxKind::EnumMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(
                    self.convert_markup_stripped(markup)
                        .nest(self.config.tab_spaces as isize),
                )
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_term_item(&'a self, term: TermItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(term.to_untyped(), |child| {
            if child.kind() == SyntaxKind::TermMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if child.kind() == SyntaxKind::Colon {
                FlowItem::tight_spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                // Here we can safely strip the space. Should never turn space into line.
                let doc = self.convert_markup_stripped(markup);
                FlowItem::spaced(doc.nest(self.config.tab_spaces as isize))
            } else {
                FlowItem::none()
            }
        })
    }
}

struct MarkupItem<'a> {
    node: &'a SyntaxNode,
    format_disabled: bool,
}

struct MarkupItems<'a> {
    items: Vec<MarkupItem<'a>>,
    has_leading_space: bool,
    has_trailing_space: bool,
}

// Break markup into lines, split by stmt, parbreak, newline, multiline raw,
// equation if a line contains text, it will be skipped by the formatter
// to keep the original format.
fn collect_markup_items(markup: Markup<'_>, is_top_level: bool) -> MarkupItems {
    let mut items = MarkupItems {
        items: vec![],
        has_leading_space: false,
        has_trailing_space: false,
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
                Expr::Text(_) => current_line_has_text = true,
                Expr::Raw(r) => {
                    if r.block() {
                        break_line = true;
                    } else {
                        current_line_has_text = true;
                    }
                }
                Expr::Strong(_) | Expr::Emph(_) => current_line_has_text = true,
                Expr::Code(_) => break_line = true,
                Expr::Equation(e) if e.block() => break_line = true,
                _ => (),
            }
        }
        if node.kind() == SyntaxKind::Space && items.items.is_empty() {
            // Discard leading space and mark it.
            if !is_top_level || node.text().has_linebreak() {
                // Only spaces with linebreaks are counted at document level
                items.has_leading_space = true;
            }
        } else {
            items.items.push(MarkupItem {
                node,
                format_disabled: false,
            });
        }
        if break_line {
            if current_line_has_text {
                for item in &mut items.items[cursor..] {
                    item.format_disabled = true;
                }
            }
            cursor = items.items.len();
            current_line_has_text = false;
        }
    }
    if current_line_has_text {
        for item in &mut items.items[cursor..] {
            item.format_disabled = true;
        }
    }

    // Remove trailing spaces
    while items
        .items
        .last()
        .is_some_and(|last| last.node.kind() == SyntaxKind::Space)
    {
        items.items.pop();
        items.has_trailing_space = true;
    }

    // Check space inside comments
    if !is_top_level
        && !items.has_leading_space
        && items
            .items
            .iter()
            .find(|item| !is_comment_node(item.node))
            .is_some_and(|it| it.node.kind() == SyntaxKind::Space)
    {
        items.has_leading_space = true;
    }
    if !is_top_level
        && !items.has_trailing_space
        && items
            .items
            .iter()
            .rev()
            .find(|item| !is_comment_node(item.node))
            .is_some_and(|it| it.node.kind() == SyntaxKind::Space)
    {
        items.has_trailing_space = true;
    }

    items
}
