use pretty::BoxDoc;
use typst_syntax::{ast::*, SyntaxKind};

use crate::{
    util::{comma_seprated_items, FoldStyle},
    PrettyPrinter,
};

use super::{
    table,
    util::{self, get_parenthesized_args},
};

impl PrettyPrinter {
    pub(super) fn convert_func_call<'a>(&'a self, func_call: FuncCall<'a>) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::nil().append(self.convert_expr(func_call.callee()));
        if let Some(res) = self.check_disabled(func_call.args().to_untyped()) {
            return doc.append(res);
        }
        let has_parenthesized_args = util::has_parenthesized_args(func_call);
        if let Some(cols) = table::is_formatable_table(func_call) {
            doc = doc.append(self.convert_table(func_call, cols));
        } else if has_parenthesized_args {
            doc = doc.append(self.convert_parenthesized_args(func_call.args()));
        };
        doc.append(self.convert_additional_args(func_call.args(), has_parenthesized_args))
    }

    pub(super) fn convert_parenthesized_args<'a>(&'a self, args: Args<'a>) -> BoxDoc<'a, ()> {
        let (args, prefer_tighter, is_multiline) = self.convert_parenthesized_args_impl(args);
        let doc = if prefer_tighter {
            BoxDoc::text("(")
                .append(args.into_iter().next().unwrap_or_else(BoxDoc::nil))
                .append(BoxDoc::text(")"))
        } else {
            comma_seprated_items(
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

    fn convert_parenthesized_args_impl<'a>(
        &'a self,
        args: Args<'a>,
    ) -> (Vec<BoxDoc<'a, ()>>, bool, bool) {
        let node = args.to_untyped();
        let mut last_arg = None;
        let mut is_multiline = false;
        for node in node
            .children()
            .take_while(|node| node.kind() != SyntaxKind::RightParen)
        {
            if let Some(space) = node.cast::<Space>() {
                is_multiline = is_multiline || space.to_untyped().text().contains('\n');
                break;
            }
        }
        let args: Vec<BoxDoc<'a, ()>> = get_parenthesized_args(args)
            .map(|arg| {
                last_arg = Some(arg);
                is_multiline = is_multiline
                    || self
                        .attr_map
                        .get(arg.to_untyped())
                        .map_or(false, |attr| attr.is_multiline_flavor());
                self.convert_arg(arg)
            })
            .collect();
        // We prefer tighter style if...
        // 1. There are no arguments
        // 2. There is only one argument and it is not a function call
        let prefer_tighter = args.is_empty()
            || (args.len() == 1 && {
                let arg = last_arg.unwrap();
                let rhs = match arg {
                    Arg::Pos(p) => p,
                    Arg::Named(n) => n.expr(),
                    Arg::Spread(s) => s.expr(),
                };
                !matches!(rhs, Expr::FuncCall(..))
            });
        (args, prefer_tighter, is_multiline)
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
