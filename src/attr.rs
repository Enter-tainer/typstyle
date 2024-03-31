use std::collections::{hash_map::Entry, HashMap};

use typst_syntax::{
    ast::{Args, AstNode, Math, Raw},
    SyntaxKind, SyntaxNode,
};

struct State {
    is_math: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes {
    no_format: bool,
}

impl Attributes {
    pub fn no_format(&self) -> bool {
        self.no_format
    }
}

pub fn calculate_attributes(node: SyntaxNode) -> HashMap<SyntaxNode, Attributes> {
    let mut attr_map = HashMap::new();
    get_no_format_nodes(node, &mut attr_map);
    attr_map
}

fn get_no_format_nodes(root: SyntaxNode, attr_map: &mut HashMap<SyntaxNode, Attributes>) {
    let mut state = State { is_math: false };
    get_no_format_nodes_impl(root, attr_map, &mut state);
}

fn get_no_format_nodes_impl(
    node: SyntaxNode,
    attr_map: &mut HashMap<SyntaxNode, Attributes>,
    state: &mut State,
) {
    if attr_map.get(&node).map_or(false, |attr| attr.no_format) {
        return;
    }
    let mut no_format = false;
    let original_is_math = state.is_math;
    if node.cast::<Math>().is_some() {
        state.is_math = true;
    }
    let set_no_format =
        |node: SyntaxNode, attr_map: &mut HashMap<SyntaxNode, Attributes>| match attr_map
            .entry(node)
        {
            Entry::Occupied(mut entry) => {
                entry.get_mut().no_format = true;
            }
            Entry::Vacant(entry) => {
                entry.insert(Attributes { no_format: true });
            }
        };
    for child in node.children() {
        if child.kind() == SyntaxKind::LineComment || child.kind() == SyntaxKind::BlockComment {
            // @typstyle off affects the whole next block
            if child.text().contains("@typstyle off") {
                no_format = true;
                set_no_format(child.clone(), attr_map);
            }
            // child contains block comment and is not a code block or content block
            // no format current node
            if child.kind() == SyntaxKind::BlockComment
                || (node.kind() != SyntaxKind::ContentBlock
                    && node.kind() != SyntaxKind::CodeBlock
                    && node.kind() != SyntaxKind::Code)
            {
                set_no_format(node.clone(), attr_map);
            }
            continue;
        }
        // no format multiline single backtick raw block
        if let Some(raw) = child.cast::<Raw>() {
            if !raw.block() {
                if let Some(line) = raw.lines().next() {
                    if line.get().contains('\n') {
                        set_no_format(child.clone(), attr_map);
                    }
                }
            }
        }
        // no format hash related nodes in math blocks
        if child.kind() == SyntaxKind::Hash && state.is_math {
            set_no_format(node.clone(), attr_map);
            break;
        }
        // no format args in math blocks
        if child.cast::<Args>().is_some() && state.is_math {
            set_no_format(child.clone(), attr_map);
            continue;
        }
        if child.children().count() == 0 {
            continue;
        }
        // no format nodes with @typstyle off
        if no_format {
            set_no_format(child.clone(), attr_map);
            no_format = false;
            continue;
        }
        get_no_format_nodes_impl(child.clone(), attr_map, state);
    }
    state.is_math = original_is_math;
}

#[allow(unused)]
fn is_2d_arg(arg: Args) -> bool {
    for child in arg.to_untyped().children() {
        if child.kind() == SyntaxKind::Semicolon {
            return true;
        }
    }
    false
}
