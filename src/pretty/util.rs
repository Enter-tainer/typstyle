use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

pub(super) fn indent_func_name(node: FuncCall<'_>) -> Option<&str> {
    node.callee()
        .to_untyped()
        .cast::<Ident>()
        .map(|ident| ident.as_str())
}

pub(super) fn func_name(node: FuncCall<'_>) -> String {
    node.callee().to_untyped().clone().into_text().to_string()
}

pub(super) fn has_parenthesized_args(node: Args<'_>) -> bool {
    node.to_untyped()
        .children()
        .any(|node| matches!(node.kind(), SyntaxKind::LeftParen | SyntaxKind::RightParen))
}

pub(super) fn get_parenthesized_args_untyped(node: Args<'_>) -> impl Iterator<Item = &SyntaxNode> {
    node.to_untyped()
        .children()
        .skip_while(|node| node.kind() != SyntaxKind::LeftParen)
        .skip(1)
        .take_while(|node| node.kind() != SyntaxKind::RightParen)
}

#[allow(unused)]
pub(super) fn get_parenthesized_args(node: Args<'_>) -> impl Iterator<Item = Arg<'_>> {
    get_parenthesized_args_untyped(node).filter_map(|node| node.cast::<Arg>())
}

#[allow(unused)]
pub(super) fn has_additional_args(node: Args<'_>) -> bool {
    let has_paren_args = has_parenthesized_args(node);
    let args = node
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
