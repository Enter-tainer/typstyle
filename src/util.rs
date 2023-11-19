use pretty::BoxDoc;

pub fn pretty_items<'a>(
    items: &[BoxDoc<'a, ()>],
    single_line_separator: BoxDoc<'a, ()>,
    multi_line_bracket: BoxDoc<'a, ()>,
    bracket: (BoxDoc<'a, ()>, BoxDoc<'a, ()>),
) -> BoxDoc<'a, ()> {
    let (left, right) = bracket;
    let inner_flat: BoxDoc<'a, ()> = {
        BoxDoc::intersperse(
            items.iter().cloned(),
            single_line_separator.append(BoxDoc::space()),
        )
    };
    let inner_multi = {
        let mut inner = BoxDoc::nil();
        for item in items {
            inner = inner
                .append(item.clone())
                .append(multi_line_bracket.clone().append(BoxDoc::hardline()));
        }
        BoxDoc::line().append(inner)
    }
    .nest(2);
    let inner = inner_multi.flat_alt(inner_flat).group();
    left.append(inner).append(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_items() {
        let strs = ["123", "12345", "1234", "1234567"];
        let docs: Vec<_> = strs.iter().map(|s| BoxDoc::text(s.to_string())).collect();
        let outer = pretty_items(
            &docs,
            BoxDoc::text(","),
            BoxDoc::text(","),
            (BoxDoc::text("["), BoxDoc::text("]")),
        );
        insta::assert_debug_snapshot!(outer);
        insta::assert_snapshot!(outer.pretty(10).to_string());
        insta::assert_snapshot!(outer.pretty(80).to_string());
    }
}
