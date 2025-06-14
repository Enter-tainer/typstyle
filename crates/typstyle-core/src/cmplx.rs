use typst_syntax::ast::{self, AstNode, Expr};

use crate::ext::StrExt;

pub trait SumComplexity {
    fn sum_complexity(self) -> u32;
}

impl<T, U> SumComplexity for T
where
    T: Iterator<Item = U>,
    U: Complexity,
{
    fn sum_complexity(self) -> u32 {
        self.map(U::complexity).sum::<u32>()
    }
}

pub trait Complexity {
    fn complexity(self) -> u32;
}

impl Complexity for &str {
    fn complexity(self) -> u32 {
        self.len().div_ceil(10) as u32
    }
}

impl Complexity for Expr<'_> {
    fn complexity(self) -> u32 {
        match self {
            Expr::Text(text) => text.complexity(),
            Expr::Space(space) => {
                if space.to_untyped().text().has_linebreak() {
                    10
                } else {
                    0
                }
            }
            Expr::Parbreak(_) => 20,
            Expr::Strong(strong) => 1 + strong.body().complexity(),
            Expr::Emph(emph) => 1 + emph.body().complexity(),
            Expr::Raw(raw) => 2 + raw.lines().sum_complexity(),
            Expr::Link(_) => 3,
            Expr::Label(_) => 2,
            Expr::Ref(_) => 2,
            Expr::Heading(heading) => 1 + heading.body().complexity(),
            Expr::List(list_item) => 1 + list_item.body().complexity(),
            Expr::Enum(enum_item) => 1 + enum_item.body().complexity(),
            Expr::Term(term_item) => {
                1 + term_item.term().complexity() + term_item.description().complexity()
            }

            Expr::Equation(equation) => 2 + equation.body().exprs().sum_complexity(),
            Expr::Math(math) => 1 + math.exprs().sum_complexity(),

            Expr::Str(str) => str.get().complexity(),

            Expr::Code(code_block) => 2 + code_block.body().exprs().sum_complexity(),
            Expr::Content(content_block) => 2 + content_block.body().exprs().sum_complexity(),
            Expr::Parenthesized(parenthesized) => parenthesized.expr().complexity(),
            Expr::Array(array) => {
                2 + array
                    .items()
                    .map(|item| match item {
                        ast::ArrayItem::Pos(expr) => expr.complexity(),
                        ast::ArrayItem::Spread(spread) => 1 + spread.expr().complexity(),
                    })
                    .sum::<u32>()
            }
            Expr::Dict(dict) => {
                2 + dict
                    .items()
                    .map(|item| match item {
                        ast::DictItem::Named(named) => 1 + named.expr().complexity(),
                        ast::DictItem::Keyed(keyed) => 1 + keyed.expr().complexity(),
                        ast::DictItem::Spread(spread) => 1 + spread.expr().complexity(),
                    })
                    .sum::<u32>()
            }
            Expr::Unary(unary) => 1 + unary.expr().complexity(),
            Expr::Binary(binary) => 1 + binary.lhs().complexity() + binary.rhs().complexity(),
            Expr::FieldAccess(field_access) => 1 + field_access.target().complexity(),
            Expr::FuncCall(func_call) => {
                1 + func_call.callee().complexity() + func_call.args().complexity()
            }
            Expr::Closure(closure) => {
                2 + closure.params().complexity() + closure.body().complexity()
            }

            // As for our usage, statements cannot appear in args,
            // so we can directly give them a large complexity.
            Expr::Let(_) => 10,
            Expr::DestructAssign(_) => 10,
            Expr::Set(_) => 10,
            Expr::Show(_) => 10,

            Expr::Contextual(contextual) => 2 + contextual.body().complexity(),
            Expr::Conditional(conditional) => {
                4 + conditional.condition().complexity()
                    + conditional.if_body().complexity()
                    + conditional
                        .else_body()
                        .map(Expr::complexity)
                        .unwrap_or_default()
            }
            Expr::While(while_loop) => {
                5 + while_loop.condition().complexity() + while_loop.body().complexity()
            }
            Expr::For(for_loop) => {
                5 + for_loop.pattern().complexity()
                    + for_loop.iterable().complexity()
                    + for_loop.body().complexity()
            }
            Expr::Import(_) | Expr::Include(_) => 10,

            _ => 1,
        }
    }
}

impl Complexity for ast::Markup<'_> {
    fn complexity(self) -> u32 {
        self.exprs().sum_complexity()
    }
}

impl Complexity for ast::Text<'_> {
    fn complexity(self) -> u32 {
        self.get().complexity()
    }
}

impl Complexity for ast::Args<'_> {
    fn complexity(self) -> u32 {
        self.items()
            .map(|arg| match arg {
                ast::Arg::Pos(expr) => expr.complexity(),
                ast::Arg::Named(named) => 1 + named.expr().complexity(),
                ast::Arg::Spread(spread) => 1 + spread.expr().complexity(),
            })
            .sum::<u32>()
    }
}

impl Complexity for ast::Params<'_> {
    fn complexity(self) -> u32 {
        self.children()
            .map(|param| match param {
                ast::Param::Pos(pattern) => pattern.complexity(),
                ast::Param::Named(named) => 1 + named.expr().complexity(),
                ast::Param::Spread(spread) => 1 + spread.expr().complexity(),
            })
            .sum::<u32>()
    }
}

impl Complexity for ast::Pattern<'_> {
    fn complexity(self) -> u32 {
        match self {
            ast::Pattern::Normal(expr) => expr.complexity(),
            ast::Pattern::Placeholder(_) => 1,
            ast::Pattern::Parenthesized(parenthesized) => parenthesized.pattern().complexity(),
            ast::Pattern::Destructuring(destructuring) => destructuring.complexity(),
        }
    }
}

impl Complexity for ast::Destructuring<'_> {
    fn complexity(self) -> u32 {
        self.items().sum_complexity()
    }
}

impl Complexity for ast::DestructuringItem<'_> {
    fn complexity(self) -> u32 {
        match self {
            ast::DestructuringItem::Pattern(pattern) => pattern.complexity(),
            ast::DestructuringItem::Named(named) => 1 + named.expr().complexity(),
            ast::DestructuringItem::Spread(spread) => 1 + spread.expr().complexity(),
        }
    }
}
