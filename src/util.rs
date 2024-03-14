use pretty::BoxDoc;

/// A style for formatting items
pub enum FoldStyle {
    /// Fold items if them can fit in a single line
    Fit,
    /// Fold items if there is only one item and it fit in a single line, other wise put each item in a line
    Single,
    /// Never fold items
    Never,
}

pub fn pretty_items<'a>(
    items: &[BoxDoc<'a, ()>],
    single_line_separator: BoxDoc<'a, ()>,
    multi_line_separator: BoxDoc<'a, ()>,
    bracket: (BoxDoc<'a, ()>, BoxDoc<'a, ()>),
    bracket_space: bool,
    fold_style: FoldStyle,
) -> BoxDoc<'a, ()> {
    if items.is_empty() {
        return bracket.0.append(if bracket_space {
            BoxDoc::space().append(bracket.1)
        } else {
            bracket.1
        });
    }
    pretty_items_impl(
        items,
        single_line_separator,
        multi_line_separator,
        bracket,
        bracket_space,
        fold_style,
    )
}

fn pretty_items_impl<'a>(
    items: &[BoxDoc<'a, ()>],
    single_line_separator: BoxDoc<'a, ()>,
    multi_line_separator: BoxDoc<'a, ()>,
    bracket: (BoxDoc<'a, ()>, BoxDoc<'a, ()>),
    bracket_space: bool,
    fold_style: FoldStyle,
) -> BoxDoc<'a, ()> {
    let (left, right) = bracket;
    let flat: BoxDoc<'a, ()> = {
        let inner = BoxDoc::intersperse(items.iter().cloned(), single_line_separator);
        let (left, right) = if bracket_space {
            (
                left.clone().append(BoxDoc::space()),
                BoxDoc::space().append(right.clone()),
            )
        } else {
            (left.clone(), right.clone())
        };
        left.append(inner).append(right)
    };
    let multi = {
        let mut inner = BoxDoc::nil();
        for item in items {
            inner = inner
                .append(item.clone())
                .append(multi_line_separator.clone().append(BoxDoc::hardline()));
        }
        let doc = BoxDoc::hardline().append(inner).nest(2);
        left.append(doc).append(right)
    };
    let auto_items = multi.clone().flat_alt(flat).group();
    match fold_style {
        FoldStyle::Fit => auto_items,
        FoldStyle::Single => {
            if items.len() == 1 {
                auto_items
            } else {
                multi
            }
        }
        FoldStyle::Never => multi,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_items_fit() {
        let strs = ["123", "12345", "1234", "1234567"];
        let docs: Vec<_> = strs.iter().map(|s| BoxDoc::text(s.to_string())).collect();
        let outer = pretty_items(
            &docs,
            BoxDoc::text(",").append(BoxDoc::space()),
            BoxDoc::text(","),
            (BoxDoc::text("["), BoxDoc::text("]")),
            false,
            FoldStyle::Fit,
        );
        insta::assert_debug_snapshot!(outer);
        insta::assert_snapshot!(outer.pretty(10).to_string());
        insta::assert_snapshot!(outer.pretty(80).to_string());
    }

    #[test]
    fn test_pretty_items_single() {
        let strs = ["let a = 1"];
        let docs: Vec<_> = strs.iter().map(|s| BoxDoc::text(s.to_string())).collect();
        let outer = pretty_items(
            &docs,
            BoxDoc::text(";").append(BoxDoc::space()),
            BoxDoc::text(";"),
            (BoxDoc::text("{"), BoxDoc::text("}")),
            true,
            FoldStyle::Single,
        );
        insta::assert_debug_snapshot!(outer);
        insta::assert_snapshot!(outer.pretty(10).to_string());
        insta::assert_snapshot!(outer.pretty(80).to_string());
    }
}
