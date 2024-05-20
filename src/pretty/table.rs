use itertools::Itertools;
use pretty::BoxDoc;
use typst_syntax::{ast::*, SyntaxKind};

use crate::PrettyPrinter;

use super::util::func_name;

impl PrettyPrinter {
    pub(super) fn convert_table<'a>(
        &'a self,
        table: FuncCall<'a>,
        columns: usize,
    ) -> BoxDoc<'a, ()> {
        let mut doc = BoxDoc::text("(").append(BoxDoc::hardline());
        for named in table.args().items().filter_map(|node| match node {
            Arg::Named(named) => Some(named),
            _ => None,
        }) {
            doc = doc
                .append(self.convert_named(named))
                .append(BoxDoc::text(","))
                .append(BoxDoc::hardline());
        }
        let cells = table
            .args()
            .to_untyped()
            .children()
            .take_while(|node| node.kind() != SyntaxKind::RightParen)
            .filter_map(|node| node.cast::<Arg>())
            .filter(|node| matches!(node, Arg::Pos(_)));
        for (row_pos, row) in cells.chunks(columns).into_iter().with_position() {
            let mut row_doc = BoxDoc::nil();
            for (pos, cell) in row.with_position() {
                row_doc = row_doc
                    .append(self.convert_arg(cell))
                    .append(BoxDoc::text(","))
                    .append(match pos {
                        itertools::Position::First | itertools::Position::Middle => {
                            BoxDoc::softline()
                        }
                        itertools::Position::Last | itertools::Position::Only => BoxDoc::line_(),
                    })
            }
            doc = doc.append(row_doc.group()).append(
                if matches!(
                    row_pos,
                    itertools::Position::Last | itertools::Position::Only
                ) {
                    BoxDoc::nil()
                } else {
                    BoxDoc::hardline()
                },
            );
        }
        doc.nest(2).append(BoxDoc::hardline()).append(")")
    }
}

fn is_table(node: FuncCall<'_>) -> bool {
    func_name(node) == Some("table") || func_name(node) == Some("grid")
}

fn is_formatable(node: FuncCall<'_>) -> bool {
    // 1. no comments
    // 2. no spread args
    // 3. no named args or named args first then unnamed args
    // node.args()
    // 4. no table/grid.header/footer/vline/hline/cell
    for node in node.args().to_untyped().children() {
        if node.kind() == SyntaxKind::LineComment || node.kind() == SyntaxKind::BlockComment {
            return false;
        }
    }
    let mut pos_arg_index = None;
    for (i, node) in node.args().items().enumerate() {
        match node {
            Arg::Pos(_) => {
                pos_arg_index = Some(i);
                if let Some(_func_call) = node.to_untyped().cast::<FuncCall>() {
                    // TODO: further detect table/grid.header/footer/vline/hline/cell
                    return false;
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
