use std::collections::VecDeque;

use pretty::BoxDoc;
use typst_syntax::ast::*;
use typst_syntax::{ast, SyntaxNode};

use crate::pretty::trivia;
use crate::PrettyPrinter;

impl PrettyPrinter {
    pub fn convert_markup<'a>(&'a self, root: Markup<'a>) -> BoxDoc<'a, ()> {
        let mut doc: BoxDoc<()> = BoxDoc::nil();
        #[derive(Debug, Default)]
        struct Line<'a> {
            has_text: bool,
            nodes: VecDeque<&'a SyntaxNode>,
        }
        // break markup into lines, split by stmt, parbreak, newline, multiline raw, equation
        // if a line contains text, it will be skipped by the formatter to keep the original format
        let mut lines = {
            let mut lines: VecDeque<Line> = VecDeque::new();
            let mut current_line = Line {
                has_text: false,
                nodes: VecDeque::new(),
            };
            for node in root.to_untyped().children() {
                let mut break_line = false;
                if let Some(space) = node.cast::<Space>() {
                    if space.to_untyped().text().contains('\n') {
                        break_line = true;
                    }
                } else if let Some(pb) = node.cast::<Parbreak>() {
                    if pb.to_untyped().text().contains('\n') {
                        break_line = true;
                    }
                } else if node.kind().is_stmt() {
                    break_line = true;
                } else if let Some(expr) = node.cast::<Expr>() {
                    match expr {
                        ast::Expr::Text(_) => current_line.has_text = true,
                        ast::Expr::Raw(r) => {
                            if r.block() {
                                break_line = true;
                            } else {
                                current_line.has_text = true;
                            }
                        }
                        ast::Expr::Strong(_) | ast::Expr::Emph(_) => current_line.has_text = true,
                        ast::Expr::Code(_) => break_line = true,
                        ast::Expr::Equation(e) if e.block() => break_line = true,
                        _ => (),
                    }
                }
                current_line.nodes.push_back(node);
                if break_line {
                    lines.push_back(current_line);
                    current_line = Line::default();
                }
            }
            if !current_line.nodes.is_empty() {
                lines.push_back(current_line);
            }
            lines
        };

        // remove prefix and postfix spaces
        while let Some(Line { has_text: _, nodes }) = lines.front() {
            if nodes.len() == 1 {
                if let Some(_space) = nodes[0].cast::<Space>() {
                    lines.pop_front();
                    continue;
                }
            }
            break;
        }
        while let Some(Line { has_text: _, nodes }) = lines.back() {
            if nodes.len() == 1 {
                if let Some(_space) = nodes[0].cast::<Space>() {
                    lines.pop_back();
                    continue;
                }
            }
            break;
        }
        if let Some(Line { has_text: _, nodes }) = lines.front_mut() {
            if let Some(_space) = nodes.front().and_then(|node| node.cast::<Space>()) {
                nodes.pop_front();
            }
        }
        if let Some(Line { has_text: _, nodes }) = lines.back_mut() {
            if let Some(_space) = nodes.back().and_then(|node| node.cast::<Space>()) {
                nodes.pop_back();
            }
        }

        for Line { has_text, nodes } in lines.into_iter() {
            for node in nodes {
                if let Some(space) = node.cast::<Space>() {
                    doc = doc.append(self.convert_space(space));
                    continue;
                }
                if let Some(pb) = node.cast::<Parbreak>() {
                    doc = doc.append(self.convert_parbreak(pb));
                    continue;
                }
                if has_text {
                    doc = doc.append(self.format_disabled(node));
                } else if let Some(expr) = node.cast::<Expr>() {
                    let expr_doc = self.convert_expr(expr);
                    doc = doc.append(expr_doc);
                } else {
                    doc = doc.append(trivia(node));
                }
            }
        }
        doc
    }
}
