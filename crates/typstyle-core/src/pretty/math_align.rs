use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

use crate::ext::StrExt;

use super::{doc_ext::AllocExt, ArenaDoc, Context, PrettyPrinter};

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

        let (printed, col_widths) = self.render_cells_in_aligned(ctx, aligned_elems)?;
        let doc = self.print_aligned_cells(printed, col_widths);
        Some(doc)
    }

    #[allow(clippy::type_complexity)]
    fn render_cells_in_aligned(
        &'a self,
        ctx: Context,
        aligned_elems: Vec<Vec<Vec<&'a SyntaxNode>>>,
    ) -> Option<(Vec<Vec<(String, usize)>>, Vec<usize>)> {
        // column widths have already considered padding around separators

        let col_num = aligned_elems.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut col_widths = vec![0; col_num];

        let mut printed = vec![];
        for row in aligned_elems {
            let mut row_doc = vec![];
            for (j, col) in row.into_iter().enumerate() {
                let cell_doc = self.convert_math_children(ctx, col.into_iter());
                let mut rendered = String::new();
                cell_doc.render_fmt(200, &mut rendered).ok()?;

                let cell_width = if rendered.is_empty() {
                    0
                } else {
                    let render_width = rendered
                        .lines()
                        .map(|line| line.trim_ascii().width())
                        .max()
                        .unwrap_or(0);
                    if j == 0 || j + 1 == col_num {
                        render_width + 1
                    } else {
                        render_width + 2
                    }
                };
                col_widths[j] = col_widths[j].max(cell_width);
                row_doc.push((rendered, cell_width));
            }
            printed.push(row_doc);
        }

        Some((printed, col_widths))
    }

    fn print_aligned_cells(
        &'a self,
        printed: Vec<Vec<(String, usize)>>,
        col_widths: Vec<usize>,
    ) -> ArenaDoc<'a> {
        /*
        printed as:
          aa & bbbb && cccc \
        dddd & e    && f    \
         */
        let mut doc = self.arena.nil();

        let row_num = printed.len();
        let col_num = col_widths.len();
        for (i, row) in printed.into_iter().enumerate() {
            let mut row_doc = self.arena.nil();
            let row_len = row.len();
            let mut is_prev_empty = false;
            for (j, (cell, cell_width)) in row.into_iter().enumerate() {
                let col_width = col_widths[j];
                let is_cur_empty = cell_width == 0;

                let padded_cell_doc = if cell.has_linebreak() {
                    let mut indent = col_widths[..j].iter().sum::<usize>() + j;
                    if j > 0 {
                        indent += 1;
                    }
                    self.arena
                        .intersperse(
                            cell.lines()
                                .filter_map(|line| {
                                    let trimmed = line.trim_ascii();
                                    if trimmed.is_empty() {
                                        None
                                    } else {
                                        Some(trimmed)
                                    }
                                })
                                .map(|line| {
                                    let render_width = line.width();
                                    let line_width = if j == 0 || j + 1 == col_num {
                                        render_width + 1
                                    } else {
                                        render_width + 2
                                    };
                                    let pad_spaces = self.arena.spaces(col_width - line_width);
                                    let line_doc = self.arena.text(line.to_string());
                                    #[allow(clippy::if_same_then_else)]
                                    if j % 2 == 1 || col_widths.len() == 1 {
                                        line_doc + pad_spaces
                                    } else {
                                        pad_spaces + line_doc
                                    }
                                }),
                            self.arena.hardline(),
                        )
                        .nest(indent as isize)
                } else {
                    let pad_spaces = self.arena.spaces(col_width - cell_width);
                    let cell_doc = self.arena.text(cell);
                    #[allow(clippy::if_same_then_else)]
                    if j % 2 == 1 || col_widths.len() == 1 {
                        cell_doc + pad_spaces
                    } else {
                        pad_spaces + cell_doc
                    }
                };

                let sep = {
                    let mut sep = self.arena.nil();
                    if j > 0 {
                        if !is_prev_empty {
                            sep += self.arena.space();
                        }
                        sep += self.arena.text("&");
                        if !is_cur_empty {
                            sep += self.arena.space();
                        }
                    }
                    sep
                };

                row_doc += sep + padded_cell_doc;

                is_prev_empty = is_cur_empty;
            }
            if row_len < col_num {
                let mut padding = (col_num - row_len) + col_widths[row_len..].iter().sum::<usize>();
                if !is_prev_empty {
                    padding += 1;
                }
                row_doc += self.arena.spaces(padding);
            }
            doc += row_doc;
            if row_num > 1 {
                doc += self.arena.text(" \\");
            }
            if i + 1 != row_num {
                doc += self.arena.hardline();
            }
        }
        doc
    }
}

fn collect_aligned(math: Math<'_>) -> Vec<Vec<Vec<&SyntaxNode>>> {
    // Helper function to remove trailing spaces from a cell.
    fn trim_trailing_spaces(cell: &mut Vec<&SyntaxNode>) {
        while cell
            .last()
            .is_some_and(|last| last.kind() == SyntaxKind::Space)
        {
            cell.pop();
        }
    }

    // First pass: split all children into lines (split at Linebreak)
    let mut lines: Vec<Vec<&SyntaxNode>> = Vec::new();
    let mut current_line: Vec<&SyntaxNode> = Vec::new();
    for node in math.to_untyped().children() {
        if node.kind() == SyntaxKind::Linebreak {
            lines.push(current_line);
            current_line = Vec::new();
        } else {
            current_line.push(node);
        }
    }
    // Push any remaining nodes as the final line.
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Second pass: for each line, split by MathAlignPoint.
    let mut rows: Vec<Vec<Vec<&SyntaxNode>>> = Vec::new();
    for line in lines {
        let mut row: Vec<Vec<&SyntaxNode>> = Vec::new();
        let mut current_cell: Vec<&SyntaxNode> = Vec::new();
        for node in line {
            if node.kind() == SyntaxKind::MathAlignPoint {
                trim_trailing_spaces(&mut current_cell);
                row.push(current_cell);
                current_cell = Vec::new();
            } else if current_cell.is_empty() && node.kind() == SyntaxKind::Space {
                // Skip leading spaces in a cell.
                continue;
            } else {
                current_cell.push(node);
            }
        }
        trim_trailing_spaces(&mut current_cell);
        row.push(current_cell);
        // Only add the row if it has at least one cell.
        if !row.is_empty() {
            rows.push(row);
        }
    }
    rows
}
