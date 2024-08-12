use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use crate::{pretty::trivia, util::FoldStyle, PrettyPrinter};

use super::{
    table,
    util::{self, get_parenthesized_args_untyped},
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
        printer: &'a PrettyPrinter,
        reduce_newline: Option<usize>,
    ) -> BoxDoc<'a, ()> {
        match self {
            ParenthesizedFuncCallArg::Argument(arg) => printer.convert_arg(arg),
            ParenthesizedFuncCallArg::Comma => BoxDoc::text(","),
            ParenthesizedFuncCallArg::Space => BoxDoc::space(),
            ParenthesizedFuncCallArg::Newline(count) => {
                let mut inner = BoxDoc::nil();
                for _ in reduce_newline.unwrap_or(0)..count {
                    inner = inner.append(BoxDoc::hardline());
                }
                inner
            }
            ParenthesizedFuncCallArg::LineComment(comment)
            | ParenthesizedFuncCallArg::BlockComment(comment) => trivia(comment),
        }
    }
}

impl PrettyPrinter {
    pub(super) fn convert_func_call<'a>(&'a self, func_call: FuncCall<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil().append(self.convert_expr(func_call.callee()));
        if let Some(res) = self.check_disabled(func_call.args().to_untyped()) {
            return doc.append(res);
        }
        let has_parenthesized_args = util::has_parenthesized_args(func_call.args());
        if table::is_table(func_call) {
            if let Some(cols) = table::is_formatable_table(func_call) {
                doc = doc.append(self.convert_table(func_call, cols));
            } else if has_parenthesized_args {
                doc = doc.append(self.convert_parenthesized_args_as_is(func_call.args()));
            }
        } else if has_parenthesized_args {
            doc = doc.append(self.convert_parenthesized_args(func_call.args()));
        };
        doc.append(self.convert_additional_args(func_call.args(), has_parenthesized_args))
    }

    pub(super) fn convert_args<'a>(&'a self, args: Args<'a>) -> BoxDoc<'a, ()> {
        let has_parenthesized_args = util::has_parenthesized_args(args);
        let mut doc = BoxDoc::nil();
        if has_parenthesized_args {
            doc = doc.append(self.convert_parenthesized_args_as_is(args));
        }
        doc = doc.append(self.convert_additional_args(args, has_parenthesized_args));
        doc
    }

    pub(super) fn convert_parenthesized_args<'a>(&'a self, args: Args<'a>) -> BoxDoc<'a, ()> {
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
            BoxDoc::text("(")
                .append(
                    non_trivial_args
                        .into_iter()
                        .find(|x| matches!(x, ParenthesizedFuncCallArg::Argument(_)))
                        .map(|x| x.into_doc(self, None))
                        .unwrap_or_else(BoxDoc::nil),
                )
                .append(BoxDoc::text(")"))
        } else {
            // remove trailing newlines
            let mut args = non_trivial_args.collect_vec();
            while let Some(ParenthesizedFuncCallArg::Newline(_)) = args.last() {
                args.pop();
            }
            comma_separated_args(
                args.into_iter(),
                if is_multiline {
                    FoldStyle::Never
                } else {
                    FoldStyle::Fit
                },
                self,
            )
        };
        doc
    }

    pub(super) fn convert_parenthesized_args_as_is<'a>(&'a self, args: Args<'a>) -> BoxDoc<'a, ()> {
        let args = parse_args(args);
        let mut inner = BoxDoc::nil();
        for arg in args {
            inner = inner.append(arg.into_doc(self, None));
        }
        BoxDoc::text("(")
            .append(inner.nest(2))
            .append(BoxDoc::text(")"))
    }

    fn convert_additional_args<'a>(&'a self, args: Args<'a>, has_paren: bool) -> BoxDoc<'a, ()> {
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
            .filter_map(|node| node.cast::<'_, Arg>());
        BoxDoc::concat(args.map(|arg| self.convert_arg(arg))).group()
    }

    pub(super) fn convert_arg<'a>(&'a self, arg: Arg<'a>) -> BoxDoc<'a, ()> {
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

fn comma_separated_args<'a, I>(
    args: I,
    fold_style: FoldStyle,
    pp: &'a PrettyPrinter,
) -> BoxDoc<'a, ()>
where
    I: Iterator<Item = ParenthesizedFuncCallArg<'a>> + ExactSizeIterator,
{
    if args.len() == 0 {
        return BoxDoc::text("()");
    }
    let format_inner = |sep: BoxDoc<'a, ()>, comma_: BoxDoc<'a, ()>| {
        let mut inner = BoxDoc::nil();
        for (pos, arg) in args.with_position() {
            let need_sep = matches!(arg, ParenthesizedFuncCallArg::Argument(_));
            inner = inner.append(arg.into_doc(pp, Some(1)));
            if matches!(
                pos,
                itertools::Position::First | itertools::Position::Middle
            ) && need_sep
            {
                inner = inner.append(sep.clone());
            }
        }
        inner.append(comma_)
    };
    match fold_style {
        FoldStyle::Fit => {
            let comma_ = BoxDoc::text(",").flat_alt(BoxDoc::nil());
            let sep = BoxDoc::text(",").append(BoxDoc::line());
            let inner = format_inner(sep, comma_);
            BoxDoc::text("(")
                .append(
                    BoxDoc::line_()
                        .append(inner)
                        .nest(2)
                        .append(BoxDoc::line_())
                        .group(),
                )
                .append(")")
        }
        FoldStyle::Never => {
            let sep = BoxDoc::text(",").append(BoxDoc::hardline());
            let comma_ = BoxDoc::text(",");
            let inner = format_inner(sep, comma_);
            BoxDoc::text("(")
                .append(BoxDoc::hardline().append(inner).nest(2))
                .append(BoxDoc::hardline())
                .append(")")
        }
    }
}
