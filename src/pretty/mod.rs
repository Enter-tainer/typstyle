pub mod config;
pub mod style;

mod arg;
mod comment;
mod dot_chain;
mod func_call;
mod items;
mod list;
mod mode;
mod parened_expr;
mod table;
mod util;

use std::cell::RefCell;

use arg::ArgStylist;
use config::PrinterConfig;
use items::{comma_separated_items, pretty_items};
use itertools::Itertools;
use list::ListStylist;
use mode::Mode;
use parened_expr::optional_paren;
use pretty::{Arena, DocAllocator, DocBuilder};
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use util::is_comment_node;

use crate::AttrStore;
use style::FoldStyle;

type ArenaDoc<'a> = DocBuilder<'a, Arena<'a>>;

#[derive(Default)]
pub struct PrettyPrinter<'a> {
    config: PrinterConfig,
    attr_store: AttrStore,
    mode: RefCell<Vec<Mode>>,
    arena: Arena<'a>,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(attr_store: AttrStore) -> Self {
        Self {
            config: Default::default(),
            attr_store,
            mode: vec![].into(),
            arena: Arena::new(),
        }
    }

    fn get_fold_style(&self, node: impl AstNode<'a>) -> FoldStyle {
        self.get_fold_style_untyped(node.to_untyped())
    }

    fn get_fold_style_untyped(&self, node: &'a SyntaxNode) -> FoldStyle {
        if self.attr_store.is_node_multiline(node) {
            FoldStyle::Never
        } else {
            FoldStyle::Fit
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    pub fn convert_markup(&'a self, root: Markup<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Markup);
        let mut doc = self.arena.nil();
        #[derive(Debug, Default)]
        struct Line<'a> {
            has_text: bool,
            nodes: Vec<&'a SyntaxNode>,
        }
        // break markup into lines, split by stmt, parbreak, newline, multiline raw,
        // equation if a line contains text, it will be skipped by the formatter
        // to keep the original format
        let lines = {
            let mut lines: Vec<Line> = vec![];
            let mut current_line = Line {
                has_text: false,
                nodes: vec![],
            };
            for node in root.to_untyped().children() {
                let mut break_line = false;
                if let Some(space) = node.cast::<Space>() {
                    if space.to_untyped().text().contains('\n') {
                        break_line = true;
                    }
                } else if let Some(pb) = node.cast::<Parbreak>() {
                    if pb.to_untyped().text().contains('\n') {
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
        };
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
                if has_text {
                    doc += self.format_disabled(node);
                } else if let Some(expr) = node.cast::<Expr>() {
                    doc += self.convert_expr(expr);
                } else if is_comment_node(node) {
                    doc += self.convert_comment(node);
                } else {
                    doc += trivia_prefix(&self.arena, node);
                }
            }
        }
        doc
    }

    fn check_disabled(&'a self, node: &'a SyntaxNode) -> Option<ArenaDoc<'a>> {
        if self.attr_store.is_node_no_format(node) {
            Some(self.format_disabled(node))
        } else {
            None
        }
    }

    fn format_disabled(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        return self.arena.text(node.clone().into_text().to_string());
    }

    fn convert_expr(&'a self, expr: Expr<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(expr.to_untyped()) {
            return res;
        }
        match expr {
            Expr::Text(t) => self.convert_text(t),
            Expr::Space(s) => self.convert_space(s),
            Expr::Linebreak(b) => self.convert_linebreak(b),
            Expr::Parbreak(b) => self.convert_parbreak(b),
            Expr::Escape(e) => self.convert_escape(e),
            Expr::Shorthand(s) => self.convert_shorthand(s),
            Expr::SmartQuote(s) => self.convert_smart_quote(s),
            Expr::Strong(s) => self.convert_strong(s),
            Expr::Emph(e) => self.convert_emph(e),
            Expr::Raw(r) => self.convert_raw(r),
            Expr::Link(l) => self.convert_link(l),
            Expr::Label(l) => self.convert_label(l),
            Expr::Ref(r) => self.convert_ref(r),
            Expr::Heading(h) => self.convert_heading(h),
            Expr::List(l) => self.convert_list_item(l),
            Expr::Enum(e) => self.convert_enum_item(e),
            Expr::Term(t) => self.convert_term_item(t),
            Expr::Equation(e) => self.convert_equation(e),
            Expr::Math(m) => self.convert_math(m),
            Expr::MathIdent(mi) => self.convert_trivia(mi),
            Expr::MathAlignPoint(map) => self.convert_trivia(map),
            Expr::MathDelimited(md) => self.convert_math_delimited(md),
            Expr::MathAttach(ma) => self.convert_math_attach(ma),
            Expr::MathPrimes(mp) => self.convert_math_primes(mp),
            Expr::MathFrac(mf) => self.convert_math_frac(mf),
            Expr::MathRoot(mr) => self.convert_math_root(mr),
            Expr::MathShorthand(ms) => self.convert_trivia(ms),
            Expr::Ident(i) => self.convert_ident(i),
            Expr::None(n) => self.convert_none(n),
            Expr::Auto(a) => self.convert_auto(a),
            Expr::Bool(b) => self.convert_bool(b),
            Expr::Int(i) => self.convert_int(i),
            Expr::Float(f) => self.convert_float(f),
            Expr::Numeric(n) => self.convert_numeric(n),
            Expr::Str(s) => self.convert_str(s),
            Expr::Code(c) => self.convert_code_block(c),
            Expr::Content(c) => self.convert_content_block(c),
            Expr::Parenthesized(p) => self.convert_parenthesized(p, false),
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
            Expr::While(w) => self.convert_while(w),
            Expr::For(f) => self.convert_for(f),
            Expr::Import(i) => self.convert_import(i),
            Expr::Include(i) => self.convert_include(i),
            Expr::Break(b) => self.convert_break(b),
            Expr::Continue(c) => self.convert_continue(c),
            Expr::Return(r) => self.convert_return(r),
            Expr::Contextual(c) => self.convert_contextual(c),
        }
        .group()
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
        if node.text().contains('\n') {
            self.arena.hardline()
        } else {
            self.arena.space()
        }
    }

    fn convert_linebreak(&'a self, linebreak: Linebreak<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(linebreak)
    }

    fn convert_parbreak(&'a self, parbreak: Parbreak<'a>) -> ArenaDoc<'a> {
        let newline_count = parbreak
            .to_untyped()
            .text()
            .chars()
            .filter(|c| *c == '\n')
            .count();
        self.arena
            .concat(std::iter::repeat_n(self.arena.hardline(), newline_count))
    }

    fn convert_escape(&'a self, escape: Escape<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(escape)
    }

    fn convert_shorthand(&'a self, shorthand: Shorthand<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(shorthand)
    }

    fn convert_smart_quote(&'a self, smart_quote: SmartQuote<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(smart_quote)
    }

    fn convert_strong(&'a self, strong: Strong<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup(strong.body());
        body.enclose("*", "*")
    }

    fn convert_emph(&'a self, emph: Emph<'a>) -> ArenaDoc<'a> {
        let body = self.convert_markup(emph.body());
        body.enclose("_", "_")
    }

    fn convert_raw(&'a self, raw: Raw<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.nil();
        for child in raw.to_untyped().children() {
            if let Some(delim) = child.cast::<RawDelim>() {
                doc += self.convert_trivia(delim);
            } else if let Some(lang) = child.cast::<RawLang>() {
                doc += self.convert_trivia(lang);
            } else if let Some(line) = child.cast::<Text>() {
                doc += self.convert_trivia(line);
            } else if child.kind() == SyntaxKind::RawTrimmed {
                if child.text().contains('\n') {
                    doc += self.arena.hardline();
                } else {
                    doc += self.arena.space();
                }
            }
        }
        doc
    }

    fn convert_link(&'a self, link: Link<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(link)
    }

    fn convert_label(&'a self, label: Label<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(label)
    }

    fn convert_ref(&'a self, reference: Ref<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("@") + self.arena.text(reference.target());
        if let Some(supplement) = reference.supplement() {
            doc += self.convert_content_block(supplement);
        }
        doc
    }

    fn convert_heading(&'a self, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.arena.text("=".repeat(heading.depth().into()))
            + self.arena.space()
            + self.convert_markup(heading.body())
    }

    fn convert_list_item(&'a self, list_item: ListItem<'a>) -> ArenaDoc<'a> {
        self.arena.text("-") + self.arena.space() + self.convert_markup(list_item.body()).nest(2)
    }

    fn convert_enum_item(&'a self, enum_item: EnumItem<'a>) -> ArenaDoc<'a> {
        let doc = if let Some(number) = enum_item.number() {
            self.arena.text(format!("{number}."))
        } else {
            self.arena.text("+")
        };
        doc + self.arena.space() + self.convert_markup(enum_item.body()).nest(2)
    }

    fn convert_term_item(&'a self, term: TermItem<'a>) -> ArenaDoc<'a> {
        self.arena.text("/")
            + self.arena.space()
            + self.convert_markup(term.term())
            + self.arena.text(":")
            + self.arena.space()
            + self.convert_markup(term.description()).nest(2)
    }

    fn convert_equation(&'a self, equation: Equation<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Math);
        let mut doc = self.convert_math(equation.body());
        if equation.block() {
            let is_multi_line = self.attr_store.is_node_multiline(equation.to_untyped());
            let block_sep = if is_multi_line {
                self.arena.hardline()
            } else {
                self.arena.line()
            };
            doc = (block_sep.clone() + doc).nest(2) + block_sep;
        } else {
            doc = doc.nest(2);
        }
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
        self.arena.text(ident.as_str())
    }

    fn convert_none(&'a self, _none: None<'a>) -> ArenaDoc<'a> {
        self.arena.text("none")
    }

    fn convert_auto(&'a self, _auto: Auto<'a>) -> ArenaDoc<'a> {
        self.arena.text("auto")
    }

    fn convert_bool(&'a self, boolean: Bool<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(boolean)
    }

    fn convert_int(&'a self, int: Int<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(int)
    }

    fn convert_float(&'a self, float: Float<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(float)
    }

    fn convert_numeric(&'a self, numeric: Numeric<'a>) -> ArenaDoc<'a> {
        self.convert_trivia(numeric)
    }

    fn convert_str(&'a self, str: Str<'a>) -> ArenaDoc<'a> {
        let node = str.to_untyped();
        if node.text().contains('\n') {
            self.arena.text(node.text().as_str())
        } else {
            self.convert_trivia_untyped(node)
        }
    }

    fn convert_code_block(&'a self, code_block: CodeBlock<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Code);
        let mut code_nodes = vec![];
        let mut has_comment = false;
        for node in code_block.to_untyped().children() {
            if let Some(code) = node.cast::<Code>() {
                code_nodes.extend(code.to_untyped().children());
            } else if node.kind() == SyntaxKind::Space {
                code_nodes.push(node);
            } else if is_comment_node(node) {
                code_nodes.push(node);
                has_comment = true;
            }
        }
        let codes = self.convert_code(code_nodes);
        let doc = pretty_items(
            &self.arena,
            &codes,
            self.arena.text(";") + self.arena.space(),
            self.arena.nil(),
            (self.arena.text("{"), self.arena.text("}")),
            true,
            if codes.len() == 1 && !has_comment {
                self.get_fold_style(code_block)
            } else {
                FoldStyle::Never
            },
        );
        doc
    }

    fn convert_code(&'a self, code: Vec<&'a SyntaxNode>) -> Vec<ArenaDoc<'a>> {
        let mut code = &code[..];

        // Strip trailing empty lines
        while (code.last()).is_some_and(|last| last.kind() == SyntaxKind::Space) {
            code = &code[..code.len() - 1];
        }

        let mut codes: Vec<_> = vec![];
        let mut can_attach_comment = false; // Whether a comment can follow the next node.
        for node in code {
            if let Some(expr) = node.cast::<Expr>() {
                let expr_doc = self.convert_expr(expr);
                codes.push(expr_doc);
                can_attach_comment = true;
            } else if is_comment_node(node) {
                if can_attach_comment {
                    let last = codes.pop().unwrap();
                    codes.push(last + self.arena.space() + self.convert_comment(node));
                } else {
                    codes.push(self.convert_comment(node));
                }
                can_attach_comment = false;
            } else if node.kind() == SyntaxKind::Space {
                let newline_cnt = node.text().chars().filter(|c| *c == '\n').count();
                if newline_cnt > 0 {
                    // Ensures no leading empty line.
                    if !codes.is_empty() {
                        codes.extend(std::iter::repeat_n(
                            self.arena.nil(),
                            (newline_cnt - 1).min(self.config.blank_lines_upper_bound),
                        ));
                    }
                    can_attach_comment = false;
                }
            }
        }

        codes
    }

    fn convert_content_block(&'a self, content_block: ContentBlock<'a>) -> ArenaDoc<'a> {
        let content = self.convert_markup(content_block.body()).group().nest(2);
        content.brackets()
    }

    fn convert_array(&'a self, array: Array<'a>) -> ArenaDoc<'a> {
        ListStylist::new(self).convert_array(array)
    }

    fn convert_array_item(&'a self, array_item: ArrayItem<'a>) -> ArenaDoc<'a> {
        match array_item {
            ArrayItem::Pos(p) => self.convert_expr(p),
            ArrayItem::Spread(s) => self.convert_spread(s),
        }
    }

    fn convert_dict(&'a self, dict: Dict<'a>) -> ArenaDoc<'a> {
        ListStylist::new(self).convert_dict(dict)
    }

    fn convert_dict_item(&'a self, dict_item: DictItem<'a>) -> ArenaDoc<'a> {
        match dict_item {
            DictItem::Named(n) => self.convert_named(n),
            DictItem::Keyed(k) => self.convert_keyed(k),
            DictItem::Spread(s) => self.convert_spread(s),
        }
    }

    fn convert_named(&'a self, named: Named<'a>) -> ArenaDoc<'a> {
        ArgStylist::new(self).convert_named(named)
    }

    fn convert_keyed(&'a self, keyed: Keyed<'a>) -> ArenaDoc<'a> {
        ArgStylist::new(self).convert_keyed(keyed)
    }

    fn convert_unary(&'a self, unary: Unary<'a>) -> ArenaDoc<'a> {
        let op_text = match unary.op() {
            UnOp::Pos => "+",
            UnOp::Neg => "-",
            UnOp::Not => "not ",
        };
        self.arena.text(op_text) + self.convert_expr(unary.expr())
    }

    fn convert_binary(&'a self, binary: Binary<'a>) -> ArenaDoc<'a> {
        self.convert_expr(binary.lhs())
            + self.arena.space()
            + self.arena.text(binary.op().as_str())
            + self.arena.space()
            + self.convert_expr(binary.rhs())
    }

    fn convert_field_access(&'a self, field_access: FieldAccess<'a>) -> ArenaDoc<'a> {
        let chain = self.resolve_dot_chain(field_access);
        if chain.is_none() || matches!(self.current_mode(), Mode::Markup | Mode::Math) {
            let left = self.convert_expr(field_access.target());
            let singleline_right = self.arena.text(".") + self.convert_ident(field_access.field());
            return left + singleline_right;
        }
        let mut chain = chain.unwrap();
        if chain.len() == 2 {
            let last = chain.pop().unwrap();
            let first = chain.pop().unwrap();
            return first + self.arena.text(".") + last;
        }
        let first_doc = chain.remove(0);
        let other_doc = self
            .arena
            .intersperse(chain, self.arena.line_() + self.arena.text("."));
        let chain = first_doc
            + (self.arena.line_() + self.arena.text(".") + other_doc)
                .nest(2)
                .group();
        // if matches!(self.current_mode(), Mode::Markup | Mode::Math) {
        //     optional_paren(chain)
        // } else {
        //     chain
        // }
        chain
    }

    fn convert_closure(&'a self, closure: Closure<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.nil();
        let params = self.convert_params(closure.params());
        let style = self.get_fold_style(closure.params());
        let arg_list = if let Some(res) = self.check_disabled(closure.params().to_untyped()) {
            res
        } else {
            comma_separated_items(&self.arena, params.clone().into_iter(), style, None, None)
        };

        if let Some(name) = closure.name() {
            doc += self.convert_ident(name)
                + arg_list
                + self.arena.space()
                + self.arena.text("=")
                + self.arena.space()
                + self.convert_expr_with_optional_paren(closure.body());
        } else {
            if params.len() == 1
                && matches!(closure.params().children().next().unwrap(), Param::Pos(_))
                && !matches!(
                    closure.params().children().next().unwrap(),
                    Param::Pos(Pattern::Destructuring(_))
                )
            {
                doc = params[0].clone();
            } else {
                doc = arg_list
            }
            doc += self.arena.space()
                + self.arena.text("=>")
                + self.arena.space()
                + self.convert_expr_with_optional_paren(closure.body());
        }
        doc
    }

    fn convert_params(&'a self, params: Params<'a>) -> Vec<ArenaDoc<'a>> {
        params
            .children()
            .map(|param| self.convert_param(param))
            .collect()
    }

    fn convert_param(&'a self, param: Param<'a>) -> ArenaDoc<'a> {
        match param {
            Param::Pos(p) => self.convert_pattern(p),
            Param::Named(n) => self.convert_named(n),
            Param::Spread(s) => self.convert_spread(s),
        }
    }

    fn convert_spread(&'a self, spread: Spread<'a>) -> ArenaDoc<'a> {
        ArgStylist::new(self).convert_spread(spread)
    }

    fn convert_pattern(&'a self, pattern: Pattern<'a>) -> ArenaDoc<'a> {
        match pattern {
            Pattern::Normal(n) => self.convert_expr(n),
            Pattern::Placeholder(p) => self.convert_underscore(p),
            Pattern::Destructuring(d) => self.convert_destructuring(d),
            Pattern::Parenthesized(p) => self.convert_parenthesized(p, true),
        }
    }

    fn convert_underscore(&'a self, _underscore: Underscore<'a>) -> ArenaDoc<'a> {
        self.arena.text("_")
    }

    fn convert_destructuring(&'a self, destructuring: Destructuring<'a>) -> ArenaDoc<'a> {
        ListStylist::new(self).convert_destructuring(destructuring)
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

    fn convert_let_binding(&'a self, let_binding: LetBinding<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("let") + self.arena.space();
        match let_binding.kind() {
            LetBindingKind::Normal(n) => {
                doc += self.convert_pattern(n).group();
                if let Some(expr) = let_binding.init() {
                    doc += self.arena.space()
                        + self.arena.text("=")
                        + self.arena.space()
                        + self.convert_expr(expr);
                }
            }
            LetBindingKind::Closure(_c) => {
                if let Some(c) = let_binding.init() {
                    doc += self.convert_expr(c);
                }
            }
        }
        doc
    }

    fn convert_destruct_assignment(
        &'a self,
        destruct_assign: DestructAssignment<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_pattern(destruct_assign.pattern())
            + self.arena.space()
            + self.arena.text("=")
            + self.arena.space()
            + self.convert_expr(destruct_assign.value())
    }

    fn convert_set_rule(&'a self, set_rule: SetRule<'a>) -> ArenaDoc<'a> {
        let mut doc =
            self.arena.text("set") + self.arena.space() + self.convert_expr(set_rule.target());
        if let Some(res) = self.check_disabled(set_rule.args().to_untyped()) {
            doc += res;
        } else {
            doc += self.convert_parenthesized_args(set_rule.args());
        }
        if let Some(condition) = set_rule.condition() {
            doc += self.arena.space()
                + self.arena.text("if")
                + self.arena.space()
                + self.convert_expr(condition)
        }
        doc
    }

    fn convert_show_rule(&'a self, show_rule: ShowRule<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("show");
        if let Some(selector) = show_rule.selector() {
            doc += self.arena.space() + self.convert_expr(selector);
        }
        doc + self.arena.text(":") + self.arena.space() + self.convert_expr(show_rule.transform())
    }

    fn convert_conditional(&'a self, conditional: Conditional<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.nil();
        enum CastType {
            Condition,
            Then,
            Else,
        }
        let has_else = conditional.else_body().is_some();
        let mut expr_type = CastType::Condition;
        for child in conditional.to_untyped().children() {
            if child.kind() == SyntaxKind::If {
                doc += self.arena.text("if") + self.arena.space();
            } else if child.kind() == SyntaxKind::Else {
                doc += self.arena.text("else") + self.arena.space();
            } else if child.kind() == SyntaxKind::BlockComment {
                doc += self.convert_block_comment(child) + self.arena.space();
            } else if child.kind() == SyntaxKind::LineComment {
                doc += self.convert_line_comment(child) + self.arena.hardline();
            } else {
                match expr_type {
                    CastType::Condition => {
                        if let Some(condition) = child.cast() {
                            doc += self.convert_expr(condition) + self.arena.space();
                            expr_type = CastType::Then;
                        }
                    }
                    CastType::Then => {
                        if let Some(then_expr) = child.cast() {
                            doc += self.convert_expr(then_expr).group();
                            if has_else {
                                expr_type = CastType::Else;
                                doc += self.arena.space();
                            }
                        }
                    }
                    CastType::Else => {
                        if let Some(else_expr) = child.cast() {
                            doc += self.convert_expr(else_expr).group();
                        }
                    }
                }
            }
        }
        doc
    }

    fn convert_while(&'a self, while_loop: WhileLoop<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.nil();
        #[derive(Debug, PartialEq)]
        enum CastType {
            Condition,
            Body,
        }
        let mut expr_type = CastType::Condition;
        for child in while_loop.to_untyped().children() {
            if child.kind() == SyntaxKind::While {
                doc += self.arena.text("while") + self.arena.space();
            } else if child.kind() == SyntaxKind::BlockComment {
                doc += self.convert_block_comment(child) + self.arena.space();
            } else if child.kind() == SyntaxKind::LineComment {
                doc += self.convert_line_comment(child) + self.arena.hardline();
            } else if let Some(expr) = child.cast() {
                doc += self.convert_expr(expr);
                if expr_type == CastType::Condition {
                    doc += self.arena.space();
                    expr_type = CastType::Body;
                }
            }
        }
        doc
    }

    fn convert_for(&'a self, for_loop: ForLoop<'a>) -> ArenaDoc<'a> {
        let for_pattern = self.arena.text("for")
            + self.arena.space()
            + self.convert_pattern(for_loop.pattern())
            + self.arena.space();
        let in_iter = self.arena.text("in")
            + self.arena.space()
            // + self.arena.softline() // upstream issue: https://github.com/typst/typst/issues/4548
            + self.convert_expr_with_optional_paren(for_loop.iterable())
            + self.arena.space();
        let body = self.convert_expr(for_loop.body());
        (for_pattern + in_iter).group() + body
    }

    fn convert_import(&'a self, import: ModuleImport<'a>) -> ArenaDoc<'a> {
        let mut doc =
            self.arena.text("import") + self.arena.space() + self.convert_expr(import.source());
        if let Some(new_name) = import.new_name() {
            doc += self.arena.space()
                + self.arena.text("as")
                + self.arena.space()
                + self.convert_ident(new_name);
        }
        if let Some(imports) = import.imports() {
            doc += self.arena.text(":") + self.arena.space();
            let imports = match imports {
                Imports::Wildcard => self.arena.text("*"),
                Imports::Items(i) => {
                    let trailing_comma = self.arena.text(",").flat_alt(self.arena.nil());
                    let inner = self.arena.intersperse(
                        i.iter().map(|item| self.convert_import_item(item)),
                        self.arena.text(",") + self.arena.line(),
                    ) + trailing_comma;
                    optional_paren(&self.arena, inner)
                }
            };
            doc += imports.group();
        }
        doc
    }

    fn convert_import_item(&'a self, import_item: ImportItem<'a>) -> ArenaDoc<'a> {
        match import_item {
            ImportItem::Simple(s) => self.arena.intersperse(
                s.iter().map(|id| self.convert_ident(id)),
                self.arena.text("."),
            ),
            ImportItem::Renamed(r) => {
                self.convert_ident(r.original_name())
                    + self.arena.space()
                    + self.arena.text("as")
                    + self.arena.space()
                    + self.convert_ident(r.new_name())
            }
        }
    }

    fn convert_include(&'a self, include: ModuleInclude<'a>) -> ArenaDoc<'a> {
        self.arena.text("include") + self.arena.space() + self.convert_expr(include.source())
    }

    fn convert_break(&'a self, _break: LoopBreak<'a>) -> ArenaDoc<'a> {
        self.arena.text("break")
    }

    fn convert_continue(&'a self, _continue: LoopContinue<'a>) -> ArenaDoc<'a> {
        self.arena.text("continue")
    }

    fn convert_return(&'a self, return_stmt: FuncReturn<'a>) -> ArenaDoc<'a> {
        let mut doc = self.arena.text("return") + self.arena.space();
        if let Some(body) = return_stmt.body() {
            doc += self.convert_expr(body);
        }
        doc
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
        .nest(2)
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
                self.arena.text("∛")
            } else if index == 4 {
                self.arena.text("∜")
            } else {
                // TODO: actually unreachable
                self.arena.text("√")
            }
        } else {
            self.arena.text("√")
        };
        sqrt_sym + self.convert_expr(math_root.radicand())
    }

    fn convert_contextual(&'a self, ctx: Contextual<'a>) -> ArenaDoc<'a> {
        let body = self.convert_expr(ctx.body());
        self.arena.text("context") + self.arena.space() + body
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

fn trivia_prefix<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> ArenaDoc<'a> {
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

#[cfg(test)]
mod tests {
    use typst_syntax::parse;

    use super::*;

    #[test]
    fn test_to_doc() {
        let tests = [
            "command can take a directory as an argument to use as the book",
            "123\n456\n789",
            "123\n4567\n789\n",
            "123\n4568\n789\n",
        ];
        let arena = Arena::new();
        for test in tests.into_iter() {
            insta::assert_debug_snapshot!(to_doc(&arena, test, StripMode::None));
        }
    }

    #[test]
    fn convert_markup() {
        let tests = [r"=== --open

When you use the `--open` flag, typst-book will open the rendered book in
your default web browser after building it."];
        for test in tests.into_iter() {
            let root = parse(test);
            insta::assert_debug_snapshot!(root);
            let markup = root.cast().unwrap();
            let printer = PrettyPrinter::default();
            let doc = printer.convert_markup(markup);
            insta::assert_debug_snapshot!(doc.pretty(120).to_string());
        }
    }

    #[test]
    fn convert_func_call() {
        let tests = [r#"#link("http://example.com")[test]"#];
        for test in tests.into_iter() {
            let root = parse(test);
            insta::assert_debug_snapshot!(root);
            let markup = root.cast().unwrap();
            let printer = PrettyPrinter::default();
            let doc = printer.convert_markup(markup);
            insta::assert_debug_snapshot!(doc.pretty(120).to_string());
        }
    }
}
