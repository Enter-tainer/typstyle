use itertools::Itertools;
use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{
    util::{func_name, indent_func_name},
    ArenaDoc,
};
use crate::{
    pretty::{util::get_parenthesized_args, Mode},
    PrettyPrinter,
};

const BLACK_LIST: [&str; 6] = [
    "table.cell",
    "table.vline",
    "table.hline",
    "grid.cell",
    "grid.vline",
    "grid.hline",
];

const HEADER_FOOTER: [&str; 4] = ["table.header", "table.footer", "grid.header", "grid.footer"];

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_table(&'a self, table: FuncCall<'a>, columns: usize) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        let mut doc = self.arena.hardline();
        for named in table.args().items().filter_map(|node| match node {
            Arg::Named(named) => Some(named),
            _ => None,
        }) {
            doc += self.convert_named(named) + "," + self.arena.hardline();
        }
        #[derive(Debug)]
        struct Row<'a> {
            cells: Vec<Arg<'a>>,
        }

        let pos_args = table
            .args()
            .to_untyped()
            .children()
            .take_while(|node| node.kind() != SyntaxKind::RightParen)
            .filter_map(|node| node.cast::<Arg>())
            .filter(|node| matches!(node, Arg::Pos(_)));
        let has_predecessor = |pos: &itertools::Position| {
            matches!(
                pos,
                itertools::Position::Middle | itertools::Position::First
            )
        };
        let table: Vec<Row> = {
            let mut table = Vec::new();
            let mut row = Row {
                cells: Vec::with_capacity(columns),
            };
            for arg in pos_args {
                row.cells.push(arg);
                if row.cells.len() == columns {
                    table.push(row);
                    row = Row {
                        cells: Vec::with_capacity(columns),
                    };
                }
                if let Some(func_call) = arg.to_untyped().cast::<FuncCall>() {
                    if HEADER_FOOTER.contains(&func_name(func_call).as_str()) {
                        table.push(row);
                        row = Row {
                            cells: Vec::with_capacity(columns),
                        };
                    }
                }
            }
            if !row.cells.is_empty() {
                table.push(row);
            }
            table
        };
        for (row_pos, row) in table.into_iter().with_position() {
            let mut row_doc = self.arena.nil();
            for (pos, cell) in row.cells.into_iter().with_position() {
                row_doc = row_doc
                    + self.convert_arg(cell)
                    + self.arena.text(",")
                    + (if has_predecessor(&pos) {
                        self.arena.line()
                    } else if has_predecessor(&row_pos) {
                        self.arena.line_()
                    } else {
                        self.arena.nil()
                    });
            }
            doc += row_doc.group()
                + (if has_predecessor(&row_pos) {
                    self.arena.hardline()
                } else {
                    self.arena.nil()
                });
        }
        (doc.nest(self.config.tab_spaces as isize) + self.arena.hardline()).parens()
    }
}

pub fn is_table(node: FuncCall<'_>) -> bool {
    indent_func_name(node) == Some("table") || indent_func_name(node) == Some("grid")
}

fn is_formatable(node: FuncCall<'_>) -> bool {
    // 1. no comments
    // 2. no spread args
    // 3. no named args or named args first then unnamed args
    // 4. has at least one pos arg
    // 5. no table/grid.vline/hline/cell
    // 6. if table/grid.header/footer present, they should appear before/after any unnamed args
    for node in node.args().to_untyped().children() {
        if node.kind() == SyntaxKind::LineComment || node.kind() == SyntaxKind::BlockComment {
            return false;
        }
    }
    let mut pos_arg_index = None;
    for (i, node) in get_parenthesized_args(node.args()).enumerate() {
        match node {
            Arg::Pos(_) => {
                pos_arg_index = Some(i);
                if let Some(func_call) = node.to_untyped().cast::<FuncCall>() {
                    if BLACK_LIST.contains(&func_name(func_call).as_str()) {
                        return false;
                    }
                }
            }
            Arg::Named(_) => {
                if pos_arg_index.is_some() {
                    return false;
                }
            }
            Arg::Spread(_) => return false,
        }
    }
    if pos_arg_index.is_none() {
        return false;
    }
    true
}

fn get_table_columns(node: FuncCall<'_>) -> Option<usize> {
    for node in node.args().items() {
        if let Arg::Named(name) = node {
            if name.name().as_str() == "columns" {
                if let Some(count) = name.expr().to_untyped().cast::<Int>() {
                    return Some(count.get() as usize);
                }
                if let Some(arr) = name.expr().to_untyped().cast::<Array>() {
                    return Some(arr.items().count());
                }
            }
        }
    }
    None
}

/// Returns the number of columns in the table if the table is formatable.
/// Otherwise, returns None.
pub(super) fn is_formatable_table(node: FuncCall<'_>) -> Option<usize> {
    if is_table(node) && is_formatable(node) {
        get_table_columns(node)
    } else {
        None
    }
}
