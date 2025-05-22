use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use super::{
    context::AlignMode,
    layout::{
        flow::FlowItem,
        list::{ListStyle, ListStylist},
        plain::PlainStylist,
    },
    prelude::*,
    style::FoldStyle,
    table,
    util::{get_parenthesized_args, get_parenthesized_args_untyped, has_parenthesized_args},
    Context, Mode, PrettyPrinter,
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

    pub(super) fn convert_func_call_as_table(
        &'a self,
        ctx: Context,
        func_call: FuncCall<'a>,
        columns: usize,
    ) -> ArenaDoc<'a> {
        let args = func_call.args();
        let has_parenthesized_args = has_parenthesized_args(args);
        self.convert_expr(ctx, func_call.callee())
            + self.convert_table(ctx, func_call, columns)
            + self.convert_additional_args(ctx, args, has_parenthesized_args)
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
            if let Some(table) = self.try_convert_table(ctx, func_call) {
                doc += table;
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

        let get_children = || {
            args.to_untyped()
                .children()
                .take_while(|it| it.kind() != SyntaxKind::RightParen)
        };
        let arg_count = get_children()
            .filter(|it| SyntaxNode::is::<Arg>(it))
            .count(); // should exclude args in brackets

        // if there is only one blocky arg, and it's the last one, we can use compact style
        let is_blocky = |expr| {
            matches!(
                expr,
                Expr::Code(_)
                    | Expr::Conditional(_)
                    | Expr::While(_)
                    | Expr::For(_)
                    | Expr::Contextual(_)
                    | Expr::Closure(_)
            )
        };
        // if there is only one arg, and it's combinable, we can use compact style
        let is_combinable = |arg| {
            is_blocky(arg)
                || matches!(
                    arg,
                    Expr::FuncCall(_)
                        | Expr::Parenthesized(_)
                        | Expr::Content(_)
                        | Expr::Array(_)
                        | Expr::Dict(_)
                )
        };

        let fold_style = match self.get_fold_style(ctx, args) {
            FoldStyle::Always => FoldStyle::Always,
            _ if ctx.break_suppressed && arg_count == 1 => FoldStyle::Always,
            _ if ctx.break_suppressed => FoldStyle::Fit,
            _ => {
                let mut fold_style = FoldStyle::Fit;
                let mut has_initial_array = false;
                let mut has_initial_dict = false;
                for (i, arg) in get_parenthesized_args(args).enumerate() {
                    let expr = {
                        let mut expr = match arg {
                            Arg::Pos(p) => p,
                            Arg::Named(n) => n.expr(),
                            Arg::Spread(s) => s.expr(),
                        };
                        while let Expr::Parenthesized(p) = expr {
                            expr = p.expr();
                        }
                        expr
                    };

                    if i < arg_count - 1 {
                        has_initial_array |= matches!(expr, Expr::Array(_));
                        has_initial_dict |= matches!(expr, Expr::Dict(_));
                        if is_blocky(expr) {
                            break;
                        } else {
                            continue;
                        }
                    }
                    if is_combinable(expr)
                        && !(has_initial_array && matches!(expr, Expr::Array(_))
                            || has_initial_dict && matches!(expr, Expr::Dict(_)))
                    {
                        fold_style = if arg_count == 1 && !matches!(expr, Expr::FuncCall(_)) {
                            FoldStyle::Always
                        } else {
                            FoldStyle::Compact
                        }
                    }
                }
                fold_style
            }
        };

        ListStylist::new(self)
            .keep_linebreak(self.config.blank_lines_upper_bound)
            .with_fold_style(fold_style)
            .process_iterable_impl(ctx, get_children(), |ctx, child| {
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
        // strip spaces
        let mut peek_linebreak = false;
        let children = {
            let children = args.to_untyped().children().as_slice();
            let i = children
                .iter()
                .position(|child| {
                    if child.kind() == SyntaxKind::Space {
                        peek_linebreak = child.text().has_linebreak();
                    }
                    !matches!(child.kind(), SyntaxKind::LeftParen | SyntaxKind::Space)
                })
                .expect("invariant: args should have right paren");
            let j = children
                .iter()
                .rposition(|child| {
                    !matches!(child.kind(), SyntaxKind::RightParen | SyntaxKind::Space)
                })
                .expect("invariant: args should have left paren");
            if i > j {
                children[0..0].iter()
            } else {
                children[i..=j].iter()
            }
        };

        let mut peek_hashed_arg = false;
        let inner = self.convert_flow_like_iter(ctx, children, |ctx, child, _| {
            let at_hashed_arg = peek_hashed_arg;
            let at_linebreak = peek_linebreak;
            peek_hashed_arg = false;
            peek_linebreak = false;
            match child.kind() {
                SyntaxKind::Comma => FlowItem::tight_spaced(self.arena.text(",")),
                SyntaxKind::Semicolon => {
                    // We should avoid the semicolon counted the terminator of the previous hashed arg.
                    FlowItem::new(self.arena.text(";"), at_hashed_arg, true)
                }
                SyntaxKind::Space => {
                    peek_hashed_arg = at_hashed_arg;
                    if child.text().has_linebreak() {
                        peek_linebreak = true;
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
                        let ctx = ctx.aligned(
                            if at_linebreak || arg.to_untyped().kind() == SyntaxKind::MathDelimited
                            {
                                AlignMode::Inner
                            } else {
                                AlignMode::Never
                            },
                        );
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
