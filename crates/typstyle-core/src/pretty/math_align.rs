use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

use super::{context::AlignMode, doc_ext::AllocExt, ArenaDoc, Context, PrettyPrinter};
use crate::{ext::StrExt, AttrStore};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn try_convert_math_aligned(
        &'a self,
        ctx: Context,
        math: Math<'a>,
    ) -> Option<ArenaDoc<'a>> {
        if ctx.align_mode == AlignMode::Never
            || !self.attr_store.has_math_align_point(math.to_untyped())
        {
            return None;
        }
        let ctx = ctx.aligned(AlignMode::Outer);
        let aligned_elems = collect_aligned(math, &self.attr_store);
        let aligned = self.render_cells_in_aligned(ctx, aligned_elems)?;

        let doc = self.print_aligned_cells(aligned, ctx.align_mode == AlignMode::Outer);
        Some(doc)
    }

    fn render_cells_in_aligned(
        &'a self,
        ctx: Context,
        aligned_elems: Vec<RawRow<'a>>,
    ) -> Option<Aligned<'a>> {
        // column widths have already considered padding around separators

        let col_num = aligned_elems.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut col_widths = vec![0; col_num];
        let mut grid_width = col_num;
        if grid_width > self.config.max_width {
            return None;
        }

        let mut printed = vec![];
        for row in aligned_elems {
            let rendered_row = match row {
                RawRow::Comment(comment) => Row::Comment(comment.text()),
                RawRow::Cells(items) => {
                    let mut row_doc = vec![];
                    for (j, cell) in items.into_iter().enumerate() {
                        let ends_with_line_comment = cell
                            .last()
                            .is_some_and(|last| last.kind() == SyntaxKind::LineComment);
                        let cell_doc = self.convert_math_children(ctx, cell.into_iter());
                        let mut rendered = String::new();
                        cell_doc
                            .render_fmt(self.config.max_width, &mut rendered)
                            .ok()?;
                        if ends_with_line_comment {
                            rendered.push_str("\n\n"); // ensure an extra line is added
                        }

                        let rendered_cell = if rendered.is_empty() {
                            Cell::Empty
                        } else if rendered.has_linebreak() {
                            Cell::MultiLine(
                                rendered
                                    .lines()
                                    .map(|line| {
                                        let render_width = line.width();
                                        let line_width = if j == 0 || j + 1 == col_num {
                                            render_width + 1
                                        } else {
                                            render_width + 2
                                        };
                                        (line.to_string(), line_width)
                                    })
                                    .collect(),
                            )
                        } else {
                            let render_width = rendered.width();
                            let line_width = if j == 0 || j + 1 == col_num {
                                render_width + 1
                            } else {
                                render_width + 2
                            };
                            Cell::SingleLine(rendered, line_width)
                        };
                        let cell_width = rendered_cell.width();
                        if cell_width > col_widths[j] {
                            grid_width += cell_width - col_widths[j];
                            col_widths[j] = cell_width;
                            if grid_width > self.config.max_width {
                                return None; // exceeds max width
                            }
                        }

                        row_doc.push(rendered_cell);
                    }
                    Row::Cells(row_doc)
                }
            };
            printed.push(rendered_row);
        }

        Some(Aligned {
            rows: printed,
            col_widths,
        })
    }

    fn print_aligned_cells(
        &'a self,
        aligned: Aligned<'a>,
        add_trailing_linebreak: bool,
    ) -> ArenaDoc<'a> {
        /*
        printed as:
          aa & bbbb && cccc \
        dddd & e    && f    \
         */
        let printed = aligned.rows;
        let col_widths = aligned.col_widths;
        let row_num = printed.len();
        let col_num = col_widths.len();

        let mut doc = self.arena.nil();

        for (i, row) in printed.into_iter().enumerate() {
            let mut row_doc = self.arena.nil();
            match row {
                Row::Comment(cmt) => {
                    row_doc = self.arena.text(cmt) + self.arena.hardline();
                }
                Row::Cells(row) => {
                    let row_len = row.len();
                    let mut is_prev_empty = false;
                    for (j, cell) in row.into_iter().enumerate() {
                        let col_width = col_widths[j];
                        let mut is_cur_empty = false;

                        let pad = |cell_doc: ArenaDoc<'a>, width: usize| {
                            let pad_spaces = self.arena.spaces(col_width - width);
                            #[allow(clippy::if_same_then_else)]
                            if j % 2 == 1 || col_widths.len() == 1 {
                                cell_doc + pad_spaces
                            } else {
                                pad_spaces + cell_doc
                            }
                        };

                        let padded_cell_doc = match cell {
                            Cell::Empty => {
                                is_cur_empty = true;
                                pad(self.arena.nil(), 0)
                            }
                            Cell::SingleLine(line, width) => pad(self.arena.text(line), width),
                            Cell::MultiLine(lines) => {
                                let mut indent = col_widths[..j].iter().sum::<usize>() + j;
                                if j > 0 {
                                    indent += 1;
                                }
                                self.arena
                                    .intersperse(
                                        lines
                                            .into_iter()
                                            .map(|(line, width)| pad(self.arena.text(line), width)),
                                        self.arena.hardline(),
                                    )
                                    .nest(indent as isize)
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
                        let mut padding =
                            (col_num - row_len) + col_widths[row_len..].iter().sum::<usize>();
                        if !is_prev_empty {
                            padding += 1;
                        }
                        row_doc += self.arena.spaces(padding);
                    }
                    if row_num > 1 {
                        row_doc += if add_trailing_linebreak || i + 1 != row_num {
                            self.arena.text(" \\")
                        } else {
                            self.arena.text(" ")
                        };
                    }
                    if i + 1 != row_num {
                        row_doc += self.arena.hardline();
                    }
                }
            }

            doc += row_doc;
        }
        doc
    }
}

struct Aligned<'a> {
    rows: Vec<Row<'a>>,
    col_widths: Vec<usize>,
}

enum Row<'a> {
    Cells(Vec<Cell>),
    Comment(&'a str),
}

enum Cell {
    Empty,
    SingleLine(String, usize),
    MultiLine(Vec<(String, usize)>),
}

impl Cell {
    pub fn width(&self) -> usize {
        match self {
            Cell::Empty => 0,
            Cell::SingleLine(_, width) => *width,
            Cell::MultiLine(lines) => lines.iter().map(|(_, width)| *width).max().unwrap_or(0),
        }
    }
}

enum RawRow<'a> {
    Cells(Vec<Vec<&'a SyntaxNode>>),
    Comment(&'a SyntaxNode),
}

impl RawRow<'_> {
    pub fn len(&self) -> usize {
        match self {
            RawRow::Cells(items) => items.len(),
            RawRow::Comment(_) => 0,
        }
    }
}

fn collect_aligned<'a>(math: Math<'a>, attrs: &AttrStore) -> Vec<RawRow<'a>> {
    // Helper function to remove trailing spaces from a cell.
    fn trim_trailing_spaces(cell: &mut Vec<&SyntaxNode>) {
        while cell
            .last()
            .is_some_and(|last| last.kind() == SyntaxKind::Space)
        {
            cell.pop();
        }
    }

    fn collect_children<'a>(
        node: &'a SyntaxNode,
        attrs: &AttrStore,
        out: &mut Vec<&'a SyntaxNode>,
    ) {
        if !(matches!(node.kind(), SyntaxKind::Math | SyntaxKind::MathDelimited)
            && attrs.has_math_align_point(node))
        {
            out.push(node);
            return;
        }
        for child in node.children() {
            collect_children(child, attrs, out);
        }
    }

    let mut flattened_children = Vec::new();
    collect_children(math.to_untyped(), attrs, &mut flattened_children);

    // First pass: split all children into lines (split at Linebreak)
    let mut lines: Vec<Vec<&SyntaxNode>> = Vec::new();
    let mut current_line: Vec<&SyntaxNode> = Vec::new();
    for node in flattened_children {
        if node.kind() == SyntaxKind::Linebreak {
            lines.push(current_line);
            current_line = Vec::new();
        } else {
            current_line.push(node);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Second pass: create rows; if a line starts with a line comment, create a Comment row.
    let mut rows = Vec::new();
    for line in lines {
        let mut cells = Vec::new();
        let mut current_cell = Vec::new();
        for node in line {
            match node.kind() {
                SyntaxKind::MathAlignPoint => {
                    trim_trailing_spaces(&mut current_cell);
                    cells.push(current_cell);
                    current_cell = Vec::new();
                }
                SyntaxKind::Space if current_cell.is_empty() => {}
                SyntaxKind::LineComment
                    if node.kind() == SyntaxKind::LineComment
                        && cells.is_empty()
                        && current_cell.is_empty() =>
                {
                    rows.push(RawRow::Comment(node));
                }
                _ => {
                    current_cell.push(node);
                }
            }
        }
        trim_trailing_spaces(&mut current_cell);
        cells.push(current_cell);
        if !cells.is_empty() {
            rows.push(RawRow::Cells(cells));
        }
    }
    rows
}
