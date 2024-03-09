pub mod util;

use std::borrow::Cow;
use std::cell::RefCell;

use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::{ast, SyntaxNode};
use typst_syntax::{ast::*, SyntaxKind};

use crate::util::pretty_items;

#[derive(Debug)]
pub struct PrettyPrinter {}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {}
    }
}

impl PrettyPrinter {
    pub fn convert_markup<'a>(&'a self, root: Markup<'a>) -> BoxDoc<'a, ()> {
        let mut doc: BoxDoc<()> = BoxDoc::nil();
        for node in root.to_untyped().children() {
            if let Some(expr) = node.cast::<Expr>() {
                let expr_doc = self.convert_expr(expr);
                doc = doc.append(expr_doc);
            } else if let Some(space) = node.cast::<Space>() {
                doc = doc.append(self.convert_space(space));
            } else {
                doc = doc.append(trivia(node));
            }
        }
        doc
    }

    fn convert_expr<'a>(&'a self, expr: Expr<'a>) -> BoxDoc<'a, ()> {
        match expr {
            ast::Expr::Text(t) => self.convert_text(t),
            ast::Expr::Space(s) => self.convert_space(s),
            ast::Expr::Linebreak(b) => self.convert_linebreak(b),
            ast::Expr::Parbreak(b) => self.convert_parbreak(b),
            ast::Expr::Escape(e) => self.convert_escape(e),
            ast::Expr::Shorthand(s) => self.convert_shorthand(s),
            ast::Expr::SmartQuote(s) => self.convert_smart_quote(s),
            ast::Expr::Strong(s) => self.convert_strong(s),
            ast::Expr::Emph(e) => self.convert_emph(e),
            ast::Expr::Raw(r) => self.convert_raw(r),
            ast::Expr::Link(l) => self.convert_link(l),
            ast::Expr::Label(l) => self.convert_label(l),
            ast::Expr::Ref(r) => self.convert_ref(r),
            ast::Expr::Heading(h) => self.convert_heading(h),
            ast::Expr::List(l) => self.convert_list_item(l),
            ast::Expr::Enum(e) => self.convert_enum_item(e),
            ast::Expr::Term(t) => self.convert_term_item(t),
            ast::Expr::Equation(e) => self.convert_equation(e),
            ast::Expr::Math(m) => self.convert_math(m),
            ast::Expr::MathIdent(mi) => todo!(),
            ast::Expr::MathAlignPoint(map) => todo!(),
            ast::Expr::MathDelimited(md) => todo!(),
            ast::Expr::MathAttach(ma) => todo!(),
            ast::Expr::MathPrimes(mp) => todo!(),
            ast::Expr::MathFrac(mf) => todo!(),
            ast::Expr::MathRoot(mr) => todo!(),
            ast::Expr::Ident(i) => self.convert_ident(i),
            ast::Expr::None(n) => self.convert_none(n),
            ast::Expr::Auto(a) => self.convert_auto(a),
            ast::Expr::Bool(b) => self.convert_bool(b),
            ast::Expr::Int(i) => self.convert_int(i),
            ast::Expr::Float(f) => self.convert_float(f),
            ast::Expr::Numeric(n) => self.convert_numeric(n),
            ast::Expr::Str(s) => self.convert_str(s),
            ast::Expr::Code(c) => self.convert_code_block(c),
            ast::Expr::Content(c) => self.convert_content_block(c),
            ast::Expr::Parenthesized(p) => self.convert_parenthesized(p),
            ast::Expr::Array(a) => self.convert_array(a),
            ast::Expr::Dict(d) => self.convert_dict(d),
            ast::Expr::Unary(u) => self.convert_unary(u),
            ast::Expr::Binary(b) => self.convert_binary(b),
            ast::Expr::FieldAccess(fa) => self.convert_field_access(fa),
            ast::Expr::FuncCall(fc) => self.convert_func_call(fc),
            ast::Expr::Closure(c) => self.convert_closure(c),
            ast::Expr::Let(l) => self.convert_let_binding(l),
            ast::Expr::DestructAssign(da) => self.convert_destruct_assignment(da),
            ast::Expr::Set(s) => self.convert_set_rule(s),
            ast::Expr::Show(s) => self.convert_show_rule(s),
            ast::Expr::Conditional(c) => self.convert_conditional(c),
            ast::Expr::While(w) => self.convert_while(w),
            ast::Expr::For(f) => self.convert_for(f),
            ast::Expr::Import(i) => self.convert_import(i),
            ast::Expr::Include(i) => self.convert_include(i),
            ast::Expr::Break(b) => self.convert_break(b),
            ast::Expr::Continue(c) => self.convert_continue(c),
            ast::Expr::Return(r) => self.convert_return(r),
        }
        .group()
    }

    fn convert_text<'a>(&'a self, text: Text<'a>) -> BoxDoc<'a, ()> {
        let node = text.to_untyped();
        trivia(node)
    }

    fn convert_space<'a>(&'a self, space: Space<'a>) -> BoxDoc<'a, ()> {
        let node = space.to_untyped();
        if node.text().contains('\n') {
            BoxDoc::hardline()
        } else {
            BoxDoc::space()
        }
    }

    fn convert_linebreak<'a>(&'a self, linebreak: Linebreak<'a>) -> BoxDoc<'a, ()> {
        let node = linebreak.to_untyped();
        trivia(node)
    }

    fn convert_parbreak<'a>(&'a self, _parbreak: Parbreak<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::hardline().append(BoxDoc::hardline())
    }

    fn convert_escape<'a>(&'a self, escape: Escape<'a>) -> BoxDoc<'a, ()> {
        let node = escape.to_untyped();
        trivia(node)
    }

    fn convert_shorthand<'a>(&'a self, shorthand: Shorthand<'a>) -> BoxDoc<'a, ()> {
        let node = shorthand.to_untyped();
        trivia(node)
    }

    fn convert_smart_quote<'a>(&'a self, smart_quote: SmartQuote<'a>) -> BoxDoc<'a, ()> {
        let node = smart_quote.to_untyped();
        trivia(node)
    }

    fn convert_strong<'a>(&'a self, strong: Strong<'a>) -> BoxDoc<'a, ()> {
        let body = self.convert_markup(strong.body());
        BoxDoc::text("*").append(body).append(BoxDoc::text("*"))
    }

    fn convert_emph<'a>(&'a self, emph: Emph<'a>) -> BoxDoc<'a, ()> {
        let body = self.convert_markup(emph.body());
        BoxDoc::text("_").append(body).append(BoxDoc::text("_"))
    }

    fn convert_raw<'a>(&'a self, raw: Raw<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil();
        if raw.block() {
            doc = doc.append(BoxDoc::text("```"));
            if let Some(lang) = raw.lang() {
                doc = doc.append(BoxDoc::text(lang));
            }
            doc = doc.append(BoxDoc::hardline());
            doc = doc.append(to_doc(raw.text().to_string().into()));
            doc = doc.append(BoxDoc::hardline());
            doc = doc.append(BoxDoc::text("```"));
        } else {
            doc = doc.append(BoxDoc::text("`"));
            doc = doc.append(to_doc(raw.text().to_string().into()));
            doc = doc.append(BoxDoc::text("`"));
        }
        doc
    }

    fn convert_link<'a>(&'a self, link: Link<'a>) -> BoxDoc<'a, ()> {
        let node = link.to_untyped();
        trivia(node)
    }

    fn convert_label<'a>(&'a self, label: Label<'a>) -> BoxDoc<'a, ()> {
        let node = label.to_untyped();
        trivia(node)
    }

    fn convert_ref<'a>(&'a self, reference: Ref<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("@");
        doc = doc.append(BoxDoc::text(reference.target()));
        if let Some(supplement) = reference.supplement() {
            doc = doc.append(self.convert_content_block(supplement));
        }
        doc
    }

    fn convert_heading<'a>(&'a self, heading: Heading<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("=".repeat(heading.level().into()));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_markup(heading.body()));
        doc
    }

    fn convert_list_item<'a>(&'a self, list_item: ListItem<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("-");
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_markup(list_item.body()).nest(2));
        doc
    }

    fn convert_enum_item<'a>(&'a self, enum_item: EnumItem<'a>) -> BoxDoc<'a, ()> {
        let mut doc = if let Some(number) = enum_item.number() {
            BoxDoc::text(format!("{number}."))
        } else {
            BoxDoc::text("+")
        };
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_markup(enum_item.body()).nest(2));
        doc
    }

    fn convert_term_item<'a>(&'a self, term: TermItem<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("/");
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_markup(term.term()));
        doc = doc.append(BoxDoc::text(":"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_markup(term.description()).nest(2));
        doc
    }

    fn convert_equation<'a>(&'a self, equation: Equation<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("$");
        if equation.block() {
            doc = doc.append(BoxDoc::space());
        }
        doc = doc.append(self.convert_math(equation.body()).nest(2));
        if equation.block() {
            doc = doc.append(BoxDoc::space());
        }
        doc = doc.append(BoxDoc::text("$"));
        doc
    }

    fn convert_math<'a>(&'a self, math: Math<'a>) -> BoxDoc<'a, ()> {
        // TODO: check this later
        let mut doc = BoxDoc::nil();
        for expr in math.exprs() {
            doc = doc.append(self.convert_expr(expr));
        }
        doc
    }

    fn convert_ident<'a>(&'a self, ident: Ident<'a>) -> BoxDoc<'a, ()> {
        let doc = BoxDoc::nil().append(BoxDoc::text(ident.as_str()));
        doc
    }

    fn convert_none<'a>(&'a self, _none: None<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil().append(BoxDoc::text("none"))
    }

    fn convert_auto<'a>(&'a self, _auto: Auto<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil().append(BoxDoc::text("auto"))
    }

    fn convert_bool<'a>(&'a self, boolean: Bool<'a>) -> BoxDoc<'a, ()> {
        let node = boolean.to_untyped();
        trivia(node)
    }

    fn convert_int<'a>(&'a self, int: Int<'a>) -> BoxDoc<'a, ()> {
        let node = int.to_untyped();
        trivia(node)
    }

    fn convert_float<'a>(&'a self, float: Float<'a>) -> BoxDoc<'a, ()> {
        let node = float.to_untyped();
        trivia(node)
    }

    fn convert_numeric<'a>(&'a self, numeric: Numeric<'a>) -> BoxDoc<'a, ()> {
        let node = numeric.to_untyped();
        trivia(node)
    }

    fn convert_str<'a>(&'a self, str: Str<'a>) -> BoxDoc<'a, ()> {
        let node = str.to_untyped();
        trivia(node)
    }

    fn convert_code_block<'a>(&'a self, code_block: CodeBlock<'a>) -> BoxDoc<'a, ()> {
        let code = self.convert_code(code_block.body());

        let doc = pretty_items(
            &code,
            BoxDoc::text(";").append(BoxDoc::space()),
            BoxDoc::nil(),
            (BoxDoc::text("{"), BoxDoc::text("}")),
            true,
            util::FoldStyle::Never,
        );
        doc
    }

    fn convert_code<'a>(&'a self, code: Code<'a>) -> Vec<BoxDoc<'a, ()>> {
        let mut codes: Vec<_> = vec![];
        for node in code.to_untyped().children() {
            if let Some(expr) = node.cast::<Expr>() {
                let expr_doc = self.convert_expr(expr);
                codes.push(expr_doc);
            } else if node.kind() == SyntaxKind::LineComment {
                codes.push(trivia(node));
            }
        }
        codes
    }

    fn convert_content_block<'a>(&'a self, content_block: ContentBlock<'a>) -> BoxDoc<'a, ()> {
        let content = self.convert_markup(content_block.body()).group().nest(2);
        let doc = BoxDoc::text("[").append(content).append(BoxDoc::text("]"));
        doc
    }

    fn convert_parenthesized<'a>(&'a self, parenthesized: Parenthesized<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("(");
        let multiline_expr = BoxDoc::line()
            .append(self.convert_expr(parenthesized.expr()).nest(2))
            .append(BoxDoc::line())
            .group();
        let singleline_expr = self.convert_expr(parenthesized.expr());
        doc = doc.append(multiline_expr.flat_alt(singleline_expr));
        doc = doc.append(BoxDoc::text(")"));
        doc
    }

    fn convert_array<'a>(&'a self, array: Array<'a>) -> BoxDoc<'a, ()> {
        let array_items = array
            .items()
            .map(|item| self.convert_array_item(item))
            .collect_vec();
        pretty_items(
            &array_items,
            BoxDoc::text(",").append(BoxDoc::space()),
            BoxDoc::text(","),
            (BoxDoc::text("("), BoxDoc::text(")")),
            false,
            util::FoldStyle::Fit,
        )
    }

    fn convert_array_item<'a>(&'a self, array_item: ArrayItem<'a>) -> BoxDoc<'a, ()> {
        let doc = match array_item {
            ArrayItem::Pos(p) => self.convert_expr(p),
            // TODO: recheck how spread works
            ArrayItem::Spread(s) => BoxDoc::text("..").append(self.convert_expr(s)),
        };
        doc
    }

    fn convert_dict<'a>(&'a self, dict: Dict<'a>) -> BoxDoc<'a, ()> {
        if dict.items().count() == 0 {
            return BoxDoc::text("(:)");
        }
        let dict_items = dict
            .items()
            .map(|item| self.convert_dict_item(item))
            .collect_vec();
        pretty_items(
            &dict_items,
            BoxDoc::text(",").append(BoxDoc::space()),
            BoxDoc::text(","),
            (BoxDoc::text("("), BoxDoc::text(")")),
            false,
            util::FoldStyle::Fit,
        )
    }

    fn convert_dict_item<'a>(&'a self, dict_item: DictItem<'a>) -> BoxDoc<'a, ()> {
        match dict_item {
            DictItem::Named(n) => self.convert_named(n),
            DictItem::Keyed(k) => self.convert_keyed(k),
            DictItem::Spread(s) => {
                let mut doc = BoxDoc::text("..");
                doc = doc.append(self.convert_expr(s));
                doc
            }
        }
    }

    fn convert_named<'a>(&'a self, named: Named<'a>) -> BoxDoc<'a, ()> {
        let mut doc = self.convert_ident(named.name());
        doc = doc.append(BoxDoc::text(":"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_expr(named.expr()));
        doc
    }

    fn convert_keyed<'a>(&'a self, keyed: Keyed<'a>) -> BoxDoc<'a, ()> {
        let mut doc = self.convert_expr(keyed.key());
        doc = doc.append(BoxDoc::text(":"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_expr(keyed.expr()));
        doc
    }

    fn convert_unary<'a>(&'a self, unary: Unary<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::text(unary.op().as_str()).append(self.convert_expr(unary.expr()))
    }

    fn convert_binary<'a>(&'a self, binary: Binary<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil()
            .append(self.convert_expr(binary.lhs()))
            .append(BoxDoc::space())
            .append(BoxDoc::text(binary.op().as_str()))
            .append(BoxDoc::space())
            .append(self.convert_expr(binary.rhs()))
    }

    fn convert_field_access<'a>(&'a self, field_access: FieldAccess<'a>) -> BoxDoc<'a, ()> {
        let left = BoxDoc::nil().append(self.convert_expr(field_access.target()));
        let singleline_right = BoxDoc::text(".").append(self.convert_ident(field_access.field()));
        let multiline_right = BoxDoc::hardline()
            .append(BoxDoc::text("."))
            .append(self.convert_ident(field_access.field()))
            .nest(2)
            .group();
        left.append(multiline_right.flat_alt(singleline_right))
    }

    fn convert_func_call<'a>(&'a self, func_call: FuncCall<'a>) -> BoxDoc<'a, ()> {
        let doc = BoxDoc::nil().append(self.convert_expr(func_call.callee()));
        let doc = doc
            .append(pretty_items(
                &self.convert_parenthesized_args(func_call.args()),
                BoxDoc::text(",").append(BoxDoc::space()),
                BoxDoc::text(","),
                (BoxDoc::text("("), BoxDoc::text(")")),
                false,
                util::FoldStyle::Fit,
            ))
            .append(self.convert_additional_args(func_call.args()));
        doc
    }

    fn convert_parenthesized_args<'a>(&'a self, args: Args<'a>) -> Vec<BoxDoc<'a, ()>> {
        let node = args.to_untyped();
        let args = node
            .children()
            .take_while(|node| node.kind() != SyntaxKind::RightParen)
            .filter_map(|node| node.cast::<'_, Arg>())
            .map(|arg| self.convert_arg(arg))
            .collect();
        args
    }

    fn convert_additional_args<'a>(&'a self, args: Args<'a>) -> BoxDoc<'a, ()> {
        let node = args.to_untyped();
        let args = node
            .children()
            .skip_while(|node| node.kind() != SyntaxKind::RightParen)
            .filter_map(|node| node.cast::<'_, Arg>());
        BoxDoc::concat(args.map(|arg| self.convert_arg(arg))).group()
    }

    fn convert_arg<'a>(&'a self, arg: Arg<'a>) -> BoxDoc<'a, ()> {
        match arg {
            Arg::Pos(p) => self.convert_expr(p),
            Arg::Named(n) => self.convert_named(n),
            Arg::Spread(s) => {
                let mut doc = BoxDoc::text("..");
                doc = doc.append(self.convert_expr(s));
                doc
            }
        }
    }

    fn convert_closure<'a>(&'a self, closure: Closure<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil();
        let params = self.convert_params(closure.params());
        let arg_list = pretty_items(
            &params,
            BoxDoc::text(",").append(BoxDoc::space()),
            BoxDoc::text(","),
            (BoxDoc::text("("), BoxDoc::text(")")),
            false,
            util::FoldStyle::Fit,
        );
        if let Some(name) = closure.name() {
            doc = doc.append(self.convert_ident(name));
            doc = doc.append(arg_list);
            doc = doc.append(BoxDoc::space());
            doc = doc.append(BoxDoc::text("="));
            doc = doc.append(BoxDoc::space());
            doc = doc.append(self.convert_expr(closure.body()));
        } else {
            if params.len() > 1 {
                doc = arg_list
            } else {
                doc = params[0].clone();
            }
            doc = doc.append(BoxDoc::space());
            doc = doc.append(BoxDoc::text("=>"));
            doc = doc.append(BoxDoc::space());
            doc = doc.append(self.convert_expr(closure.body()));
        }
        doc
    }

    fn convert_params<'a>(&'a self, params: Params<'a>) -> Vec<BoxDoc<'a, ()>> {
        params
            .children()
            .map(|param| self.convert_param(param))
            .collect()
    }

    fn convert_param<'a>(&'a self, param: Param<'a>) -> BoxDoc<'a, ()> {
        match param {
            Param::Pos(p) => self.convert_pattern(p),
            Param::Named(n) => self.convert_named(n),
            Param::Sink(s) => self.convert_spread(s),
        }
    }

    fn convert_spread<'a>(&'a self, spread: Spread<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("..");
        if let Some(id) = spread.name() {
            doc = doc.append(self.convert_ident(id));
        }
        if let Some(expr) = spread.expr() {
            doc = doc.append(self.convert_expr(expr));
        }
        doc
    }

    fn convert_pattern<'a>(&'a self, pattern: Pattern<'a>) -> BoxDoc<'a, ()> {
        match pattern {
            Pattern::Normal(n) => self.convert_expr(n),
            Pattern::Placeholder(p) => self.convert_underscore(p),
            Pattern::Destructuring(d) => self.convert_destructuring(d),
        }
    }

    fn convert_underscore<'a>(&'a self, _underscore: Underscore<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::text("_")
    }

    fn convert_destructuring<'a>(&'a self, destructuring: Destructuring<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::text("(")
            .append(BoxDoc::intersperse(
                destructuring
                    .bindings()
                    .map(|item| self.convert_destructuring_kind(item)),
                BoxDoc::text(",").append(BoxDoc::line()),
            ))
            .append(BoxDoc::text(")"))
    }

    fn convert_destructuring_kind<'a>(
        &'a self,
        destructuring_kind: DestructuringKind<'a>,
    ) -> BoxDoc<'a, ()> {
        match destructuring_kind {
            DestructuringKind::Normal(e) => self.convert_expr(e),
            DestructuringKind::Sink(s) => self.convert_spread(s),
            DestructuringKind::Named(n) => self.convert_named(n),
            DestructuringKind::Placeholder(p) => self.convert_underscore(p),
        }
    }

    fn convert_let_binding<'a>(&'a self, let_binding: LetBinding<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil()
            .append(BoxDoc::text("let"))
            .append(BoxDoc::space());
        match let_binding.kind() {
            LetBindingKind::Normal(n) => {
                doc = doc.append(self.convert_pattern(n));
                if let Some(expr) = let_binding.init() {
                    doc = doc.append(BoxDoc::space());
                    doc = doc.append(BoxDoc::text("="));
                    doc = doc.append(BoxDoc::space());
                    doc = doc.append(self.convert_expr(expr));
                }
            }
            LetBindingKind::Closure(_c) => {
                if let Some(c) = let_binding.init() {
                    doc = doc.append(self.convert_expr(c));
                }
            }
        }
        doc
    }

    fn convert_destruct_assignment<'a>(
        &'a self,
        destruct_assign: DestructAssignment<'a>,
    ) -> BoxDoc<'a, ()> {
        self.convert_pattern(destruct_assign.pattern())
            .append(BoxDoc::space())
            .append(BoxDoc::text("="))
            .append(BoxDoc::space())
            .append(self.convert_expr(destruct_assign.value()))
    }

    fn convert_set_rule<'a>(&'a self, set_rule: SetRule<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil()
            .append(BoxDoc::text("set"))
            .append(BoxDoc::space());
        doc = doc.append(self.convert_expr(set_rule.target()));
        doc = doc.append(pretty_items(
            &self.convert_parenthesized_args(set_rule.args()),
            BoxDoc::text(",").append(BoxDoc::space()),
            BoxDoc::text(","),
            (BoxDoc::text("("), BoxDoc::text(")")),
            false,
            util::FoldStyle::Single,
        ));
        if let Some(condition) = set_rule.condition() {
            doc = doc.append(BoxDoc::space());
            doc = doc.append(BoxDoc::text("if"));
            doc = doc.append(BoxDoc::space());
            doc = doc.append(self.convert_expr(condition));
        }
        doc
    }

    fn convert_show_rule<'a>(&'a self, show_rule: ShowRule<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil().append(BoxDoc::text("show"));
        if let Some(selector) = show_rule.selector() {
            doc = doc.append(BoxDoc::space());
            doc = doc.append(self.convert_expr(selector));
        }
        doc = doc.append(BoxDoc::text(":"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_expr(show_rule.transform()));
        doc
    }

    fn convert_conditional<'a>(&'a self, conditional: Conditional<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil()
            .append(BoxDoc::text("if"))
            .append(BoxDoc::space());
        doc = doc.append(self.convert_expr(conditional.condition()));
        let body = self.convert_expr(conditional.if_body()).group();
        doc = doc.append(BoxDoc::space()).append(body);
        if let Some(else_body) = conditional.else_body() {
            doc = doc.append(BoxDoc::space());
            doc = doc.append(BoxDoc::text("else"));
            doc = doc.append(BoxDoc::space());
            doc = doc.append(self.convert_expr(else_body).group());
        }
        doc
    }

    fn convert_while<'a>(&'a self, while_loop: WhileLoop<'a>) -> BoxDoc<'a, ()> {
        let doc = BoxDoc::nil().append(BoxDoc::text("while"));
        let doc = doc
            .append(BoxDoc::space())
            .append(self.convert_expr(while_loop.condition()))
            .append(BoxDoc::space())
            .append(self.convert_expr(while_loop.body()));
        doc
    }

    fn convert_for<'a>(&'a self, for_loop: ForLoop<'a>) -> BoxDoc<'a, ()> {
        let doc = BoxDoc::nil().append(BoxDoc::text("for"));
        let doc = doc
            .append(BoxDoc::space())
            .append(self.convert_pattern(for_loop.pattern()))
            .append(BoxDoc::space())
            .append(BoxDoc::text("in"))
            .append(BoxDoc::space())
            .append(self.convert_expr(for_loop.iter()))
            .append(BoxDoc::space())
            .append(self.convert_expr(for_loop.body()));
        doc
    }

    fn convert_import<'a>(&'a self, import: ModuleImport<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil().append(BoxDoc::text("import"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(self.convert_expr(import.source()));
        if let Some(imports) = import.imports() {
            doc = doc.append(BoxDoc::text(":"));
            doc = doc.append(BoxDoc::space());
            let imports = match imports {
                Imports::Wildcard => BoxDoc::text("*"),
                Imports::Items(i) => BoxDoc::intersperse(
                    i.iter().map(|item| self.convert_import_item(item)),
                    BoxDoc::text(",").append(BoxDoc::line()),
                ),
            };
            doc = doc.append(imports.group());
        }
        doc
    }

    fn convert_import_item<'a>(&'a self, import_item: ImportItem<'a>) -> BoxDoc<'a, ()> {
        match import_item {
            ImportItem::Simple(s) => self.convert_ident(s),
            ImportItem::Renamed(r) => self
                .convert_ident(r.original_name())
                .append(BoxDoc::space())
                .append(BoxDoc::text("as"))
                .append(BoxDoc::space())
                .append(self.convert_ident(r.new_name())),
        }
    }

    fn convert_include<'a>(&'a self, include: ModuleInclude<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil()
            .append(BoxDoc::text("include"))
            .append(BoxDoc::space())
            .append(self.convert_expr(include.source()))
    }

    fn convert_break<'a>(&'a self, _break: LoopBreak<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil().append(BoxDoc::text("break"))
    }

    fn convert_continue<'a>(&'a self, _continue: LoopContinue<'a>) -> BoxDoc<'a, ()> {
        BoxDoc::nil().append(BoxDoc::text("continue"))
    }

    fn convert_return<'a>(&'a self, return_stmt: FuncReturn<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil()
            .append(BoxDoc::text("return"))
            .append(BoxDoc::space());
        if let Some(body) = return_stmt.body() {
            doc = doc.append(self.convert_expr(body));
        }
        doc
    }
}

fn trivia(node: &SyntaxNode) -> BoxDoc<'_, ()> {
    to_doc(std::borrow::Cow::Borrowed(node.text()))
}

pub fn to_doc(s: Cow<'_, str>) -> BoxDoc<'_, ()> {
    match s {
        Cow::Borrowed(s) => BoxDoc::intersperse(s.lines().map(BoxDoc::text), BoxDoc::hardline()),
        Cow::Owned(o) => BoxDoc::intersperse(
            o.lines().map(|s| BoxDoc::text(s.to_string())),
            BoxDoc::hardline(),
        ),
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
        for test in tests.into_iter() {
            insta::assert_debug_snapshot!(to_doc(test.into()));
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
