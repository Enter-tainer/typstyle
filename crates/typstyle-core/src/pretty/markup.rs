use pretty::DocAllocator;
use smallvec::SmallVec;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{
    doc_ext::DocExt,
    layout::flow::FlowItem,
    util::{is_comment_node, is_only_one_and},
    ArenaDoc, Context, Mode, PrettyPrinter,
};
use crate::ext::StrExt;

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
    pub fn convert_markup(&'a self, ctx: Context, markup: Markup<'a>) -> ArenaDoc<'a> {
        self.convert_markup_impl(ctx, markup, MarkupScope::Document)
    }

    pub(super) fn convert_content_block(
        &'a self,
        ctx: Context,
        content_block: ContentBlock<'a>,
    ) -> ArenaDoc<'a> {
        let content = self
            .convert_markup_impl(ctx, content_block.body(), MarkupScope::ContentBlock)
            .nest(self.config.tab_spaces as isize);
        content.group().brackets()
    }

    pub(super) fn convert_strong(&'a self, ctx: Context, strong: Strong<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup_impl(ctx, strong.body(), MarkupScope::Strong);
        body.enclose("*", "*")
    }

    pub(super) fn convert_emph(&'a self, ctx: Context, emph: Emph<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup_impl(ctx, emph.body(), MarkupScope::Strong);
        body.enclose("_", "_")
    }

    pub(super) fn convert_raw(&'a self, _ctx: Context, raw: Raw<'a>) -> ArenaDoc<'a> {
        // no format multiline single backtick raw block
        if !raw.block() && raw.lines().count() > 1 {
            return self.convert_verbatim(raw);
        }

        let mut doc = self.arena.nil();
        for child in raw.to_untyped().children() {
            if let Some(delim) = child.cast::<RawDelim>() {
                doc += self.convert_trivia(delim);
            } else if let Some(lang) = child.cast::<RawLang>() {
                doc += self.convert_trivia(lang);
            } else if let Some(text) = child.cast::<Text>() {
                doc += self.convert_text(text);
            } else if child.kind() == SyntaxKind::RawTrimmed {
                doc += if child.text().has_linebreak() {
                    self.arena.hardline()
                } else {
                    self.arena.space()
                }
            }
        }
        doc
    }

    pub(super) fn convert_ref(&'a self, ctx: Context, reference: Ref<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("@") + self.arena.text(reference.target());
        if let Some(supplement) = reference.supplement() {
            doc += self.convert_content_block(ctx, supplement);
        }
        doc
    }

    pub(super) fn convert_heading(&'a self, ctx: Context, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(ctx, heading.to_untyped(), |ctx, child| {
            if child.kind() == SyntaxKind::HeadingMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup_impl(ctx, markup, MarkupScope::Item))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_list_item(
        &'a self,
        ctx: Context,
        list_item: ListItem<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_list_item_like(ctx, list_item.to_untyped())
    }

    pub(super) fn convert_enum_item(
        &'a self,
        ctx: Context,
        enum_item: EnumItem<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_list_item_like(ctx, enum_item.to_untyped())
    }

    pub(super) fn convert_term_item(
        &'a self,
        ctx: Context,
        term_item: TermItem<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_list_item_like(ctx, term_item.to_untyped())
    }

    fn convert_list_item_like(&'a self, ctx: Context, item: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.convert_flow_like(ctx, item, |ctx, child| match child.kind() {
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
                FlowItem::spaced(self.convert_markup_impl(
                    ctx,
                    child.cast().expect("markup"),
                    MarkupScope::Item,
                ))
            }
            _ => FlowItem::none(),
        })
        .nest(self.config.tab_spaces as isize)
    }

    fn convert_markup_impl(
        &'a self,
        ctx: Context,
        markup: Markup<'a>,
        scope: MarkupScope,
    ) -> ArenaDoc<'a> {
        let ctx = ctx.with_mode(Mode::Markup);

        if is_only_one_and(markup.to_untyped().children(), |node| {
            node.kind() == SyntaxKind::Space
        }) {
            return self.arena.space();
        }

        let repr = collect_markup_repr(markup);

        let mut doc = self.arena.nil();
        for MarkupLine {
            nodes,
            breaks,
            mixed_text,
        } in repr.lines
        {
            for node in nodes {
                doc += if node.kind() == SyntaxKind::Space {
                    self.arena.space()
                } else if let Some(text) = node.cast::<Text>() {
                    self.convert_text(text)
                } else if let Some(expr) = node.cast::<Expr>() {
                    let ctx = if mixed_text {
                        ctx.suppress_breaks()
                    } else {
                        ctx
                    };
                    self.convert_expr(ctx, expr)
                } else if is_comment_node(node) {
                    self.convert_comment(ctx, node)
                } else {
                    // can be Hash, Semicolon, Shebang
                    self.convert_trivia_untyped(node)
                };
            }
            if breaks > 0 {
                doc += self.arena.hardline().repeat_n(breaks);
            }
        }

        // Add line or space (if any) to both sides.
        // Only turn space into, not the other way around.
        let has_line_break = self.attr_store.is_multiline(markup.to_untyped());
        let is_symmetric = repr.start_bound != Boundary::Nil && repr.end_bound != Boundary::Nil;
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
                        || ctx.break_suppressed
                    {
                        self.arena.nil()
                    } else {
                        self.arena.line_()
                    }
                }
                Boundary::SpaceOrBreak | Boundary::WeakSpaceOrBreak => {
                    if is_symmetric && !ctx.break_suppressed || has_line_break {
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
        doc.enclose(get_delim(repr.start_bound), get_delim(repr.end_bound))
    }
}

#[derive(Default)]
struct MarkupLine<'a> {
    nodes: SmallVec<[&'a SyntaxNode; 4]>,
    breaks: usize,
    mixed_text: bool,
}

struct MarkupRepr<'a> {
    lines: Vec<MarkupLine<'a>>,
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
fn collect_markup_repr(markup: Markup<'_>) -> MarkupRepr {
    let mut repr = MarkupRepr {
        lines: vec![],
        start_bound: Boundary::Nil,
        end_bound: Boundary::Nil,
    };
    let mut current_line = MarkupLine::default();
    for node in markup.to_untyped().children() {
        let break_line = match node.kind() {
            SyntaxKind::Parbreak => {
                current_line.breaks = node.text().count_linebreaks(); // This is >= 2
                true
            }
            SyntaxKind::Space if current_line.nodes.is_empty() => {
                repr.start_bound = Boundary::from_space(node.text());
                continue;
            }
            SyntaxKind::Space if node.text().has_linebreak() => {
                current_line.breaks = 1; // Must only one
                true
            }
            _ => {
                if matches!(
                    node.kind(),
                    SyntaxKind::Text | SyntaxKind::Strong | SyntaxKind::Emph | SyntaxKind::Raw
                ) {
                    current_line.mixed_text = true;
                }
                if current_line.nodes.is_empty() && is_block_elem(node) {
                    repr.start_bound = repr.start_bound.strip_space();
                }
                current_line.nodes.push(node);
                false
            }
        };
        if break_line {
            repr.lines.push(current_line);
            current_line = MarkupLine::default();
        }
    }
    if !current_line.nodes.is_empty() {
        repr.lines.push(current_line);
    }

    // Remove trailing spaces
    if let Some(last_line) = repr.lines.last_mut() {
        if last_line.breaks > 0 {
            last_line.breaks -= 1;
            repr.end_bound = Boundary::Break;
        }
        while let Some(last) = last_line.nodes.last() {
            if last.kind() == SyntaxKind::Space {
                repr.end_bound = Boundary::from_space(last.text());
                last_line.nodes.pop();
            } else {
                if is_block_elem(last) {
                    repr.end_bound = repr.end_bound.strip_space();
                }
                break;
            }
        }
    }

    // Check boundary through comments
    if repr.start_bound == Boundary::Nil {
        if let Some(first_line) = repr.lines.first() {
            match first_line.nodes.iter().find(|it| !is_comment_node(it)) {
                Some(it) if is_block_elem(it) => {
                    repr.start_bound = Boundary::NilOrBreak;
                }
                Some(it) if it.kind() == SyntaxKind::Space => {
                    repr.start_bound = Boundary::WeakSpaceOrBreak;
                }
                None if !first_line.nodes.is_empty() => repr.start_bound = Boundary::WeakBreak,
                _ => {}
            }
        }
    }
    if repr.end_bound == Boundary::Nil {
        if let Some(last_line) = repr.lines.last() {
            match last_line.nodes.iter().rfind(|it| !is_comment_node(it)) {
                Some(it) if is_block_elem(it) => {
                    repr.end_bound = Boundary::NilOrBreak;
                }
                Some(it) if it.kind() == SyntaxKind::Space => {
                    repr.end_bound = Boundary::WeakSpaceOrBreak;
                }
                None if !last_line.nodes.is_empty() => repr.end_bound = Boundary::WeakBreak,
                _ => {}
            }
        }
    }

    repr
}

fn is_block_elem(it: &'_ SyntaxNode) -> bool {
    matches!(
        it.kind(),
        SyntaxKind::ListItem | SyntaxKind::EnumItem | SyntaxKind::TermItem
    )
}
