use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::list::{ListStyle, ListStylist};
use super::mode::Mode;
use super::plain::PlainStylist;
use super::style::FoldStyle;
use super::util::is_only_one_and;
use super::PrettyPrinter;

use super::{
    table,
    util::{get_parenthesized_args_untyped, has_parenthesized_args},
    ArenaDoc,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_func_call(&'a self, func_call: FuncCall<'a>) -> ArenaDoc<'a> {
        if func_call.callee().to_untyped().kind() == SyntaxKind::FieldAccess {
            if let Some(res) = self.try_convert_dot_chain(func_call.to_untyped()) {
                return res;
            }
        }
        self.convert_func_call_plain(func_call)
    }

    pub(super) fn convert_func_call_plain(&'a self, func_call: FuncCall<'a>) -> ArenaDoc<'a> {
        self.convert_expr(func_call.callee())
            + self.convert_func_call_args(func_call, func_call.args())
    }

    fn convert_func_call_args(&'a self, func_call: FuncCall<'a>, args: Args<'a>) -> ArenaDoc<'a> {
        if self.current_mode().is_math() {
            return self.format_disabled(args.to_untyped());
        }
        let _g = self.with_mode(Mode::CodeCont);

        let mut doc = self.arena.nil();
        let has_parenthesized_args = has_parenthesized_args(args);
        if table::is_table(func_call) {
            if let Some(cols) = table::is_formatable_table(func_call) {
                doc += self.convert_table(func_call, cols);
            } else if has_parenthesized_args {
                doc += self.convert_parenthesized_args_as_is(args);
            }
        } else if has_parenthesized_args {
            doc += self.convert_parenthesized_args(args);
        };
        doc + self.convert_additional_args(args, has_parenthesized_args)
    }

    pub(super) fn convert_args(&'a self, args: Args<'a>) -> ArenaDoc<'a> {
        let has_parenthesized_args = has_parenthesized_args(args);
        let parenthesized = if has_parenthesized_args {
            self.convert_parenthesized_args(args)
        } else {
            self.arena.nil()
        };
        parenthesized + self.convert_additional_args(args, has_parenthesized_args)
    }

    pub(super) fn convert_parenthesized_args(&'a self, args: Args<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        let mut fold_style = self.get_fold_style(args);

        let children = || {
            args.to_untyped()
                .children()
                .take_while(|it| it.kind() != SyntaxKind::RightParen)
        };
        let arg_count = children().filter(|it| it.is::<Arg>()).count();

        is_only_one_and(args.items().take(arg_count), |arg| {
            let inner = match arg {
                Arg::Pos(p) => *p,
                Arg::Named(_) => {
                    fold_style = FoldStyle::Fit;
                    return false;
                }
                Arg::Spread(s) => s.expr(),
            };
            fold_style = if matches!(
                inner,
                Expr::FuncCall(_) | Expr::FieldAccess(_) | Expr::Unary(_) | Expr::Binary(_)
            ) {
                FoldStyle::Fit
            } else {
                FoldStyle::Always
            };
            true
        });

        ListStylist::new(self)
            .keep_linebreak(self.config.blank_lines_upper_bound)
            .with_fold_style(fold_style)
            .process_iterable_impl(children(), |child| {
                // We should ignore additional args here.
                child.cast().map(|arg| self.convert_arg(arg))
            })
            .print_doc(ListStyle {
                ..Default::default()
            })
    }

    fn convert_parenthesized_args_as_is(&'a self, args: Args<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        let inner = PlainStylist::new(self)
            .process_iterable(get_parenthesized_args_untyped(args), |child| {
                self.convert_arg(child)
            })
            .print_doc();
        inner.nest(self.config.tab_spaces as isize).parens()
    }

    /// Handle additional content blocks
    fn convert_additional_args(&'a self, args: Args<'a>, has_paren: bool) -> ArenaDoc<'a> {
        let args = args
            .to_untyped()
            .children()
            .skip_while(|node| {
                if has_paren {
                    node.kind() != SyntaxKind::RightParen
                } else {
                    node.kind() != SyntaxKind::ContentBlock
                }
            })
            .filter_map(|node| node.cast::<ContentBlock>());
        self.arena
            .concat(args.map(|arg| self.convert_content_block(arg)))
            .group()
    }

    pub(super) fn convert_arg(&'a self, arg: Arg<'a>) -> ArenaDoc<'a> {
        match arg {
            Arg::Pos(p) => self.convert_expr(p),
            Arg::Named(n) => self.convert_named(n),
            Arg::Spread(s) => self.convert_spread(s),
        }
    }
}
