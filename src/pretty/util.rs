use typst_syntax::{ast::*, SyntaxKind};

pub(super) fn func_name(node: FuncCall<'_>) -> Option<&str> {
    node.callee()
        .to_untyped()
        .cast::<Ident>()
        .map(|ident| ident.as_str())
}

pub(super) fn has_parenthesized_args(node: FuncCall<'_>) -> bool {
    node.args()
        .to_untyped()
        .children()
        .any(|node| matches!(node.kind(), SyntaxKind::LeftParen | SyntaxKind::RightParen))
}

#[allow(unused)]
pub(super) fn has_additional_args(node: FuncCall<'_>) -> bool {
    let has_paren_args = has_parenthesized_args(node);
    let args = node
        .args()
        .to_untyped()
        .children()
        .skip_while(|node| {
            if has_paren_args {
                node.kind() != SyntaxKind::RightParen
            } else {
                node.kind() != SyntaxKind::ContentBlock
            }
        })
        .filter_map(|node| node.cast::<'_, Arg>());
    args.count() > 1
}
