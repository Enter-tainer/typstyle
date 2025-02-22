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
mod mode;
mod parened_expr;
mod plain;
mod table;
mod util;

pub use mode::Mode;

use std::cell::RefCell;

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
    arena: Arena<'a>,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(config: Config, attr_store: AttrStore) -> Self {
        Self {
            config,
            attr_store,
            mode: vec![].into(),
            arena: Arena::new(),
        }
    }

    fn get_fold_style(&self, node: impl AstNode<'a>) -> FoldStyle {
        self.get_fold_style_untyped(node.to_untyped())
    }

    fn get_fold_style_untyped(&self, node: &'a SyntaxNode) -> FoldStyle {
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

    #[allow(dead_code)]
    fn check_unformattable(&'a self, node: &'a SyntaxNode) -> Option<ArenaDoc<'a>> {
        if self.attr_store.is_unformattable(node) {
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
        if self.current_mode().is_math() {
            if let Some(res) = self.check_unformattable(expr.to_untyped()) {
                return res;
            }
        }
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
            Expr::MathIdent(mi) => self.convert_verbatim(mi),
            Expr::MathAlignPoint(map) => self.convert_verbatim(map),
            Expr::MathDelimited(md) => self.convert_math_delimited(md),
            Expr::MathAttach(ma) => self.convert_math_attach(ma),
            Expr::MathPrimes(mp) => self.convert_math_primes(mp),
            Expr::MathFrac(mf) => self.convert_math_frac(mf),
            Expr::MathRoot(mr) => self.convert_math_root(mr),
            Expr::MathShorthand(ms) => self.convert_verbatim(ms),
            Expr::Ident(i) => self.convert_ident(i),
            Expr::None(n) => self.convert_verbatim(n),
            Expr::Auto(a) => self.convert_verbatim(a),
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
            Expr::Conditional(c) => self.convert_conditional(c),
            Expr::While(w) => self.convert_while_loop(w),
            Expr::For(f) => self.convert_for_loop(f),
            Expr::Import(i) => self.convert_import(i),
            Expr::Include(i) => self.convert_include(i),
            Expr::Break(b) => self.convert_break(b),
            Expr::Continue(c) => self.convert_continue(c),
            Expr::Return(r) => self.convert_return(r),
            Expr::Contextual(c) => self.convert_contextual(c),
            Expr::MathText(math_text) => self.convert_trivia(math_text),
        }
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

    fn convert_equation(&'a self, equation: Equation<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_unformattable(equation.to_untyped()) {
            return res;
        }

        let _g = self.with_mode(Mode::Math);
        let body = self.convert_math(equation.body());
        let doc = if equation.block() {
            let is_multi_line = self.attr_store.is_multiline(equation.to_untyped());
            if is_multi_line {
                (self.arena.hardline() + body).nest(self.config.tab_spaces as isize)
                    + self.arena.hardline()
            } else {
                ((self.arena.line() + body).nest(self.config.tab_spaces as isize)
                    + self.arena.line())
                .group()
            }
        } else {
            body.nest(self.config.tab_spaces as isize)
        };
        doc.enclose("$", "$")
    }

    fn convert_math(&'a self, math: Math<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(math.to_untyped()) {
            return res;
        }
        let mut doc = self.arena.nil();
        for node in math.to_untyped().children() {
            if let Some(expr) = node.cast::<Expr>() {
                let expr_doc = self.convert_expr(expr);
                doc += expr_doc;
            } else if let Some(space) = node.cast::<Space>() {
                doc += self.convert_space(space);
            } else {
                doc += self.convert_trivia_untyped(node);
            }
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

    fn convert_break(&'a self, _break: LoopBreak<'a>) -> ArenaDoc<'a> {
        self.arena.text("break")
    }

    fn convert_continue(&'a self, _continue: LoopContinue<'a>) -> ArenaDoc<'a> {
        self.arena.text("continue")
    }

    fn convert_math_delimited(&'a self, math_delimited: MathDelimited<'a>) -> ArenaDoc<'a> {
        fn has_spaces(math_delimited: MathDelimited<'_>) -> (bool, bool) {
            let mut has_space_before_math = false;
            let mut has_space_after_math = false;
            let mut is_before_math = true;
            for child in math_delimited.to_untyped().children() {
                if child.kind() == SyntaxKind::Math {
                    is_before_math = false;
                } else if child.kind() == SyntaxKind::Space {
                    if is_before_math {
                        has_space_before_math = true;
                    } else {
                        has_space_after_math = true;
                    }
                }
            }
            (has_space_before_math, has_space_after_math)
        }
        let open = self.convert_expr(math_delimited.open());
        let close = self.convert_expr(math_delimited.close());
        let body = self.convert_math(math_delimited.body());
        let (has_space_before_math, has_space_after_math) = has_spaces(math_delimited);

        body.enclose(
            if has_space_before_math {
                self.arena.space()
            } else {
                self.arena.nil()
            },
            if has_space_after_math {
                self.arena.space()
            } else {
                self.arena.nil()
            },
        )
        .nest(self.config.tab_spaces as isize)
        .enclose(open, close)
    }

    fn convert_math_attach(&'a self, math_attach: MathAttach<'a>) -> ArenaDoc<'a> {
        let mut doc = self.convert_expr(math_attach.base());
        let prime_index = math_attach
            .to_untyped()
            .children()
            .enumerate()
            .skip_while(|(_i, node)| node.cast::<Expr<'_>>().is_none())
            .nth(1)
            .filter(|(_i, n)| n.cast::<MathPrimes>().is_some())
            .map(|(i, _n)| i);

        let bottom_index = math_attach
            .to_untyped()
            .children()
            .enumerate()
            .skip_while(|(_i, node)| !matches!(node.kind(), SyntaxKind::Underscore))
            .find_map(|(i, n)| SyntaxNode::cast::<Expr<'_>>(n).map(|n| (i, n)))
            .map(|(i, _n)| i);

        let top_index = math_attach
            .to_untyped()
            .children()
            .enumerate()
            .skip_while(|(_i, node)| !matches!(node.kind(), SyntaxKind::Hat))
            .find_map(|(i, n)| SyntaxNode::cast::<Expr<'_>>(n).map(|n| (i, n)))
            .map(|(i, _n)| i);

        #[derive(Debug)]
        enum IndexType {
            Prime,
            Bottom,
            Top,
        }

        let mut index_types = [IndexType::Prime, IndexType::Bottom, IndexType::Top];
        index_types.sort_by_key(|index_type| match index_type {
            IndexType::Prime => prime_index,
            IndexType::Bottom => bottom_index,
            IndexType::Top => top_index,
        });

        for index in index_types {
            match index {
                IndexType::Prime => {
                    if let Some(primes) = math_attach.primes() {
                        doc += self.convert_math_primes(primes);
                    }
                }
                IndexType::Bottom => {
                    if let Some(bottom) = math_attach.bottom() {
                        doc += self.arena.text("_") + self.convert_expr(bottom);
                    }
                }
                IndexType::Top => {
                    if let Some(top) = math_attach.top() {
                        doc += self.arena.text("^") + self.convert_expr(top);
                    }
                }
            }
        }
        doc
    }

    fn convert_math_primes(&'a self, math_primes: MathPrimes<'a>) -> ArenaDoc<'a> {
        self.arena.text("'".repeat(math_primes.count()))
    }

    fn convert_math_frac(&'a self, math_frac: MathFrac<'a>) -> ArenaDoc<'a> {
        let singleline = self.convert_expr(math_frac.num())
            + self.arena.space()
            + self.arena.text("/")
            + self.arena.space()
            + self.convert_expr(math_frac.denom());
        // TODO: add multiline version
        singleline
    }

    fn convert_math_root(&'a self, math_root: MathRoot<'a>) -> ArenaDoc<'a> {
        let sqrt_sym = if let Some(index) = math_root.index() {
            if index == 3 {
                "∛"
            } else if index == 4 {
                "∜"
            } else {
                // TODO: actually unreachable
                "√"
            }
        } else {
            "√"
        };
        self.arena.text(sqrt_sym) + self.convert_expr(math_root.radicand())
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
