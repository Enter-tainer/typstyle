use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{
    layout::{
        flow::FlowItem,
        list::{ListStyle, ListStylist},
        plain::PlainStylist,
    },
    style::FoldStyle,
    table,
    util::{get_parenthesized_args_untyped, has_parenthesized_args, is_only_one_and},
    ArenaDoc, Context, Mode, PrettyPrinter,
};
use crate::ext::StrExt;

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_func_call(
        &'a self,
        ctx: Context,
        func_call: FuncCall<'a>,
    ) -> ArenaDoc<'a> {
        if func_call.callee().to_untyped().kind() == SyntaxKind::FieldAccess {
            if let Some(res) = self.try_convert_dot_chain(ctx, func_call.to_untyped()) {
                return res;
            }
        }
        self.convert_func_call_plain(ctx, func_call)
    }

    pub(super) fn convert_func_call_plain(
        &'a self,
        ctx: Context,
        func_call: FuncCall<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_expr(ctx, func_call.callee())
            + self.convert_func_call_args(ctx, func_call, func_call.args())
    }

    fn convert_func_call_args(
        &'a self,
        ctx: Context,
        func_call: FuncCall<'a>,
        args: Args<'a>,
    ) -> ArenaDoc<'a> {
        if ctx.mode.is_math() {
            return self.convert_args_in_math(ctx, args);
        };

        let mut doc = self.arena.nil();
        let has_parenthesized_args = has_parenthesized_args(args);
        if table::is_table(func_call) {
            if let Some(cols) = table::is_formatable_table(func_call) {
                doc += self.convert_table(ctx, func_call, cols);
            } else if has_parenthesized_args {
                doc += self.convert_parenthesized_args_as_list(ctx, args);
            }
        } else if has_parenthesized_args {
            doc += self.convert_parenthesized_args(ctx, args);
        };
        doc + self.convert_additional_args(ctx, args, has_parenthesized_args)
    }

    pub(super) fn convert_args(&'a self, ctx: Context, args: Args<'a>) -> ArenaDoc<'a> {
        let has_parenthesized_args = has_parenthesized_args(args);
        let parenthesized = if has_parenthesized_args {
            self.convert_parenthesized_args(ctx, args)
        } else {
            self.arena.nil()
        };
        parenthesized + self.convert_additional_args(ctx, args, has_parenthesized_args)
    }

    pub(super) fn convert_parenthesized_args(
        &'a self,
        ctx: Context,
        args: Args<'a>,
    ) -> ArenaDoc<'a> {
        let ctx = ctx.with_mode(Mode::CodeCont);

        let mut fold_style = self.get_fold_style(ctx, args);

        let children = || {
            args.to_untyped()
                .children()
                .take_while(|it| it.kind() != SyntaxKind::RightParen)
        };
        let arg_count = children().filter(|it| it.is::<Arg>()).count();

        if !ctx.break_suppressed {
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
        }

        ListStylist::new(self)
            .keep_linebreak(self.config.blank_lines_upper_bound)
            .with_fold_style(fold_style)
            .process_iterable_impl(ctx, children(), |ctx, child| {
                // We should ignore additional args here.
                child.cast().map(|arg| self.convert_arg(ctx, arg))
            })
            .print_doc(ListStyle {
                ..Default::default()
            })
    }

    fn convert_parenthesized_args_as_list(&'a self, ctx: Context, args: Args<'a>) -> ArenaDoc<'a> {
        let ctx = ctx.with_mode(Mode::CodeCont);

        let inner = PlainStylist::new(self)
            .process_iterable(ctx, get_parenthesized_args_untyped(args), |ctx, child| {
                self.convert_arg(ctx, child)
            })
            .print_doc();
        inner.nest(self.config.tab_spaces as isize).parens()
    }

    fn convert_args_in_math(&'a self, ctx: Context, args: Args<'a>) -> ArenaDoc<'a> {
        let ctx = ctx.aligned();

        // strip spaces
        let children = {
            let children = args.to_untyped().children().as_slice();
            let i = children
                .iter()
                .position(|child| {
                    !matches!(child.kind(), SyntaxKind::LeftParen | SyntaxKind::Space)
                })
                .unwrap_or(0);
            let j = children
                .iter()
                .rposition(|child| {
                    !matches!(child.kind(), SyntaxKind::RightParen | SyntaxKind::Space)
                })
                .unwrap_or(children.len().saturating_sub(1));
            children[i..=j].iter()
        };

        let mut peek_hashed_arg = false;
        let inner = self.convert_flow_like_iter(ctx, children, |ctx, child| {
            let at_hashed_arg = peek_hashed_arg;
            peek_hashed_arg = false;
            match child.kind() {
                SyntaxKind::Comma => FlowItem::tight_spaced(self.arena.text(",")),
                SyntaxKind::Semicolon => {
                    // We should avoid the semicolon counted the terminator of the previous hashed arg.
                    FlowItem::new(self.arena.text(";"), at_hashed_arg, true)
                }
                SyntaxKind::Space => {
                    peek_hashed_arg = at_hashed_arg;
                    if child.text().has_linebreak() {
                        FlowItem::tight(self.arena.hardline())
                    } else {
                        FlowItem::none()
                    }
                }
                _ => {
                    if let Some(arg) = child.cast::<Arg>() {
                        if is_ends_with_hashed_expr(arg.to_untyped().children()) {
                            peek_hashed_arg = true;
                        }
                        FlowItem::spaced(self.convert_arg(ctx, arg))
                    } else {
                        FlowItem::none()
                    }
                }
            }
        });
        if self.attr_store.is_multiline(args.to_untyped()) {
            ((self.arena.line_() + inner).nest(self.config.tab_spaces as isize)
                + self.arena.line_())
            .group()
            .parens()
        } else {
            inner.parens()
        }
    }

    /// Handle additional content blocks
    fn convert_additional_args(
        &'a self,
        ctx: Context,
        args: Args<'a>,
        has_paren: bool,
    ) -> ArenaDoc<'a> {
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
            .concat(args.map(|arg| self.convert_content_block(ctx, arg)))
    }

    pub(super) fn convert_arg(&'a self, ctx: Context, arg: Arg<'a>) -> ArenaDoc<'a> {
        match arg {
            Arg::Pos(p) => self.convert_expr(ctx, p),
            Arg::Named(n) => self.convert_named(ctx, n),
            Arg::Spread(s) => self.convert_spread(ctx, s),
        }
    }
}

fn is_ends_with_hashed_expr(mut children: std::slice::Iter<'_, SyntaxNode>) -> bool {
    children.next_back().is_some_and(|it| it.is::<Expr>())
        && children
            .next_back()
            .is_some_and(|it| it.kind() == SyntaxKind::Hash)
}
