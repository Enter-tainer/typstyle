use pretty::BoxDoc;

/// A style for formatting items
pub enum FoldStyle {
    /// Fold items if them can fit in a single line
    Fit,
    /// Fold items if there is only one item, other wise put each item in a single line
    Single,
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
    let bracket = if bracket_space {
        (
            bracket.0.append(BoxDoc::space()),
            BoxDoc::space().append(bracket.1),
        )
    } else {
        bracket
    };
    match fold_style {
        FoldStyle::Fit => {
            pretty_items_fit(items, single_line_separator, multi_line_separator, bracket)
        }
        FoldStyle::Single => {
            pretty_items_single(items, single_line_separator, multi_line_separator, bracket)
        }
    }
}

fn pretty_items_fit<'a>(
    items: &[BoxDoc<'a, ()>],
    single_line_separator: BoxDoc<'a, ()>,
    multi_line_separator: BoxDoc<'a, ()>,
    bracket: (BoxDoc<'a, ()>, BoxDoc<'a, ()>),
) -> BoxDoc<'a, ()> {
    let (left, right) = bracket;
    let inner_flat: BoxDoc<'a, ()> =
        { BoxDoc::intersperse(items.iter().cloned(), single_line_separator) };
    let inner_multi = {
        let mut inner = BoxDoc::nil();
        for item in items {
            inner = inner
                .append(item.clone())
                .append(multi_line_separator.clone().append(BoxDoc::hardline()));
        }
        BoxDoc::hardline().append(inner)
    }
    .nest(2);
    let inner = inner_multi.flat_alt(inner_flat).group();
    left.append(inner).append(right)
}

fn pretty_items_single<'a>(
    items: &[BoxDoc<'a, ()>],
    _single_line_separator: BoxDoc<'a, ()>,
    multi_line_separator: BoxDoc<'a, ()>,
    bracket: (BoxDoc<'a, ()>, BoxDoc<'a, ()>),
) -> BoxDoc<'a, ()> {
    let (left, right) = bracket;
    if items.len() == 1 {
        left.append(items[0].clone()).append(right)
    } else {
        let multi = {
            let mut inner = BoxDoc::nil();
            for item in items {
                inner = inner
                    .append(item.clone())
                    .append(multi_line_separator.clone().append(BoxDoc::hardline()));
            }
            BoxDoc::hardline().append(inner).nest(2).group()
        };
        left.append(multi).append(right)
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
        let strs = ["let a = 1;"];
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
