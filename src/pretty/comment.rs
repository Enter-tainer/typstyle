use pretty::{Arena, DocAllocator};
use typst_syntax::{SyntaxKind, SyntaxNode};

use super::{MyDoc, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_comment(&'a self, node: &'a SyntaxNode) -> MyDoc<'a> {
        comment(&self.arena, node)
    }

    pub(super) fn convert_line_comment(&'a self, node: &'a SyntaxNode) -> MyDoc<'a> {
        line_comment(&self.arena, node)
    }

    pub(super) fn convert_block_comment(&'a self, node: &'a SyntaxNode) -> MyDoc<'a> {
        block_comment(&self.arena, node)
    }
}

enum CommentStyle {
    Plain,
    Bullet,
}

/// Convert either line comment or block comment.
pub fn comment<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> MyDoc<'a> {
    if node.kind() == SyntaxKind::LineComment {
        line_comment(arena, node)
    } else if node.kind() == SyntaxKind::BlockComment {
        block_comment(arena, node)
    } else {
        unreachable!("the node should not be a comment node!")
    }
}

pub fn line_comment<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> MyDoc<'a> {
    arena.text(node.text().as_str())
}

pub fn block_comment<'a>(arena: &'a Arena<'a>, node: &'a SyntaxNode) -> MyDoc<'a> {
    // Calculate the number of leading spaces except the first line.
    let line_num = node.text().lines().count();
    if line_num == 0 {
        return arena.text(node.text().as_str());
    }
    // Then the comment is multiline.
    let text = node.text().clone();
    let style = get_comment_style(&text);
    match style {
        CommentStyle::Plain => {
            let leading = get_follow_leading(&text).unwrap();
            arena.column(move |col| {
                arena
                    .text(align_multiline(&text, leading, line_num, col))
                    .into_doc()
            })
        }
        CommentStyle::Bullet => {
            arena.column(move |col| arena.text(align_multiline_simple(&text, col)).into_doc())
        }
    }
}

fn get_comment_style(text: &str) -> CommentStyle {
    if text
        .lines()
        .skip(1)
        .all(|line| line.trim_start().starts_with('*'))
    {
        CommentStyle::Bullet // /*
    } else {
        CommentStyle::Plain // otherwise
    }
}

/// Get the minimum number of leading spaces in all lines except the first.
/// Returns None only when the text is a single line.
fn get_follow_leading(text: &str) -> Option<usize> {
    text.lines()
        .skip(1)
        .map(|line| line.chars().position(|c| c != ' ').unwrap_or(usize::MAX))
        .min()
}

/// For general cases. All lines need to be indented simultaneously.
fn align_multiline(text: &str, leading: usize, line_num: usize, col: usize) -> String {
    if col == leading {
        return text.to_string();
    }
    let offset = col as isize - leading as isize;
    let mut result =
        String::with_capacity((text.len() as isize + (line_num as isize - 1) * offset) as usize);
    if col < leading {
        // need to remove line prefix
        let offset = leading - col;
        for (i, line) in text.lines().enumerate() {
            if i == 0 {
                result.push_str(line);
                continue;
            }
            result.push('\n');
            if line.len() > offset {
                result.push_str(&line[offset..]);
            } // otherwise this line is blank
        }
    } else {
        // need to add spaces
        let offset = col - leading;
        for (i, line) in text.lines().enumerate() {
            if i == 0 {
                result.push_str(line);
                continue;
            }
            result.push('\n');
            result.extend(std::iter::repeat_n(' ', offset));
            result.push_str(line);
        }
    }
    result
}

/// For special cases. All lines can be indented independently.
fn align_multiline_simple(text: &str, col: usize) -> String {
    let mut result = String::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            result.push_str(line);
            continue;
        }
        result.push('\n');
        result.extend(std::iter::repeat_n(' ', col + 1));
        result.push_str(line.trim_start());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::pretty::comment::{align_multiline, align_multiline_simple, get_follow_leading};

    #[test]
    fn test_align() {
        let text = "/*
      ---
        --
    ---
      */";
        let leading = get_follow_leading(text).unwrap();
        assert_eq!(leading, 4);
        assert_eq!(
            align_multiline(text, leading, 5, 0),
            "/*
  ---
    --
---
  */"
        );
        assert_eq!(
            align_multiline(text, leading, 5, 8),
            "/*
          ---
            --
        ---
          */"
        );
    }

    #[test]
    fn test_align2() {
        let text = "/*
      *
        *
    *
      */";
        let leading = get_follow_leading(text).unwrap();
        assert_eq!(leading, 4);
        assert_eq!(
            align_multiline_simple(text, 0),
            "/*
 *
 *
 *
 */"
        );
        assert_eq!(
            align_multiline_simple(text, 8),
            "/*
         *
         *
         *
         */"
        );
    }
}
