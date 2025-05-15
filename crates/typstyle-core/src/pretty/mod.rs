pub mod context;
pub mod style;

mod code_chain;
mod code_flow;
mod code_list;
mod code_misc;
mod comment;
mod func_call;
mod import;
mod layout;
mod markup;
mod math;
mod math_align;
mod parened_expr;
mod table;
mod text;
mod util;

pub use context::{Context, Mode};
use pretty::{Arena, DocAllocator, DocBuilder};
use style::FoldStyle;
use typst_syntax::{ast::*, SyntaxNode};

use crate::{AttrStore, Config};

pub type ArenaDoc<'a> = DocBuilder<'a, Arena<'a>>;

pub struct PrettyPrinter<'a> {
    config: Config,
    attr_store: AttrStore,
    arena: Arena<'a>,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(config: Config, attr_store: AttrStore) -> Self {
        Self {
            config,
            attr_store,
            arena: Arena::new(),
        }
    }

    fn get_fold_style(&self, ctx: Context, node: impl AstNode<'a>) -> FoldStyle {
        self.get_fold_style_untyped(ctx, node.to_untyped())
    }

    fn get_fold_style_untyped(&self, ctx: Context, node: &'a SyntaxNode) -> FoldStyle {
        if ctx.break_suppressed {
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
            Some(self.convert_verbatim_untyped(node))
        } else {
            None
        }
    }

    /// For inner or lead nodes.
    fn convert_verbatim(&'a self, node: impl AstNode<'a>) -> ArenaDoc<'a> {
        self.convert_verbatim_untyped(node.to_untyped())
    }

    /// For inner or lead nodes.
    fn convert_verbatim_untyped(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.arena.text(node.clone().into_text().to_string())
    }

    /// For leaf only.
    fn convert_trivia(&'a self, node: impl AstNode<'a>) -> ArenaDoc<'a> {
        self.convert_trivia_untyped(node.to_untyped())
    }

    /// For leaf only.
    fn convert_trivia_untyped(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.arena.text(node.text().as_str())
    }

    pub fn convert_expr(&'a self, ctx: Context, expr: Expr<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(expr.to_untyped()) {
            return res;
        }
        self.convert_expr_impl(ctx, expr)
    }

    fn convert_expr_impl(&'a self, ctx: Context, expr: Expr<'a>) -> ArenaDoc<'a> {
        match expr {
            Expr::Text(t) => self.convert_text(t),
            Expr::Space(s) => self.convert_space(s),
            Expr::Linebreak(b) => self.convert_trivia(b),
            Expr::Parbreak(b) => self.convert_parbreak(b),
            Expr::Escape(e) => self.convert_trivia(e),
            Expr::Shorthand(s) => self.convert_trivia(s),
            Expr::SmartQuote(s) => self.convert_trivia(s),
            Expr::Strong(s) => self.convert_strong(ctx, s),
            Expr::Emph(e) => self.convert_emph(ctx, e),
            Expr::Raw(r) => self.convert_raw(ctx, r),
            Expr::Link(l) => self.convert_trivia(l),
            Expr::Label(l) => self.convert_trivia(l),
            Expr::Ref(r) => self.convert_ref(ctx, r),
            Expr::Heading(h) => self.convert_heading(ctx, h),
            Expr::List(l) => self.convert_list_item(ctx, l),
            Expr::Enum(e) => self.convert_enum_item(ctx, e),
            Expr::Term(t) => self.convert_term_item(ctx, t),
            Expr::Equation(e) => self.convert_equation(ctx, e),
            Expr::Math(m) => self.convert_math(ctx, m),
            Expr::MathText(math_text) => self.convert_trivia(math_text),
            Expr::MathIdent(mi) => self.convert_trivia(mi),
            Expr::MathAlignPoint(map) => self.convert_trivia(map),
            Expr::MathDelimited(md) => self.convert_math_delimited(ctx, md),
            Expr::MathAttach(ma) => self.convert_math_attach(ctx, ma),
            Expr::MathPrimes(mp) => self.convert_math_primes(ctx, mp),
            Expr::MathFrac(mf) => self.convert_math_frac(ctx, mf),
            Expr::MathRoot(mr) => self.convert_math_root(ctx, mr),
            Expr::MathShorthand(ms) => self.convert_trivia(ms),
            Expr::Ident(i) => self.convert_ident(i),
            Expr::None(_) => self.convert_literal("none"),
            Expr::Auto(_) => self.convert_literal("auto"),
            Expr::Bool(b) => self.convert_trivia(b),
            Expr::Int(i) => self.convert_trivia(i),
            Expr::Float(f) => self.convert_trivia(f),
            Expr::Numeric(n) => self.convert_trivia(n),
            Expr::Str(s) => self.convert_trivia(s),
            Expr::Code(c) => self.convert_code_block(ctx, c),
            Expr::Content(c) => self.convert_content_block(ctx, c),
            Expr::Parenthesized(p) => self.convert_parenthesized(ctx, p),
            Expr::Array(a) => self.convert_array(ctx, a),
            Expr::Dict(d) => self.convert_dict(ctx, d),
            Expr::Unary(u) => self.convert_unary(ctx, u),
            Expr::Binary(b) => self.convert_binary(ctx, b),
            Expr::FieldAccess(fa) => self.convert_field_access(ctx, fa),
            Expr::FuncCall(fc) => self.convert_func_call(ctx, fc),
            Expr::Closure(c) => self.convert_closure(ctx, c),
            Expr::Let(l) => self.convert_let_binding(ctx, l),
            Expr::DestructAssign(da) => self.convert_destruct_assignment(ctx, da),
            Expr::Set(s) => self.convert_set_rule(ctx, s),
            Expr::Show(s) => self.convert_show_rule(ctx, s),
            Expr::Contextual(c) => self.convert_contextual(ctx, c),
            Expr::Conditional(c) => self.convert_conditional(ctx, c),
            Expr::While(w) => self.convert_while_loop(ctx, w),
            Expr::For(f) => self.convert_for_loop(ctx, f),
            Expr::Import(i) => self.convert_import(ctx, i),
            Expr::Include(i) => self.convert_include(ctx, i),
            Expr::Break(_) => self.convert_literal("break"),
            Expr::Continue(_) => self.convert_literal("continue"),
            Expr::Return(r) => self.convert_return(ctx, r),
        }
    }

    fn convert_literal(&'a self, literal: &'a str) -> ArenaDoc<'a> {
        self.arena.text(literal)
    }
}
