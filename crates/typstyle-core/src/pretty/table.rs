use typst_syntax::{ast::*, SyntaxKind};

use super::{
    prelude::*,
    util::{func_name, get_parenthesized_args_untyped},
    Context,
};
use crate::{
    ext::StrExt,
    pretty::{layout::table::TableCollector, util::get_parenthesized_args, Mode},
    PrettyPrinter,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn try_convert_table(
        &'a self,
        ctx: Context,
        table: FuncCall<'a>,
    ) -> Option<ArenaDoc<'a>> {
        let columns = if is_table(table) && is_table_formattable(table) {
            get_table_columns(table)
        } else {
            None
        }?;
        Some(self.convert_table(ctx, table, columns))
    }

    /// Handle parenthesized args of a table.
    pub(super) fn convert_table(
        &'a self,
        ctx: Context,
        table: FuncCall<'a>,
        columns: usize,
    ) -> ArenaDoc<'a> {
        let ctx = ctx.with_mode(Mode::CodeCont);

        // Rules:
        // - named/spread args, header/footer: occupy a line.
        // - reflow cells if no special cells (cell, hline, vline, )
        // - hard break at linebreaks with at least 1 empty lines
        let can_reflow_cells = table.args().items().any(is_special_cell);
        let mut collector =
            TableCollector::new(&self.arena, if can_reflow_cells { 0 } else { columns });

        for node in get_parenthesized_args_untyped(table.args()) {
            if let Some(arg) = node.cast::<Arg>() {
                match arg {
                    Arg::Pos(Expr::FuncCall(func_call)) if is_header_footer(func_call) => {
                        collector
                            .push_row(self.convert_func_call_as_table(ctx, func_call, columns));
                    }
                    Arg::Pos(expr) => {
                        collector.push_cell(self.convert_expr(ctx, expr));
                    }
                    Arg::Named(named) => {
                        collector.push_row(self.convert_named(ctx, named));
                    }
                    Arg::Spread(spread) => {
                        // NOTE: when spread exists, regarding it as a cell will not affect layout.
                        collector.push_cell(self.convert_spread(ctx, spread));
                    }
                }
            } else if node.kind() == SyntaxKind::Space {
                collector.push_newline(node.text().count_linebreaks());
            } else if node.kind() == SyntaxKind::LineComment {
                collector.push_comment(self.convert_comment(ctx, node));
            };
        }
        let doc = collector.collect();
        doc.enclose(self.arena.line_(), self.arena.line_())
            .group()
            .nest(self.config.tab_spaces as isize)
            .parens()
    }
}

pub fn is_table(func_call: FuncCall<'_>) -> bool {
    matches!(func_name(func_call), Some("table") | Some("grid"))
}

fn is_table_formattable(func_call: FuncCall<'_>) -> bool {
    // 1. no block comments
    // 2. has at least one pos arg
    if func_call
        .args()
        .to_untyped()
        .children()
        .any(|it| matches!(it.kind(), SyntaxKind::BlockComment))
    {
        return false;
    }
    get_parenthesized_args(func_call.args()).any(|it| matches!(it, Arg::Pos(_)))
}

fn get_table_columns(func_call: FuncCall<'_>) -> Option<usize> {
    let Some(columns_expr) = func_call.args().items().find_map(|node| {
        if let Arg::Named(named) = node {
            if named.name().as_str() == "columns" {
                return Some(named.expr());
            }
        }
        None
    }) else {
        return if (func_call.args().items()).any(|arg| matches!(arg, Arg::Spread(_))) {
            None // the columns may be provided in spread args.
        } else {
            Some(1) // if not `columns` is provided, regard as 1.
        };
    };
    match columns_expr {
        Expr::Auto(_) => Some(1),
        Expr::Int(int) => Some(int.get() as usize),
        Expr::Array(array) => Some(array.items().count()),
        _ => None,
    }
}

fn is_header_footer(func_call: FuncCall) -> bool {
    const HEADER_FOOTER: &[&str] = &["header", "footer"];

    func_name(func_call).is_some_and(|name| HEADER_FOOTER.contains(&name))
}

fn is_special_cell(arg: Arg) -> bool {
    const BLACK_LIST: &[&str] = &["cell", "vline", "hline"];

    match arg {
        Arg::Pos(Expr::FuncCall(func_call)) => {
            func_name(func_call).is_some_and(|name| BLACK_LIST.contains(&name))
        }
        Arg::Spread(_) => true,
        _ => false,
    }
}
