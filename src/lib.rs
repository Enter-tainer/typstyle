use std::borrow::Cow;

use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::ast::*;
use typst_syntax::{ast, SyntaxNode};

pub fn convert(root: Markup<'_>) -> BoxDoc<'_, ()> {
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
        ast::Expr::Link(l) => todo!(),
        ast::Expr::Label(l) => todo!(),
        ast::Expr::Ref(r) => todo!(),
        ast::Expr::Heading(h) => todo!(),
        ast::Expr::List(l) => todo!(),
        ast::Expr::Enum(e) => todo!(),
        ast::Expr::Term(t) => todo!(),
        ast::Expr::Equation(e) => todo!(),
        ast::Expr::Math(m) => todo!(),
        ast::Expr::MathIdent(mi) => todo!(),
        ast::Expr::MathAlignPoint(map) => todo!(),
        ast::Expr::MathDelimited(md) => todo!(),
        ast::Expr::MathAttach(ma) => todo!(),
        ast::Expr::MathPrimes(mp) => todo!(),
        ast::Expr::MathFrac(mf) => todo!(),
        ast::Expr::MathRoot(mr) => todo!(),
        ast::Expr::Ident(i) => todo!(),
        ast::Expr::None(n) => todo!(),
        ast::Expr::Auto(a) => todo!(),
        ast::Expr::Bool(b) => todo!(),
        ast::Expr::Int(i) => todo!(),
        ast::Expr::Float(f) => todo!(),
        ast::Expr::Numeric(n) => todo!(),
        ast::Expr::Str(s) => todo!(),
        ast::Expr::Code(c) => todo!(),
        ast::Expr::Content(c) => todo!(),
        ast::Expr::Parenthesized(p) => todo!(),
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
    let body = convert(strong.body());
    BoxDoc::text("*").append(body).append(BoxDoc::text("*"))
}

fn convert_emph(emph: Emph<'_>) -> BoxDoc<'_, ()> {
    let body = convert(emph.body());
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

fn trivia(node: &SyntaxNode) -> BoxDoc<'_, ()> {
    to_doc(node.text().to_string().into())
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
