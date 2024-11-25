use itertools::Itertools;
use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{style::FoldStyle, PrettyPrinter};

use super::{
    table,
    util::{get_parenthesized_args_untyped, has_parenthesized_args},
    MyDoc,
};

#[derive(Debug)]
pub(super) enum ParenthesizedFuncCallArg<'a> {
    Argument(Arg<'a>),
    Comma,
    Space,
    Newline(usize),
    LineComment(&'a SyntaxNode),
    BlockComment(&'a SyntaxNode),
}

impl ParenthesizedFuncCallArg<'_> {
    #[allow(unused)]
    pub fn is_comment(&self) -> bool {
        matches!(
            self,
            ParenthesizedFuncCallArg::LineComment(_) | ParenthesizedFuncCallArg::BlockComment(_)
        )
    }

    pub fn is_function_call(&self) -> bool {
        if let ParenthesizedFuncCallArg::Argument(arg) = self {
            let inner = match arg {
                Arg::Pos(p) => *p,
                Arg::Named(n) => n.expr(),
                Arg::Spread(s) => s.expr(),
            };
            matches!(inner, Expr::FuncCall(_))
        } else {
            false
        }
    }

    #[allow(unused)]
    pub fn is_newline(&self) -> Option<usize> {
        if let ParenthesizedFuncCallArg::Newline(count) = self {
            Some(*count)
        } else {
            None
        }
    }

    pub fn is_trivial(&self) -> bool {
        matches!(
            self,
            ParenthesizedFuncCallArg::Space | ParenthesizedFuncCallArg::Comma
        )
    }
}

impl<'a> ParenthesizedFuncCallArg<'a> {
    pub fn into_doc(
        self,
        printer: &'a PrettyPrinter<'a>,
        reduce_newline: Option<usize>,
    ) -> MyDoc<'a> {
        match self {
            ParenthesizedFuncCallArg::Argument(arg) => printer.convert_arg(arg),
            ParenthesizedFuncCallArg::Comma => printer.arena.text(","),
            ParenthesizedFuncCallArg::Space => printer.arena.space(),
            ParenthesizedFuncCallArg::Newline(count) => printer.arena.concat(std::iter::repeat_n(
                printer.arena.hardline(),
                count - reduce_newline.unwrap_or(0),
            )),
            ParenthesizedFuncCallArg::LineComment(cmt)
            | ParenthesizedFuncCallArg::BlockComment(cmt) => printer.convert_comment(cmt),
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_func_call(&'a self, func_call: FuncCall<'a>) -> MyDoc<'a> {
        let mut doc = self.convert_expr(func_call.callee());
        if let Some(res) = self.check_disabled(func_call.args().to_untyped()) {
            return doc + res;
        }
        let has_parenthesized_args = has_parenthesized_args(func_call.args());
        if table::is_table(func_call) {
            if let Some(cols) = table::is_formatable_table(func_call) {
                doc += self.convert_table(func_call, cols);
            } else if has_parenthesized_args {
                doc += self.convert_parenthesized_args_as_is(func_call.args());
            }
        } else if has_parenthesized_args {
            doc += self.convert_parenthesized_args(func_call.args());
        };
        doc + self.convert_additional_args(func_call.args(), has_parenthesized_args)
    }

    pub(super) fn convert_args(&'a self, args: Args<'a>) -> MyDoc<'a> {
        let has_parenthesized_args = has_parenthesized_args(args);
        let parenthesized = if has_parenthesized_args {
            self.convert_parenthesized_args_as_is(args)
        } else {
            self.arena.nil()
        };
        parenthesized + self.convert_additional_args(args, has_parenthesized_args)
    }

    pub(super) fn convert_parenthesized_args(&'a self, args: Args<'a>) -> MyDoc<'a> {
        let args = parse_args(args).collect_vec();
        let is_multiline = {
            let mut is_multiline = false;
            for arg in &args {
                if let ParenthesizedFuncCallArg::Space = arg {
                    break;
                }
                if let ParenthesizedFuncCallArg::Newline(_) = arg {
                    is_multiline = true;
                    break;
                }
            }
            is_multiline
        };
        let prefer_tighter = {
            let real_args = args
                .iter()
                .filter(|x| matches!(x, ParenthesizedFuncCallArg::Argument(_)))
                .collect_vec();
            real_args.is_empty() || (real_args.len() == 1 && !real_args[0].is_function_call())
        };
        let non_trivial_args = args.into_iter().filter(|x| !x.is_trivial());
        let doc = if prefer_tighter {
            non_trivial_args
                .into_iter()
                .find(|x| matches!(x, ParenthesizedFuncCallArg::Argument(_)))
                .map(|x| x.into_doc(self, None))
                .unwrap_or_else(|| self.arena.nil())
                .parens()
        } else {
            // remove trailing newlines
            let mut args = non_trivial_args.collect_vec();
            while let Some(ParenthesizedFuncCallArg::Newline(_)) = args.last() {
                args.pop();
            }
            comma_seprated_args(
                self,
                args.into_iter(),
                if is_multiline {
                    FoldStyle::Never
                } else {
                    FoldStyle::Fit
                },
            )
        };
        doc
    }

    pub(super) fn convert_parenthesized_args_as_is(&'a self, args: Args<'a>) -> MyDoc<'a> {
        let args = parse_args(args);
        let inner = self.arena.concat(args.map(|arg| arg.into_doc(self, None)));
        inner.nest(2).parens()
    }

    fn convert_additional_args(&'a self, args: Args<'a>, has_paren: bool) -> MyDoc<'a> {
        let node = args.to_untyped();
        let args = node
            .children()
            .skip_while(|node| {
                if has_paren {
                    node.kind() != SyntaxKind::RightParen
                } else {
                    node.kind() != SyntaxKind::ContentBlock
                }
            })
            .filter_map(|node| node.cast::<Arg>());
        self.arena
            .concat(args.map(|arg| self.convert_arg(arg)))
            .group()
    }

    pub(super) fn convert_arg(&'a self, arg: Arg<'a>) -> MyDoc<'a> {
        match arg {
            Arg::Pos(p) => self.convert_expr(p),
            Arg::Named(n) => self.convert_named(n),
            Arg::Spread(s) => self.convert_spread(s),
        }
    }
}

fn parse_args(args: Args<'_>) -> impl Iterator<Item = ParenthesizedFuncCallArg<'_>> {
    get_parenthesized_args_untyped(args).map(|node| match node.kind() {
        SyntaxKind::Comma => ParenthesizedFuncCallArg::Comma,
        SyntaxKind::Space => {
            let newline_count = node.text().chars().filter(|&c| c == '\n').count();
            if newline_count > 0 {
                ParenthesizedFuncCallArg::Newline(newline_count)
            } else {
                ParenthesizedFuncCallArg::Space
            }
        }
        SyntaxKind::LineComment => ParenthesizedFuncCallArg::LineComment(node),
        SyntaxKind::BlockComment => ParenthesizedFuncCallArg::BlockComment(node),
        _ => ParenthesizedFuncCallArg::Argument(node.cast::<Arg>().unwrap()),
    })
}

fn comma_seprated_args<'a, I>(
    pp: &'a PrettyPrinter<'a>,
    args: I,
    fold_style: FoldStyle,
) -> MyDoc<'a>
where
    I: Iterator<Item = ParenthesizedFuncCallArg<'a>> + ExactSizeIterator,
{
    if args.len() == 0 {
        return pp.arena.nil().parens();
    }
    let format_inner = |sep: MyDoc<'a>, comma_: MyDoc<'a>| {
        let mut inner = pp.arena.nil();
        for (pos, arg) in args.with_position() {
            let need_sep = matches!(arg, ParenthesizedFuncCallArg::Argument(_));
            inner += arg.into_doc(pp, Some(1));
            if matches!(
                pos,
                itertools::Position::First | itertools::Position::Middle
            ) && need_sep
            {
                inner += sep.clone();
            }
        }
        inner + comma_
    };
    match fold_style {
        FoldStyle::Fit => {
            let comma_ = pp.arena.text(",").flat_alt(pp.arena.nil());
            let sep = pp.arena.text(",") + pp.arena.line();
            let inner = format_inner(sep, comma_);
            ((pp.arena.line_() + inner).nest(2) + pp.arena.line_())
                .group()
                .parens()
        }
        FoldStyle::Never => {
            let sep = pp.arena.text(",") + pp.arena.hardline();
            let comma_ = pp.arena.text(",");
            let inner = format_inner(sep, comma_);
            ((pp.arena.hardline() + inner).nest(2) + pp.arena.hardline()).parens()
        }
    }
}
