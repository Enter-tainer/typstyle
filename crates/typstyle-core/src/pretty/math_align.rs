use itertools::Itertools;
use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

use super::{context::AlignMode, doc_ext::AllocExt, ArenaDoc, Context, PrettyPrinter};
use crate::{ext::StrExt, AttrStore};

impl<'a> PrettyPrinter<'a> {
    /// Attempt to format a math node as an aligned grid if there are align points.
    pub(super) fn try_convert_math_aligned(
        &'a self,
        ctx: Context,
        math: Math<'a>,
    ) -> Option<ArenaDoc<'a>> {
        // Skip if alignment is disabled or no math align points present
        if ctx.align_mode == AlignMode::Never
            || !self.attr_store.can_align_in_math(math.to_untyped())
        {
            return None;
        }
        let ctx = ctx.aligned(AlignMode::Outer);
        let aligned_elems = collect_aligned(math, &self.attr_store);
        let aligned = self.render_aligned(ctx, aligned_elems)?;

        let doc = self.print_aligned(aligned, ctx.align_mode == AlignMode::Outer);
        Some(doc)
    }

    /// Build aligned rows by measuring each cell and tracking column widths.
    fn render_aligned(
        &'a self,
        ctx: Context,
        aligned_elems: Vec<RawRow<'a>>,
    ) -> Option<Aligned<'a>> {
        // Determine how many columns we need
        let col_num = aligned_elems
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default();

        // Early‑exit if even the empty grid would exceed max width
        if col_num > self.config.max_width {
            return None;
        }

        let mut col_widths = vec![0; col_num];
        let mut grid_width = col_num;

        // Render each raw row into a Row and set column widths
        let rows = (aligned_elems.into_iter()).try_fold(vec![], |mut rows, row| {
            let rendered_row = match row {
                RawRow::Comment(comment) => Row::Comment(comment.text()),
                RawRow::Cells(cells) => {
                    let mut rendered_cells = Vec::with_capacity(cells.len());
                    for (j, cell_nodes) in cells.into_iter().enumerate() {
                        // Render the content of each cell into a string buffer
                        let ends_with_line_comment = cell_nodes
                            .last()
                            .is_some_and(|n| n.kind() == SyntaxKind::LineComment);

                        let mut buf = String::new();
                        self.convert_math_children(ctx, cell_nodes.into_iter())
                            .render_fmt(self.config.max_width, &mut buf)
                            .ok()?;
                        if ends_with_line_comment {
                            buf.push_str("\n "); // ensure an extra line is added
                        }

                        let measure_width = |line: &str| {
                            let render_width = line.width();
                            if j == 0 || j + 1 == col_num {
                                render_width + 1
                            } else {
                                render_width + 2
                            }
                        };

                        let rendered_cell = if buf.is_empty() {
                            Cell::Empty
                        } else if buf.has_linebreak() {
                            Cell::MultiLine(
                                buf.lines()
                                    .map(|line| (line.to_string(), measure_width(line)))
                                    .collect(),
                            )
                        } else {
                            let line_width = measure_width(&buf);
                            Cell::SingleLine(buf, line_width)
                        };

                        // Update col_widths and bail out if we exceed max_width
                        let cell_width = rendered_cell.width();
                        if cell_width > col_widths[j] {
                            grid_width += cell_width - col_widths[j];
                            col_widths[j] = cell_width;
                            if grid_width > self.config.max_width {
                                return None; // bail out
                            }
                        }

                        rendered_cells.push(rendered_cell);
                    }
                    Row::Cells(rendered_cells)
                }
            };
            rows.push(rendered_row);
            Some(rows)
        })?;

        Some(Aligned { rows, col_widths })
    }

    /// Combine aligned cells together, inserting '&', spaces, and linebreaks.
    fn print_aligned(&'a self, aligned: Aligned<'a>, add_trailing_linebreak: bool) -> ArenaDoc<'a> {
        let rows = aligned.rows;
        let col_widths = aligned.col_widths;
        let num_rows = rows.len();
        let num_cols = col_widths.len();
        let col_widths_sum = {
            let mut sums = Vec::with_capacity(num_cols + 1);
            sums.push(0);
            for &width in &col_widths {
                sums.push(sums.last().unwrap() + width);
            }
            sums
        };
        let grid_width = col_widths_sum.last().unwrap() + num_cols;

        enum Alignment {
            Left,
            Right,
        }

        (self.arena).concat(rows.into_iter().enumerate().map(|(i, row)| match row {
            Row::Comment(cmt) => {
                // Emit a full‑line comment followed by a hard linebreak
                self.arena.text(cmt) + self.arena.hardline()
            }
            Row::Cells(cells) => {
                let mut row_doc = self.arena.nil();

                // For each cell: pad to column width and insert separators
                let num_cells = cells.len();
                let mut is_prev_empty = false;
                for (j, cell) in cells.into_iter().enumerate() {
                    let alignment = if j % 2 == 1 || num_cols == 1 {
                        Alignment::Left
                    } else {
                        Alignment::Right
                    };
                    let col_width = col_widths[j];

                    let pad = |cell_doc: ArenaDoc<'a>, width: usize| {
                        let pad_spaces = self.arena.spaces(col_width - width);
                        match alignment {
                            Alignment::Left => cell_doc + pad_spaces,
                            Alignment::Right => pad_spaces + cell_doc,
                        }
                    };

                    let cell_width = cell.width();
                    let (padded_cell_doc, is_cur_empty) = match cell {
                        Cell::Empty => (pad(self.arena.nil(), 0), true),
                        Cell::SingleLine(line, width) => (pad(self.arena.text(line), width), false),
                        Cell::MultiLine(lines) => {
                            let padding_left = match alignment {
                                Alignment::Left => 0,
                                Alignment::Right => col_width - cell_width,
                            };
                            let indent = {
                                let mut indent = col_widths_sum[j] + j + padding_left;
                                if j > 0 {
                                    indent += 1;
                                }
                                indent
                            };

                            let trailing_padding =
                                col_width - padding_left - lines[lines.len() - 1].1;
                            let doc = self.arena.spaces(padding_left)
                                + self.arena.intersperse(
                                    lines.into_iter().map(|(line, _)| line),
                                    self.arena.hardline(),
                                )
                                + self.arena.spaces(trailing_padding);
                            (doc.nest(indent as isize), false)
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

                // If row has fewer cells than columns, add trailing spaces
                if num_cells < num_cols {
                    let mut padding = grid_width - num_cells - col_widths_sum[num_cells];
                    if !is_prev_empty {
                        padding += 1;
                    }
                    row_doc += self.arena.spaces(padding);
                }
                // Append trailing backslashes and linebreaks when multiple rows
                if num_rows > 1 {
                    row_doc += if add_trailing_linebreak || i + 1 != num_rows {
                        self.arena.text(" \\")
                    } else {
                        self.arena.text(" ")
                    };
                }
                if i + 1 != num_rows {
                    row_doc += self.arena.hardline();
                }
                row_doc
            }
        }))
    }
}

// Data structures for the alignment grid

/// A fully measured grid of rows and column widths.
struct Aligned<'a> {
    rows: Vec<Row<'a>>,
    col_widths: Vec<usize>,
}

/// A single row, either a list of cells or a standalone comment.
enum Row<'a> {
    Cells(Vec<Cell>),
    Comment(&'a str),
}

/// A formatted cell, storing its content and computed width.
#[derive(Debug)]
enum Cell {
    Empty,
    SingleLine(String, usize),       // text and its width
    MultiLine(Vec<(String, usize)>), // lines with their widths
}

impl Cell {
    /// Return the maximum display width of this cell.
    pub fn width(&self) -> usize {
        match self {
            Cell::Empty => 0,
            Cell::SingleLine(_, width) => *width,
            Cell::MultiLine(lines) => lines.iter().map(|(_, width)| *width).max().unwrap_or(0),
        }
    }
}

/// A raw row before rendering, coming from syntax nodes.
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

/// Collect math syntax nodes, split into lines/cells by align points and linebreaks.
fn collect_aligned<'a>(math: Math<'a>, attrs: &AttrStore) -> Vec<RawRow<'a>> {
    // Helper to trim trailing space nodes from a cell
    fn trim_trailing_spaces(cell: &mut Vec<&SyntaxNode>) {
        while cell
            .last()
            .is_some_and(|last| last.kind() == SyntaxKind::Space)
        {
            cell.pop();
        }
    }

    // Gather all relevant children, then split on linebreaks
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

    let flat = {
        let mut flat = Vec::with_capacity(math.to_untyped().children().len());
        collect_children(math.to_untyped(), attrs, &mut flat);
        flat
    };

    // First pass: split all children into lines (split at Linebreak)
    let lines = {
        let mut lines = flat
            .split(|n| n.kind() == SyntaxKind::Linebreak)
            .collect_vec();
        if lines.last().is_some_and(|last| last.is_empty()) {
            lines.pop();
        }
        lines
    };

    // Second pass: create rows; if a line starts with a line comment, create a Comment row.
    let mut rows = Vec::with_capacity(lines.len());
    for line in lines {
        let mut cells = Vec::new();
        let mut current_cell = Vec::new();
        for node in line {
            match node.kind() {
                SyntaxKind::MathAlignPoint => {
                    trim_trailing_spaces(&mut current_cell);
                    cells.push(std::mem::take(&mut current_cell));
                }
                SyntaxKind::Space if current_cell.is_empty() => {}
                SyntaxKind::LineComment if cells.is_empty() && current_cell.is_empty() => {
                    rows.push(RawRow::Comment(node));
                }
                _ => {
                    current_cell.push(node);
                }
            }
        }
        trim_trailing_spaces(&mut current_cell);
        cells.push(current_cell);
        rows.push(RawRow::Cells(cells));
    }
    rows
}
