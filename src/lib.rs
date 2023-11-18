use std::borrow::Cow;

use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::ast::*;
use typst_syntax::{ast, SyntaxNode};

pub fn convert_markup(root: Markup<'_>) -> BoxDoc<'_, ()> {
    let doc: BoxDoc<()> = BoxDoc::nil();
    for expr in root.exprs() {
        let expr_doc = convert_expr(expr);
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
        ast::Expr::Array(a) => todo!(),
        ast::Expr::Dict(d) => todo!(),
        ast::Expr::Unary(u) => todo!(),
        ast::Expr::Binary(b) => todo!(),
        ast::Expr::FieldAccess(fa) => todo!(),
        ast::Expr::FuncCall(fc) => todo!(),
        ast::Expr::Closure(c) => todo!(),
        ast::Expr::Let(l) => todo!(),
        ast::Expr::DestructAssign(da) => todo!(),
        ast::Expr::Set(s) => todo!(),
        ast::Expr::Show(s) => todo!(),
        ast::Expr::Conditional(c) => todo!(),
        ast::Expr::While(w) => todo!(),
        ast::Expr::For(f) => todo!(),
        ast::Expr::Import(i) => todo!(),
        ast::Expr::Include(i) => todo!(),
        ast::Expr::Break(b) => todo!(),
        ast::Expr::Continue(c) => todo!(),
        ast::Expr::Return(r) => todo!(),
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
    todo!()
}

fn convert_parenthesized(parenthesized: Parenthesized<'_>) -> BoxDoc<'_, ()> {
    let mut doc = BoxDoc::text("(");
    doc = doc.append(convert_expr(parenthesized.expr()));
    doc = doc.append(BoxDoc::text(")"));
    doc
}

fn convert_content_block(content_block: ContentBlock<'_>) -> BoxDoc<'_, ()> {
    // let mut doc = BoxDoc::text("{");
    // for expr in content_block.exprs() {
    //   doc = doc.append(convert_expr(expr));
    // }
    // doc = doc.append(BoxDoc::text("}"));
    // doc
    todo!()
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
