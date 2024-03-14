use std::collections::HashSet;

use typst_syntax::{SyntaxKind, SyntaxNode};

pub fn get_no_format_nodes(root: SyntaxNode) -> HashSet<SyntaxNode> {
    let mut no_format_nodes = HashSet::new();
    get_no_format_nodes_impl(root, &mut no_format_nodes);
    no_format_nodes
}

fn get_no_format_nodes_impl(node: SyntaxNode, map: &mut HashSet<SyntaxNode>) {
    if map.get(&node).is_some() {
        return;
    }
    let mut no_format = false;
    for child in node.children() {
        if child.kind() == SyntaxKind::LineComment || child.kind() == SyntaxKind::BlockComment {
            if child.text().contains("@geshihua off") {
                no_format = true;
                map.insert(child.clone());
            }
            if node.kind() != SyntaxKind::ContentBlock
                || node.kind() != SyntaxKind::CodeBlock
                || node.kind() != SyntaxKind::Code
            {
                map.insert(node.clone());
            }
        }
        if child.children().count() == 0 {
            continue;
        }
        if no_format {
            map.insert(child.clone());
            no_format = false;
        } else {
            get_no_format_nodes_impl(child.clone(), map);
        }
    }
}
