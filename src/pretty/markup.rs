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
    pub strip_space: bool,
}

impl<'a> PrettyPrinter<'a> {
    pub fn convert_markup(&'a self, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(markup, Default::default())
    }

    fn convert_markup_impl(&'a self, markup: Markup<'a>, sty: MarkupStyle) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Markup);

        if is_only_one_and(markup.to_untyped().children(), |node| {
            node.kind() == SyntaxKind::Space
        }) {
            return self.arena.space();
        }

        let mut lines = collect_lines(markup);
        let mut has_leading_space = false;
        let mut has_trailing_space = false;
        // Strip spaces, which are handled later.
        if !lines.is_empty() {
            let first_line = lines.first_mut().unwrap();
            if first_line
                .nodes
                .first()
                .is_some_and(|node| node.kind() == SyntaxKind::Space)
            {
                first_line.nodes.remove(0);
                has_leading_space = true;
            }
            let last_line = lines.last_mut().unwrap();
            if last_line
                .nodes
                .last()
                .is_some_and(|node| node.kind() == SyntaxKind::Space)
            {
                last_line.nodes.pop();
                has_trailing_space = true;
            }
        }

        let mut doc = self.arena.nil();
        for Line { has_text, nodes } in lines {
            for node in nodes {
                if let Some(space) = node.cast::<Space>() {
                    doc += self.convert_space(space);
                    continue;
                }
                if let Some(pb) = node.cast::<Parbreak>() {
                    doc += self.convert_parbreak(pb);
                    continue;
                }
                doc += if has_text {
                    self.format_disabled(node)
                } else if let Some(expr) = node.cast::<Expr>() {
                    self.convert_expr(expr)
                } else if is_comment_node(node) {
                    self.convert_comment(node)
                } else {
                    trivia_strip_prefix(&self.arena, node)
                };
            }
        }

        // Add line or space (if any) to both sides.
        let has_line_break = self.attr_store.is_multiline(markup.to_untyped());
        let get_delim = |has_space: bool| {
            if !has_space || sty.strip_space {
                self.arena.nil()
            } else if has_line_break {
                self.arena.hardline()
            } else {
                self.arena.line()
            }
        };

        doc.enclose(get_delim(has_leading_space), get_delim(has_trailing_space))
    }

    pub(super) fn convert_heading(&'a self, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(heading.to_untyped(), |child| {
            if child.kind() == SyntaxKind::HeadingMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup(markup))
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
                FlowItem::spaced(self.convert_markup(markup).nest(2))
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
                FlowItem::spaced(self.convert_markup(markup).nest(2))
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
                let doc = self.convert_markup_impl(markup, MarkupStyle { strip_space: true });
                FlowItem::spaced(doc.nest(2))
            } else {
                FlowItem::none()
            }
        })
    }
}

#[derive(Default)]
struct Line<'a> {
    has_text: bool,
    nodes: Vec<&'a SyntaxNode>,
}

// Break markup into lines, split by stmt, parbreak, newline, multiline raw,
// equation if a line contains text, it will be skipped by the formatter
// to keep the original format.
fn collect_lines(markup: Markup<'_>) -> Vec<Line<'_>> {
    let mut lines: Vec<Line> = vec![];
    let mut current_line = Line {
        has_text: false,
        nodes: vec![],
    };
    for node in markup.to_untyped().children() {
        let mut break_line = false;
        if let Some(space) = node.cast::<Space>() {
            if space.to_untyped().text().has_linebreak() {
                break_line = true;
            }
        } else if let Some(pb) = node.cast::<Parbreak>() {
            if pb.to_untyped().text().has_linebreak() {
                break_line = true;
            }
        } else if node.kind().is_stmt() {
            break_line = true;
        } else if let Some(expr) = node.cast::<Expr>() {
            match expr {
                Expr::Text(_) => current_line.has_text = true,
                Expr::Raw(r) => {
                    if r.block() {
                        break_line = true;
                    } else {
                        current_line.has_text = true;
                    }
                }
                Expr::Strong(_) | Expr::Emph(_) => current_line.has_text = true,
                Expr::Code(_) => break_line = true,
                Expr::Equation(e) if e.block() => break_line = true,
                _ => (),
            }
        }
        current_line.nodes.push(node);
        if break_line {
            lines.push(current_line);
            current_line = Line::default();
        }
    }
    if !current_line.nodes.is_empty() {
        lines.push(current_line);
    }
    lines
}
