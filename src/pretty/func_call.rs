use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::list::{ListStyle, ListStylist};
use super::mode::Mode;
use super::plain::PlainStylist;
use super::util::is_only_one_and;
use super::PrettyPrinter;

use super::{
    table,
    util::{get_parenthesized_args_untyped, has_parenthesized_args},
    ArenaDoc,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_func_call(&'a self, func_call: FuncCall<'a>) -> ArenaDoc<'a> {
        if self.current_mode().is_code()
            && func_call.callee().to_untyped().kind() == SyntaxKind::FieldAccess
        {
            return self.convert_dot_chain(func_call.to_untyped());
        }
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
        // let always_fold = is_only_one_and(args.items(), |item| {
        //     matches!(item, Arg::Pos(Expr::Code(_)) | Arg::Pos(Expr::Content(_)))
        // });
        let arg_count = args
            .to_untyped()
            .children()
            .take_while(|it| it.kind() != SyntaxKind::RightParen)
            .filter(|it| it.is::<Arg>())
            .count();
        let always_fold = is_only_one_and(args.items().take(arg_count), |arg| {
            let inner = match arg {
                Arg::Pos(p) => *p,
                Arg::Named(n) => n.expr(),
                Arg::Spread(s) => s.expr(),
            };
            !matches!(inner, Expr::FuncCall(_))
        });
        let mut closed = false;
        ListStylist::new(self)
            .keep_linebreak(self.config.blank_lines_upper_bound)
            .process_list_impl(args.to_untyped(), |child| {
                // We should ignore additional args here.
                if child.kind() == SyntaxKind::RightParen {
                    closed = true;
                } else if !closed {
                    return child.cast().map(|arg| self.convert_arg(arg));
                }
                Option::None
            })
            .always_fold_if(|| always_fold)
            .print_doc(ListStyle {
                ..Default::default()
            })
    }

    pub(super) fn convert_parenthesized_args_as_is(&'a self, args: Args<'a>) -> ArenaDoc<'a> {
        let inner = PlainStylist::new(self)
            .process_iterable(get_parenthesized_args_untyped(args), |child| {
                self.convert_arg(child)
            })
            .print_doc();
        inner.nest(2).parens()
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
