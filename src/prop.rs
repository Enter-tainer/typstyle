use std::collections::HashSet;

use typst_syntax::{
    ast::{Args, AstNode, Math},
    SyntaxKind, SyntaxNode,
};

struct State {
    is_math: bool,
}

pub fn get_no_format_nodes(root: SyntaxNode) -> HashSet<SyntaxNode> {
    let mut no_format_nodes = HashSet::new();
    let mut state = State { is_math: false };
    get_no_format_nodes_impl(root, &mut no_format_nodes, &mut state);
    no_format_nodes
}

fn get_no_format_nodes_impl(node: SyntaxNode, map: &mut HashSet<SyntaxNode>, state: &mut State) {
    if map.get(&node).is_some() {
        return;
    }
    let mut no_format = false;
    let original_is_math = state.is_math;
    if node.cast::<Math>().is_some() {
        state.is_math = true;
    }
    for child in node.children() {
        if child.kind() == SyntaxKind::LineComment || child.kind() == SyntaxKind::BlockComment {
            if child.text().contains("@typstyle off") {
                no_format = true;
                map.insert(child.clone());
            }
            if node.kind() != SyntaxKind::ContentBlock
                || node.kind() != SyntaxKind::CodeBlock
                || node.kind() != SyntaxKind::Code
            {
                map.insert(node.clone());
            }
            continue;
        }
        if child.cast::<Args>().is_some() && state.is_math {
            map.insert(child.clone());
            continue;
        }
        if child.children().count() == 0 {
            continue;
        }
        if no_format {
            map.insert(child.clone());
            no_format = false;
            continue;
        }
        get_no_format_nodes_impl(child.clone(), map, state);
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
