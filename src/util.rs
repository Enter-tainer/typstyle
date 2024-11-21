use pretty::BoxDoc;

use crate::attr::Attributes;

/// A style for formatting items
pub enum FoldStyle {
    /// Fold items if them can fit in a single line
    Fit,
    /// Never fold items
    Never,
}

impl FoldStyle {
    pub fn from_attr(attr: Option<&Attributes>) -> FoldStyle {
        match attr {
            Some(attr) => {
                if attr.is_multiline_flavor() {
                    FoldStyle::Never
                } else {
                    FoldStyle::Fit
                }
            }
            None => FoldStyle::Fit,
        }
    }
}

pub fn comma_seprated_items<'a, I>(
    items: I,
    fold_style: FoldStyle,
    left: Option<&'static str>,
    right: Option<&'static str>,
) -> BoxDoc<'a, ()>
where
    I: IntoIterator<Item = BoxDoc<'a, ()>> + ExactSizeIterator,
{
    let left = left.unwrap_or("(");
    let right = right.unwrap_or(")");
    if items.len() == 0 {
        return BoxDoc::text(left).append(BoxDoc::text(right));
    }
    let comma_ = BoxDoc::text(",").flat_alt(BoxDoc::nil());
    match fold_style {
        FoldStyle::Fit => {
            let sep = BoxDoc::text(",").append(BoxDoc::line());
            let inner = BoxDoc::intersperse(items, sep).append(comma_);
            BoxDoc::text(left)
                .append(
                    BoxDoc::line_()
                        .append(inner)
                        .nest(2)
                        .append(BoxDoc::line_())
                        .group(),
                )
                .append(BoxDoc::text(right))
        }
        FoldStyle::Never => {
            let sep = BoxDoc::text(",").append(BoxDoc::hardline());
            let inner = BoxDoc::intersperse(items, sep).append(BoxDoc::text(","));
            BoxDoc::text(left)
                .append(BoxDoc::hardline().append(inner).nest(2))
                .append(BoxDoc::hardline())
                .append(BoxDoc::text(right))
        }
    }
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
    let multi = {
        let mut inner = BoxDoc::nil();
        for item in items {
            inner = inner
                .append(item.clone())
                .append(multi_line_separator.clone().append(BoxDoc::hardline()));
        }
        let doc = BoxDoc::hardline().append(inner).nest(2);
        left.clone().append(doc).append(right.clone())
    };
    match fold_style {
        FoldStyle::Fit => {
            let flat = {
                let inner = BoxDoc::intersperse(items.iter().cloned(), single_line_separator);
                let (left, right) = if bracket_space {
                    (
                        left.clone().append(BoxDoc::space()),
                        BoxDoc::space().append(right.clone()),
                    )
                } else {
                    (left, right)
                };
                left.append(inner).append(right)
            };
            multi.clone().flat_alt(flat).group()
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
}
