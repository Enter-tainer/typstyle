use std::borrow::Cow;

use pretty::BoxDoc;
use typst_syntax::ast::*;
use typst_syntax::{ast, SyntaxNode};

pub fn convert_markup(root: Markup<'_>) -> BoxDoc<'_, ()> {
    let mut doc: BoxDoc<()> = BoxDoc::nil();
    for expr in root.exprs() {
        let expr_doc = convert_expr(expr);
        doc = doc.append(expr_doc).append(BoxDoc::hardline());
    }
    doc
}

fn convert_expr(expr: Expr<'_>) -> BoxDoc<'_, ()> {
    match expr {
        ast::Expr::Text(t) => convert_text(t),
        ast::Expr::Space(s) => convert_space(s),
        ast::Expr::Linebreak(b) => convert_linebreak(b),
        ast::Expr::Parbreak(b) => convert_parbreak(b),
        ast::Expr::Escape(e) => convert_escape(e),
        ast::Expr::Shorthand(s) => convert_shorthand(s),
        ast::Expr::SmartQuote(s) => convert_smart_quote(s),
        ast::Expr::Strong(s) => convert_strong(s),
        ast::Expr::Emph(e) => convert_emph(e),
        ast::Expr::Raw(r) => convert_raw(r),
        ast::Expr::Link(l) => convert_link(l),
        ast::Expr::Label(l) => convert_label(l),
        ast::Expr::Ref(r) => convert_ref(r),
        ast::Expr::Heading(h) => convert_heading(h),
        ast::Expr::List(l) => convert_list_item(l),
        ast::Expr::Enum(e) => convert_enum_item(e),
        ast::Expr::Term(t) => convert_term_item(t),
        ast::Expr::Equation(e) => convert_equation(e),
        ast::Expr::Math(m) => convert_math(m),
        ast::Expr::MathIdent(mi) => todo!(),
        ast::Expr::MathAlignPoint(map) => todo!(),
        ast::Expr::MathDelimited(md) => todo!(),
        ast::Expr::MathAttach(ma) => todo!(),
        ast::Expr::MathPrimes(mp) => todo!(),
        ast::Expr::MathFrac(mf) => todo!(),
        ast::Expr::MathRoot(mr) => todo!(),
        ast::Expr::Ident(i) => convert_ident(i),
        ast::Expr::None(n) => convert_none(n),
        ast::Expr::Auto(a) => convert_auto(a),
        ast::Expr::Bool(b) => convert_bool(b),
        ast::Expr::Int(i) => convert_int(i),
        ast::Expr::Float(f) => convert_float(f),
        ast::Expr::Numeric(n) => convert_numeric(n),
        ast::Expr::Str(s) => convert_str(s),
        ast::Expr::Code(c) => convert_code_block(c),
        ast::Expr::Content(c) => convert_content_block(c),
        ast::Expr::Parenthesized(p) => convert_parenthesized(p),
        ast::Expr::Array(a) => convert_array(a),
        ast::Expr::Dict(d) => convert_dict(d),
        ast::Expr::Unary(u) => convert_unary(u),
        ast::Expr::Binary(b) => convert_binary(b),
        ast::Expr::FieldAccess(fa) => convert_field_access(fa),
        ast::Expr::FuncCall(fc) => convert_func_call(fc),
        ast::Expr::Closure(c) => convert_closure(c),
        ast::Expr::Let(l) => convert_let_binding(l),
        ast::Expr::DestructAssign(da) => convert_destruct_assignment(da),
        ast::Expr::Set(s) => convert_set_rule(s),
        ast::Expr::Show(s) => convert_show_rule(s),
        ast::Expr::Conditional(c) => convert_conditional(c),
        ast::Expr::While(w) => convert_while(w),
        ast::Expr::For(f) => convert_for(f),
        ast::Expr::Import(i) => convert_import(i),
        ast::Expr::Include(i) => convert_include(i),
        ast::Expr::Break(b) => convert_break(b),
        ast::Expr::Continue(c) => convert_continue(c),
        ast::Expr::Return(r) => convert_return(r),
    }
}

fn convert_text(text: Text<'_>) -> BoxDoc<'_, ()> {
    let node = text.to_untyped();
    trivia(node)
}

fn convert_space(space: Space<'_>) -> BoxDoc<'_, ()> {
    let node = space.to_untyped();
    if node.text().contains('\n') {
        BoxDoc::hardline()
    } else {
        BoxDoc::space()
    }
}

fn convert_linebreak(linebreak: Linebreak<'_>) -> BoxDoc<'_, ()> {
    let node = linebreak.to_untyped();
    trivia(node)
}

fn convert_parbreak(_parbreak: Parbreak<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::hardline().append(BoxDoc::hardline())
}

fn convert_escape(escape: Escape<'_>) -> BoxDoc<'_, ()> {
    let node = escape.to_untyped();
    trivia(node)
}

fn convert_shorthand(shorthand: Shorthand<'_>) -> BoxDoc<'_, ()> {
    let node = shorthand.to_untyped();
    trivia(node)
}

fn convert_smart_quote(smart_quote: SmartQuote<'_>) -> BoxDoc<'_, ()> {
    let node = smart_quote.to_untyped();
    trivia(node)
}

fn convert_strong(strong: Strong<'_>) -> BoxDoc<'_, ()> {
    let body = convert_markup(strong.body());
    BoxDoc::text("*").append(body).append(BoxDoc::text("*"))
}

fn convert_emph(emph: Emph<'_>) -> BoxDoc<'_, ()> {
    let body = convert_markup(emph.body());
    BoxDoc::text("_").append(body).append(BoxDoc::text("_"))
}

fn convert_raw(raw: Raw<'_>) -> BoxDoc<'_, ()> {
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

fn convert_link(link: Link<'_>) -> BoxDoc<'_, ()> {
    let node = link.to_untyped();
    trivia(node)
}

fn convert_label(label: Label<'_>) -> BoxDoc<'_, ()> {
    let node = label.to_untyped();
    trivia(node)
}

fn convert_ref(reference: Ref<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("@");
    doc = doc.append(BoxDoc::text(reference.target()));
    if let Some(supplement) = reference.supplement() {
        doc = doc.append(convert_content_block(supplement));
    }
    doc
}

fn convert_heading(heading: Heading<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("=".repeat(heading.level().into()));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_markup(heading.body()));
    doc
}

fn convert_list_item(list_item: ListItem<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("-");
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_markup(list_item.body()));
    doc
}

fn convert_enum_item(enum_item: EnumItem<'_>) -> BoxDoc<'_, ()> {
    let mut doc = if let Some(number) = enum_item.number() {
        BoxDoc::text(format!("{number}."))
    } else {
        BoxDoc::text("+")
    };
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_markup(enum_item.body()));
    doc
}

fn convert_term_item(term: TermItem<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("/");
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_markup(term.term()));
    doc = doc.append(BoxDoc::text(":"));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_markup(term.description()));
    doc
}

fn convert_equation(equation: Equation<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("$");
    if equation.block() {
        doc = doc.append(BoxDoc::space());
    }
    doc = doc.append(convert_math(equation.body()));
    if equation.block() {
        doc = doc.append(BoxDoc::space());
    }
    doc = doc.append(BoxDoc::text("$"));
    doc
}

fn convert_math(math: Math<'_>) -> BoxDoc<'_, ()> {
    // TODO: check this later
    let mut doc = BoxDoc::nil();
    for expr in math.exprs() {
        doc = doc.append(convert_expr(expr));
    }
    doc
}

fn convert_ident(ident: Ident<'_>) -> BoxDoc<'_, ()> {
    let doc = BoxDoc::text(ident.as_str());
    doc
}

fn convert_none(_none: None<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("none")
}

fn convert_auto(_auto: Auto<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("auto")
}

fn convert_bool(boolean: Bool<'_>) -> BoxDoc<'_, ()> {
    let node = boolean.to_untyped();
    trivia(node)
}

fn convert_int(int: Int<'_>) -> BoxDoc<'_, ()> {
    let node = int.to_untyped();
    trivia(node)
}

fn convert_float(float: Float<'_>) -> BoxDoc<'_, ()> {
    let node = float.to_untyped();
    trivia(node)
}

fn convert_numeric(numeric: Numeric<'_>) -> BoxDoc<'_, ()> {
    let node = numeric.to_untyped();
    trivia(node)
}

fn convert_str(str: Str<'_>) -> BoxDoc<'_, ()> {
    let node = str.to_untyped();
    trivia(node)
}

fn convert_code_block(code_block: CodeBlock<'_>) -> BoxDoc<'_, ()> {
    let code = convert_code(code_block.body()).group().nest(2);
    let doc = BoxDoc::text("{")
        .append(BoxDoc::line())
        .append(code)
        .append(BoxDoc::line())
        .append(BoxDoc::text("}"));
    doc
}

fn convert_code(code: Code<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::nil();
    for expr in code.exprs() {
        doc = doc.append(convert_expr(expr)).append(BoxDoc::line());
    }
    doc
}

fn convert_content_block(content_block: ContentBlock<'_>) -> BoxDoc<'_, ()> {
    let content = convert_markup(content_block.body()).group().nest(2);
    let doc = BoxDoc::text("[")
        .append(BoxDoc::line())
        .append(content)
        .append(BoxDoc::line())
        .append(BoxDoc::text("]"));
    doc
}

fn convert_parenthesized(parenthesized: Parenthesized<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("(");
    doc = doc.append(convert_expr(parenthesized.expr()));
    doc = doc.append(BoxDoc::text(")"));
    doc
}

fn convert_array(array: Array<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("(");
    let items = BoxDoc::intersperse(
        array.items().map(convert_array_item),
        BoxDoc::text(",").append(BoxDoc::line()),
    )
    .group();
    doc = doc.append(items);
    doc = doc.append(BoxDoc::text(")"));
    doc
}

fn convert_array_item(array_item: ArrayItem<'_>) -> BoxDoc<'_, ()> {
    match array_item {
        ArrayItem::Pos(p) => convert_expr(p),
        // TODO: recheck how spread works
        ArrayItem::Spread(s) => BoxDoc::text("..").append(convert_expr(s)),
    }
}

fn convert_dict(dict: Dict<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("(");
    let items = BoxDoc::intersperse(
        dict.items().map(convert_dict_item),
        BoxDoc::text(",").append(BoxDoc::line()),
    )
    .group();
    doc = doc.append(items);
    doc = doc.append(BoxDoc::text(")"));
    doc
}

fn convert_dict_item(dict_item: DictItem<'_>) -> BoxDoc<'_, ()> {
    match dict_item {
        DictItem::Named(n) => convert_named(n),
        DictItem::Keyed(k) => convert_keyed(k),
        DictItem::Spread(s) => {
            let mut doc = BoxDoc::text("..");
            doc = doc.append(convert_expr(s));
            doc
        }
    }
}

fn convert_named(named: Named<'_>) -> BoxDoc<'_, ()> {
    let mut doc = convert_ident(named.name());
    doc = doc.append(BoxDoc::text(":"));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_expr(named.expr()));
    doc
}

fn convert_keyed(keyed: Keyed<'_>) -> BoxDoc<'_, ()> {
    let mut doc = convert_str(keyed.key());
    doc = doc.append(BoxDoc::text(":"));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_expr(keyed.expr()));
    doc
}

fn convert_unary(unary: Unary<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text(unary.op().as_str()).append(convert_expr(unary.expr()))
}

fn convert_binary(binary: Binary<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::nil()
        .append(convert_expr(binary.lhs()))
        .append(BoxDoc::space())
        .append(BoxDoc::text(binary.op().as_str()))
        .append(BoxDoc::space())
        .append(convert_expr(binary.rhs()))
}

fn convert_field_access(field_access: FieldAccess<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::nil()
        .append(convert_expr(field_access.target()))
        .append(BoxDoc::text("."))
        .append(convert_ident(field_access.field()))
}

fn convert_func_call(func_call: FuncCall<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::nil()
        .append(convert_expr(func_call.callee()))
        .append(BoxDoc::text("("))
        .append(convert_args(func_call.args()))
        .append(BoxDoc::text(")"))
}

fn convert_args(args: Args<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::intersperse(
        args.items().map(convert_arg),
        BoxDoc::text(",").append(BoxDoc::line()),
    )
    .group()
}

fn convert_arg(arg: Arg<'_>) -> BoxDoc<'_, ()> {
    match arg {
        Arg::Pos(p) => convert_expr(p),
        Arg::Named(n) => convert_named(n),
        Arg::Spread(s) => {
            let mut doc = BoxDoc::text("..");
            doc = doc.append(convert_expr(s));
            doc
        }
    }
}

fn convert_closure(closure: Closure<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::nil();
    if let Some(name) = closure.name() {
        doc = doc.append(convert_ident(name));
    }
    doc = doc.append(BoxDoc::text("("));
    doc = doc.append(convert_params(closure.params()));
    doc = doc.append(BoxDoc::text(")"));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(BoxDoc::text("=>"));
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_expr(closure.body()));
    doc
}

fn convert_params(params: Params<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::intersperse(
        params.children().map(convert_param),
        BoxDoc::text(",").append(BoxDoc::line()),
    )
}

fn convert_param(param: Param<'_>) -> BoxDoc<'_, ()> {
    match param {
        Param::Pos(p) => convert_pattern(p),
        Param::Named(n) => convert_named(n),
        Param::Sink(s) => convert_spread(s),
    }
}

fn convert_spread(spread: Spread<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("..");
    if let Some(id) = spread.name() {
        doc = doc.append(convert_ident(id));
    }
    if let Some(expr) = spread.expr() {
        doc = doc.append(convert_expr(expr));
    }
    doc
}

fn convert_pattern(pattern: Pattern<'_>) -> BoxDoc<'_, ()> {
    match pattern {
        Pattern::Normal(n) => convert_expr(n),
        Pattern::Placeholder(p) => convert_underscore(p),
        Pattern::Destructuring(d) => convert_destructuring(d),
    }
}

fn convert_underscore(_underscore: Underscore<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("_")
}

fn convert_destructuring(destructuring: Destructuring<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("(")
        .append(BoxDoc::intersperse(
            destructuring.bindings().map(convert_destructuring_kind),
            BoxDoc::text(",").append(BoxDoc::line()),
        ))
        .append(BoxDoc::text(")"))
}

fn convert_destructuring_kind(destructuring_kind: DestructuringKind<'_>) -> BoxDoc<'_, ()> {
    match destructuring_kind {
        DestructuringKind::Normal(e) => convert_expr(e),
        DestructuringKind::Sink(s) => convert_spread(s),
        DestructuringKind::Named(n) => convert_named(n),
        DestructuringKind::Placeholder(p) => convert_underscore(p),
    }
}

fn convert_let_binding(let_binding: LetBinding<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("let").append(BoxDoc::space());
    match let_binding.kind() {
        LetBindingKind::Normal(n) => {
            doc = doc.append(convert_pattern(n));
            if let Some(expr) = let_binding.init() {
                doc = doc.append(BoxDoc::space());
                doc = doc.append(BoxDoc::text("="));
                doc = doc.append(BoxDoc::space());
                doc = doc.append(convert_expr(expr));
            }
        }
        LetBindingKind::Closure(_c) => {
            if let Some(c) = let_binding.init() {
                doc = doc.append(convert_expr(c));
            }
        }
    }
    doc
}

fn convert_destruct_assignment(destruct_assign: DestructAssignment<'_>) -> BoxDoc<'_, ()> {
    convert_pattern(destruct_assign.pattern())
        .append(BoxDoc::space())
        .append(BoxDoc::text("="))
        .append(BoxDoc::space())
        .append(convert_expr(destruct_assign.value()))
}

fn convert_set_rule(set_rule: SetRule<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("set").append(BoxDoc::space());
    doc = doc.append(convert_expr(set_rule.target()));
    doc = doc.append(convert_args(set_rule.args()));
    if let Some(condition) = set_rule.condition() {
        doc = doc.append(BoxDoc::space());
        doc = doc.append(BoxDoc::text("if"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(convert_expr(condition));
    }
    doc
}

fn convert_show_rule(show_rule: ShowRule<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("show");
    if let Some(selector) = show_rule.selector() {
        doc = doc.append(BoxDoc::space());
        doc = doc.append(convert_expr(selector));
    }
    doc = doc.append(BoxDoc::text(":"));
    doc = doc.append(convert_expr(show_rule.transform()));
    doc
}

fn convert_conditional(conditional: Conditional<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("if")
        .append(BoxDoc::space())
        .append(convert_expr(conditional.condition()))
        .append(BoxDoc::space());
    let body = convert_expr(conditional.if_body());
    doc = doc.append(body);
    if let Some(else_body) = conditional.else_body() {
        doc = doc.append(BoxDoc::space());
        doc = doc.append(BoxDoc::text("else"));
        doc = doc.append(BoxDoc::space());
        doc = doc.append(convert_expr(else_body));
    }
    doc
}

fn convert_while(while_loop: WhileLoop<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("while")
        .append(BoxDoc::space())
        .append(convert_expr(while_loop.condition()))
        .append(BoxDoc::space())
        .append(convert_expr(while_loop.body()))
}

fn convert_for(for_loop: ForLoop<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("for")
        .append(BoxDoc::space())
        .append(convert_pattern(for_loop.pattern()))
        .append(BoxDoc::space())
        .append(BoxDoc::text("in"))
        .append(BoxDoc::space())
        .append(convert_expr(for_loop.iter()))
        .append(BoxDoc::space())
        .append(convert_expr(for_loop.body()))
}

fn convert_import(import: ModuleImport<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("import");
    doc = doc.append(BoxDoc::space());
    doc = doc.append(convert_expr(import.source()));
    if let Some(imports) = import.imports() {
        doc = doc.append(BoxDoc::text(":"));
        doc = doc.append(BoxDoc::space());
        let imports = match imports {
            Imports::Wildcard => BoxDoc::text("*"),
            Imports::Items(i) => BoxDoc::intersperse(
                i.iter().map(convert_import_item),
                BoxDoc::text(",").append(BoxDoc::line()),
            ),
        };
        doc = doc.append(imports.group());
    }
    doc
}

fn convert_import_item(import_item: ImportItem<'_>) -> BoxDoc<'_, ()> {
    match import_item {
        ImportItem::Simple(s) => convert_ident(s),
        ImportItem::Renamed(r) => convert_ident(r.original_name())
            .append(BoxDoc::space())
            .append(BoxDoc::text("as"))
            .append(BoxDoc::space())
            .append(convert_ident(r.new_name())),
    }
}

fn convert_include(include: ModuleInclude<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("include")
        .append(BoxDoc::space())
        .append(convert_expr(include.source()))
}

fn convert_break(_break: LoopBreak<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("break")
}

fn convert_continue(_continue: LoopContinue<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::text("continue")
}

fn convert_return(return_stmt: FuncReturn<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("return").append(BoxDoc::space());
    if let Some(body) = return_stmt.body() {
        doc = doc.append(convert_expr(body));
    }
    doc
}

fn trivia(node: &SyntaxNode) -> BoxDoc<'_, ()> {
    to_doc(std::borrow::Cow::Borrowed(node.text()))
}

pub fn to_doc(s: Cow<'_, str>) -> BoxDoc<'_, ()> {
    let mut doc: BoxDoc<()> = BoxDoc::nil();
    match s {
        Cow::Borrowed(s) => {
            for line in s.lines() {
                doc = if line.is_empty() {
                    doc.append(BoxDoc::hardline())
                } else {
                    doc.append(BoxDoc::text(line)).append(BoxDoc::hardline())
                };
            }
        }
        Cow::Owned(o) => {
            for line in o.lines() {
                doc = if line.is_empty() {
                    doc.append(BoxDoc::hardline())
                } else {
                    doc.append(BoxDoc::text(line.to_string()))
                        .append(BoxDoc::hardline())
                };
            }
        }
    }
    doc
}
