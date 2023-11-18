use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::ast;
use typst_syntax::ast::*;

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
        ast::Expr::Escape(_) => todo!(),
        ast::Expr::Shorthand(_) => todo!(),
        ast::Expr::SmartQuote(_) => todo!(),
        ast::Expr::Strong(_) => todo!(),
        ast::Expr::Emph(_) => todo!(),
        ast::Expr::Raw(_) => todo!(),
        ast::Expr::Link(_) => todo!(),
        ast::Expr::Label(_) => todo!(),
        ast::Expr::Ref(_) => todo!(),
        ast::Expr::Heading(_) => todo!(),
        ast::Expr::List(_) => todo!(),
        ast::Expr::Enum(_) => todo!(),
        ast::Expr::Term(_) => todo!(),
        ast::Expr::Equation(_) => todo!(),
        ast::Expr::Math(_) => todo!(),
        ast::Expr::MathIdent(_) => todo!(),
        ast::Expr::MathAlignPoint(_) => todo!(),
        ast::Expr::MathDelimited(_) => todo!(),
        ast::Expr::MathAttach(_) => todo!(),
        ast::Expr::MathPrimes(_) => todo!(),
        ast::Expr::MathFrac(_) => todo!(),
        ast::Expr::MathRoot(_) => todo!(),
        ast::Expr::Ident(_) => todo!(),
        ast::Expr::None(_) => todo!(),
        ast::Expr::Auto(_) => todo!(),
        ast::Expr::Bool(_) => todo!(),
        ast::Expr::Int(_) => todo!(),
        ast::Expr::Float(_) => todo!(),
        ast::Expr::Numeric(_) => todo!(),
        ast::Expr::Str(_) => todo!(),
        ast::Expr::Code(_) => todo!(),
        ast::Expr::Content(_) => todo!(),
        ast::Expr::Parenthesized(_) => todo!(),
        ast::Expr::Array(_) => todo!(),
        ast::Expr::Dict(_) => todo!(),
        ast::Expr::Unary(_) => todo!(),
        ast::Expr::Binary(_) => todo!(),
        ast::Expr::FieldAccess(_) => todo!(),
        ast::Expr::FuncCall(_) => todo!(),
        ast::Expr::Closure(_) => todo!(),
        ast::Expr::Let(_) => todo!(),
        ast::Expr::DestructAssign(_) => todo!(),
        ast::Expr::Set(_) => todo!(),
        ast::Expr::Show(_) => todo!(),
        ast::Expr::Conditional(_) => todo!(),
        ast::Expr::While(_) => todo!(),
        ast::Expr::For(_) => todo!(),
        ast::Expr::Import(_) => todo!(),
        ast::Expr::Include(_) => todo!(),
        ast::Expr::Break(_) => todo!(),
        ast::Expr::Continue(_) => todo!(),
        ast::Expr::Return(_) => todo!(),
    }
}

fn convert_text(text: Text<'_>) -> BoxDoc<'_, ()> {
    to_doc(text.get())
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
    to_doc(node.text())
}

fn convert_parbreak(_parbreak: Parbreak<'_>) -> BoxDoc<'_, ()> {
    BoxDoc::hardline().append(BoxDoc::hardline())
}

pub fn to_doc(s: &str) -> BoxDoc<'_, ()> {
    let mut doc: BoxDoc<()> = BoxDoc::nil();
    // find all '\n' indices and use that to split the string
    for (pos, slice) in s.split('\n').with_position() {
        match pos {
            itertools::Position::First | itertools::Position::Middle => {
                doc = doc.append(BoxDoc::text(slice)).append(BoxDoc::hardline());
            }
            itertools::Position::Last | itertools::Position::Only => {
                doc = doc.append(BoxDoc::text(slice));
            }
        }
    }
    doc
}
