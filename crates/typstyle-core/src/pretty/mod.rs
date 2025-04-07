pub mod doc_ext;
pub mod style;

mod chain;
mod code_chain;
mod code_flow;
mod code_list;
mod comment;
mod flow;
mod func_call;
mod import;
mod list;
mod markup;
mod math;
mod mode;
mod parened_expr;
mod plain;
mod table;
mod util;

pub use mode::Mode;

use std::cell::{Cell, RefCell};

use itertools::Itertools;
use pretty::{Arena, DocAllocator, DocBuilder};
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use crate::{ext::StrExt, AttrStore, Config};
use doc_ext::DocExt;
use style::FoldStyle;

pub type ArenaDoc<'a> = DocBuilder<'a, Arena<'a>>;

pub struct PrettyPrinter<'a> {
    config: Config,
    attr_store: AttrStore,
    mode: RefCell<Vec<Mode>>,
    break_suppressed: Cell<bool>,
    arena: Arena<'a>,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(config: Config, attr_store: AttrStore) -> Self {
        Self {
            config,
            attr_store,
            mode: vec![].into(),
            break_suppressed: false.into(),
            arena: Arena::new(),
        }
    }

    fn get_fold_style(&self, node: impl AstNode<'a>) -> FoldStyle {
        self.get_fold_style_untyped(node.to_untyped())
    }

    fn get_fold_style_untyped(&self, node: &'a SyntaxNode) -> FoldStyle {
        if self.is_break_suppressed() {
            return if self.attr_store.is_multiline(node) {
                FoldStyle::Fit
            } else {
                FoldStyle::Always
            };
        }
        if self.attr_store.is_multiline_flavor(node) {
            FoldStyle::Never
        } else {
            FoldStyle::Fit
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    fn check_disabled(&'a self, node: &'a SyntaxNode) -> Option<ArenaDoc<'a>> {
        if self.attr_store.is_format_disabled(node) {
            Some(self.format_disabled(node))
        } else {
            None
        }
    }

    fn format_disabled(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.arena.text(node.clone().into_text().to_string())
    }

    /// For leaf only.
    fn convert_verbatim(&'a self, node: impl AstNode<'a>) -> ArenaDoc<'a> {
        self.convert_verbatim_untyped(node.to_untyped())
    }

    /// For leaf only.
    fn convert_verbatim_untyped(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.arena.text(node.text().as_str())
    }

    pub fn convert_expr(&'a self, expr: Expr<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(expr.to_untyped()) {
            return res;
        }
        self.convert_expr_impl(expr)
    }

    fn convert_expr_impl(&'a self, expr: Expr<'a>) -> ArenaDoc<'a> {
        match expr {
            Expr::Text(t) => self.convert_text(t),
            Expr::Space(s) => self.convert_space(s),
            Expr::Linebreak(b) => self.convert_verbatim(b),
            Expr::Parbreak(b) => self.convert_parbreak(b),
            Expr::Escape(e) => self.convert_verbatim(e),
            Expr::Shorthand(s) => self.convert_verbatim(s),
            Expr::SmartQuote(s) => self.convert_verbatim(s),
            Expr::Strong(s) => self.convert_strong(s),
            Expr::Emph(e) => self.convert_emph(e),
            Expr::Raw(r) => self.convert_raw(r),
            Expr::Link(l) => self.convert_verbatim(l),
            Expr::Label(l) => self.convert_verbatim(l),
            Expr::Ref(r) => self.convert_ref(r),
            Expr::Heading(h) => self.convert_heading(h),
            Expr::List(l) => self.convert_list_item(l),
            Expr::Enum(e) => self.convert_enum_item(e),
            Expr::Term(t) => self.convert_term_item(t),
            Expr::Equation(e) => self.convert_equation(e),
            Expr::Math(m) => self.convert_math(m),
            Expr::MathText(math_text) => self.convert_trivia(math_text),
            Expr::MathIdent(mi) => self.convert_verbatim(mi),
            Expr::MathAlignPoint(map) => self.convert_verbatim(map),
            Expr::MathDelimited(md) => self.convert_math_delimited(md),
            Expr::MathAttach(ma) => self.convert_math_attach(ma),
            Expr::MathPrimes(mp) => self.convert_math_primes(mp),
            Expr::MathFrac(mf) => self.convert_math_frac(mf),
            Expr::MathRoot(mr) => self.convert_math_root(mr),
            Expr::MathShorthand(ms) => self.convert_verbatim(ms),
            Expr::Ident(i) => self.convert_ident(i),
            Expr::None(_) => self.convert_literal("none"),
            Expr::Auto(_) => self.convert_literal("auto"),
            Expr::Bool(b) => self.convert_verbatim(b),
            Expr::Int(i) => self.convert_verbatim(i),
            Expr::Float(f) => self.convert_verbatim(f),
            Expr::Numeric(n) => self.convert_verbatim(n),
            Expr::Str(s) => self.convert_verbatim(s),
            Expr::Code(c) => self.convert_code_block(c),
            Expr::Content(c) => self.convert_content_block(c),
            Expr::Parenthesized(p) => self.convert_parenthesized(p),
            Expr::Array(a) => self.convert_array(a),
            Expr::Dict(d) => self.convert_dict(d),
            Expr::Unary(u) => self.convert_unary(u),
            Expr::Binary(b) => self.convert_binary(b),
            Expr::FieldAccess(fa) => self.convert_field_access(fa),
            Expr::FuncCall(fc) => self.convert_func_call(fc),
            Expr::Closure(c) => self.convert_closure(c),
            Expr::Let(l) => self.convert_let_binding(l),
            Expr::DestructAssign(da) => self.convert_destruct_assignment(da),
            Expr::Set(s) => self.convert_set_rule(s),
            Expr::Show(s) => self.convert_show_rule(s),
            Expr::Contextual(c) => self.convert_contextual(c),
            Expr::Conditional(c) => self.convert_conditional(c),
            Expr::While(w) => self.convert_while_loop(w),
            Expr::For(f) => self.convert_for_loop(f),
            Expr::Import(i) => self.convert_import(i),
            Expr::Include(i) => self.convert_include(i),
            Expr::Break(_) => self.convert_literal("break"),
            Expr::Continue(_) => self.convert_literal("continue"),
            Expr::Return(r) => self.convert_return(r),
        }
    }

    fn convert_literal(&'a self, literal: &'a str) -> ArenaDoc<'a> {
        self.arena.text(literal)
    }

    fn convert_trivia(&'a self, node: impl AstNode<'a>) -> ArenaDoc<'a> {
        trivia(&self.arena, node.to_untyped())
    }

    fn convert_trivia_untyped(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        trivia(&self.arena, node)
    }

    fn convert_text(&'a self, text: Text<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(text)
    }

    fn convert_space(&'a self, space: Space<'a>) -> ArenaDoc<'a> {
        let node = space.to_untyped();
        if node.text().has_linebreak() {
            self.arena.hardline()
        } else {
            self.arena.space()
        }
    }

    fn convert_parbreak(&'a self, parbreak: Parbreak<'a>) -> ArenaDoc<'a> {
        let newline_count = parbreak.to_untyped().text().count_linebreaks();
        self.arena.hardline().repeat_n(newline_count)
    }

    fn convert_raw(&'a self, raw: Raw<'a>) -> ArenaDoc<'a> {
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
                doc += self.convert_trivia(line);
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

    fn convert_ref(&'a self, reference: Ref<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("@") + self.arena.text(reference.target());
        if let Some(supplement) = reference.supplement() {
            doc += self.convert_content_block(supplement);
        }
        doc
    }

    fn convert_ident(&'a self, ident: Ident<'a>) -> ArenaDoc<'a> {
        self.convert_verbatim(ident)
    }

    fn convert_array_item(&'a self, array_item: ArrayItem<'a>) -> ArenaDoc<'a> {
        match array_item {
            ArrayItem::Pos(p) => self.convert_expr(p),
            ArrayItem::Spread(s) => self.convert_spread(s),
        }
    }

    fn convert_dict_item(&'a self, dict_item: DictItem<'a>) -> ArenaDoc<'a> {
        match dict_item {
            DictItem::Named(n) => self.convert_named(n),
            DictItem::Keyed(k) => self.convert_keyed(k),
            DictItem::Spread(s) => self.convert_spread(s),
        }
    }

    fn convert_param(&'a self, param: Param<'a>) -> ArenaDoc<'a> {
        match param {
            Param::Pos(p) => self.convert_pattern(p),
            Param::Named(n) => self.convert_named(n),
            Param::Spread(s) => self.convert_spread(s),
        }
    }

    pub fn convert_pattern(&'a self, pattern: Pattern<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(pattern.to_untyped()) {
            return res;
        }
        match pattern {
            Pattern::Normal(n) => self.convert_expr(n),
            Pattern::Placeholder(p) => self.convert_verbatim(p),
            Pattern::Destructuring(d) => self.convert_destructuring(d),
            Pattern::Parenthesized(p) => self.convert_parenthesized(p),
        }
    }

    fn convert_destructuring_item(
        &'a self,
        destructuring_item: DestructuringItem<'a>,
    ) -> ArenaDoc<'a> {
        match destructuring_item {
            DestructuringItem::Spread(s) => self.convert_spread(s),
            DestructuringItem::Named(n) => self.convert_named(n),
            DestructuringItem::Pattern(p) => self.convert_pattern(p),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StripMode {
    None,
    Prefix,
    PrefixOnBoundaryMarkers,
}

fn trivia<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> ArenaDoc<'a> {
    to_doc(arena, node.text(), StripMode::None)
}

fn trivia_strip_prefix<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> ArenaDoc<'a> {
    to_doc(arena, node.text(), StripMode::Prefix)
}

pub fn to_doc<'a>(arena: &'a Arena<'a>, s: &'a str, strip_prefix: StripMode) -> ArenaDoc<'a> {
    let get_line = |i: itertools::Position, line: &'a str| -> &'a str {
        let should_trim = matches!(strip_prefix, StripMode::Prefix)
            || (matches!(strip_prefix, StripMode::PrefixOnBoundaryMarkers)
                && matches!(
                    i,
                    itertools::Position::First
                        | itertools::Position::Last
                        | itertools::Position::Only
                ));

        if should_trim {
            line.trim_start()
        } else {
            line
        }
    };
    // String::lines() doesn't include the trailing newline
    let has_trailing_newline = s.ends_with('\n');
    let res = arena.intersperse(
        s.lines()
            .with_position()
            .map(|(i, s)| arena.text(get_line(i, s))),
        arena.hardline(),
    );
    if has_trailing_newline {
        res + arena.hardline()
    } else {
        res
    }
}
