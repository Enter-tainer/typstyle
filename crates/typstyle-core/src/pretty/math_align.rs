use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

use super::{ArenaDoc, Context, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn try_convert_math_aligned(
        &'a self,
        ctx: Context,
        math: Math<'a>,
    ) -> Option<ArenaDoc<'a>> {
        if ctx.in_aligned
            || math
                .to_untyped()
                .children()
                .any(|it| it.kind() == SyntaxKind::LineComment)
            || !math
                .exprs()
                .any(|expr| matches!(expr, Expr::MathAlignPoint(_)))
        {
            return None;
        }

        let ctx = ctx.aligned();
        let aligned_elems = collect_aligned(math);

        let (printed, col_widths) = {
            let col_num = aligned_elems.iter().map(|row| row.len()).max().unwrap_or(0);
            let mut col_widths = vec![0; col_num];

            let mut printed = vec![];
            for row in aligned_elems {
                let mut row_doc = vec![];
                for (j, col) in row.into_iter().enumerate() {
                    let cell_doc = self.convert_math_children(ctx, col.into_iter());
                    let mut rendered = String::new();
                    cell_doc.render_fmt(200, &mut rendered).ok()?;
                    if rendered.contains('\n') {
                        return None; // linebreaks not supported yet
                    }
                    col_widths[j] = col_widths[j].max(rendered.width());
                    row_doc.push(rendered);
                }
                printed.push(row_doc);
            }
            (printed, col_widths)
        };

        let mut doc = self.arena.nil();

        /*
        printed as:
          aa & bbbb & cccc \
        dddd & e    & f    \
         */
        let row_num = printed.len();
        for (i, row) in printed.into_iter().enumerate() {
            let mut row_doc = self.arena.nil();
            let col_num = row.len();
            for (j, cell) in row.into_iter().enumerate() {
                let col_width = col_widths[j];
                let pad_spaces = self.arena.text(" ".repeat(col_width - cell.width()));
                let cell_doc = self.arena.text(cell);
                #[allow(clippy::if_same_then_else)]
                let padded_cell_doc = if j % 2 == 1 || col_widths.len() == 1 {
                    cell_doc + pad_spaces
                } else {
                    pad_spaces + cell_doc
                };
                if j > 0 {
                    row_doc += self.arena.text(" & ");
                }
                row_doc += padded_cell_doc;
            }
            for &w in col_widths[col_num..].iter() {
                row_doc += self.arena.text(" ".repeat(w + 3));
            }
            doc += row_doc;
            if row_num > 1 {
                // ~~add a space to avoid escaping a following char~~
                doc += self.arena.text(" \\");
            }
            if i + 1 != row_num {
                doc += self.arena.hardline();
            }
        }

        Some(doc)
    }
}

fn collect_aligned(math: Math<'_>) -> Vec<Vec<Vec<&SyntaxNode>>> {
    let mut rows = vec![];
    let mut cur_row = vec![];
    let mut cur_cell: Vec<&SyntaxNode> = vec![];

    fn push_cell<'a>(cur_row: &mut Vec<Vec<&'a SyntaxNode>>, mut cur_cell: Vec<&'a SyntaxNode>) {
        while cur_cell
            .last()
            .is_some_and(|last| last.kind() == SyntaxKind::Space)
        {
            cur_cell.pop();
        }
        cur_row.push(cur_cell);
    }

    for node in math.to_untyped().children() {
        match node.kind() {
            SyntaxKind::Linebreak => {
                push_cell(&mut cur_row, cur_cell);
                cur_cell = vec![];

                rows.push(cur_row);
                cur_row = vec![];
            }
            SyntaxKind::MathAlignPoint => {
                push_cell(&mut cur_row, cur_cell);
                cur_cell = vec![];
            }
            _ if cur_cell.is_empty() && node.kind() == SyntaxKind::Space => {}
            _ => cur_cell.push(node),
        }
    }
    if !cur_cell.is_empty() {
        push_cell(&mut cur_row, cur_cell);
    }
    if !cur_row.is_empty() {
        rows.push(cur_row);
    }
    rows
}
