use typst_syntax::{ast::*, SyntaxKind};

use super::{
    list::{ListStyle, ListStylist},
    mode::Mode,
    style::FoldStyle,
    util::{has_comment_children, is_only_one_and},
    ArenaDoc, PrettyPrinter,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_code_block(&'a self, code_block: CodeBlock<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Code);

        let mut nodes = vec![];
        for child in code_block.to_untyped().children() {
            if let Some(code) = child.cast::<Code>() {
                nodes.extend(code.to_untyped().children());
            } else {
                nodes.push(child);
            }
        }

        let can_fold = code_block.body().exprs().count() <= 1
            && !has_comment_children(code_block.to_untyped());
        ListStylist::new(self)
            .disallow_front_comment()
            .with_fold_style(if can_fold {
                self.get_fold_style(code_block)
            } else {
                FoldStyle::Never
            })
            .keep_linebreak(self.config.blank_lines_upper_bound)
            .process_iterable(nodes.into_iter(), |expr| self.convert_expr(expr))
            .print_doc(ListStyle {
                separator: "",
                delim: ("{", "}"),
                add_delim_space: true,
                ..Default::default()
            })
    }

    pub(super) fn convert_parenthesized_impl(
        &'a self,
        parenthesized: Parenthesized<'a>,
    ) -> ArenaDoc<'a> {
        // NOTE: This is a safe cast. The parentheses for patterns are all optional.
        // For safety, we don't remove parentheses around idents. See `paren-in-key.typ`.
        let expr = parenthesized.expr();
        let can_omit = (expr.is_literal()
            || matches!(
                expr.to_untyped().kind(),
                SyntaxKind::Array
                    | SyntaxKind::Dict
                    | SyntaxKind::Destructuring
                    | SyntaxKind::CodeBlock
                    | SyntaxKind::ContentBlock
            ))
            && !has_comment_children(parenthesized.to_untyped());

        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(parenthesized))
            .process_list(parenthesized.to_untyped(), |node| {
                self.convert_pattern(node)
            })
            .print_doc(ListStyle {
                separator: "",
                omit_delim_flat: can_omit,
                ..Default::default()
            })
    }

    pub(super) fn convert_array(&'a self, array: Array<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(array))
            .process_list(array.to_untyped(), |node| self.convert_array_item(node))
            .print_doc(ListStyle {
                add_trailing_sep_single: true,
                ..Default::default()
            })
    }

    pub(super) fn convert_dict(&'a self, dict: Dict<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        let all_spread = dict.items().all(|item| matches!(item, DictItem::Spread(_)));

        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(dict))
            .process_list(dict.to_untyped(), |node| self.convert_dict_item(node))
            .print_doc(ListStyle {
                delim: (if all_spread { "(:" } else { "(" }, ")"),
                ..Default::default()
            })
    }

    pub(super) fn convert_destructuring(
        &'a self,
        destructuring: Destructuring<'a>,
    ) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

        let only_one_pattern = is_only_one_and(destructuring.items(), |it| {
            matches!(*it, DestructuringItem::Pattern(_))
        });

        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(destructuring))
            .process_list(destructuring.to_untyped(), |node| {
                self.convert_destructuring_item(node)
            })
            .always_fold_if(|| only_one_pattern)
            .print_doc(ListStyle {
                add_trailing_sep_single: only_one_pattern,
                ..Default::default()
            })
    }

    pub(super) fn convert_params(&'a self, params: Params<'a>, is_unnamed: bool) -> ArenaDoc<'a> {
        // SAFETY: The param must be simple if the parens is optional.
        let _g = self.with_mode(Mode::CodeCont);

        let is_single_simple = is_unnamed
            && is_only_one_and(params.children(), |it| {
                matches!(
                    *it,
                    Param::Pos(Pattern::Normal(_)) | Param::Pos(Pattern::Placeholder(_))
                )
            });

        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(params))
            .process_list(params.to_untyped(), |node| self.convert_param(node))
            .always_fold_if(|| is_single_simple)
            .print_doc(ListStyle {
                omit_delim_single: is_single_simple,
                ..Default::default()
            })
    }

    pub(super) fn convert_import_items(&'a self, import_items: ImportItems<'a>) -> ArenaDoc<'a> {
        // Note that `ImportItem` does not implement `AstNode`.
        ListStylist::new(self)
            .with_fold_style(self.get_fold_style(import_items))
            .process_list_impl(import_items.to_untyped(), |child| match child.kind() {
                SyntaxKind::RenamedImportItem => child
                    .cast()
                    .map(|item| self.convert_import_item_renamed(item)),
                SyntaxKind::ImportItemPath => {
                    child.cast().map(|item| self.convert_import_item_path(item))
                }
                _ => Option::None,
            })
            .print_doc(ListStyle {
                omit_delim_flat: true,
                omit_delim_empty: true,
                ..Default::default()
            })
    }
}
