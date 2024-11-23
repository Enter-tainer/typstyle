use pretty::{Arena, DocAllocator};

use super::style::FoldStyle;
use super::ArenaDoc;

pub fn pretty_items<'a>(
    arena: &'a Arena<'a>,
    items: &[ArenaDoc<'a>],
    single_line_separator: ArenaDoc<'a>,
    multi_line_separator: ArenaDoc<'a>,
    bracket: (ArenaDoc<'a>, ArenaDoc<'a>),
    bracket_space: bool,
    fold_style: FoldStyle,
) -> ArenaDoc<'a> {
    if items.is_empty() {
        return bracket.0.append(if bracket_space {
            arena.space().append(bracket.1)
        } else {
            bracket.1
        });
    }
    pretty_items_impl(
        arena,
        items,
        single_line_separator,
        multi_line_separator,
        bracket,
        bracket_space,
        fold_style,
    )
}

fn pretty_items_impl<'a>(
    arena: &'a Arena<'a>,
    items: &[ArenaDoc<'a>],
    single_line_separator: ArenaDoc<'a>,
    multi_line_separator: ArenaDoc<'a>,
    bracket: (ArenaDoc<'a>, ArenaDoc<'a>),
    bracket_space: bool,
    fold_style: FoldStyle,
) -> ArenaDoc<'a> {
    let (left, right) = bracket;
    let multi = {
        let mut inner = arena.nil();
        for item in items {
            inner = inner
                .append(item.clone())
                .append(multi_line_separator.clone().append(arena.hardline()));
        }
        let doc = arena.hardline().append(inner).nest(2);
        left.clone().append(doc).append(right.clone())
    };
    match fold_style {
        FoldStyle::Fit | FoldStyle::Always => {
            let flat = {
                let inner = arena.intersperse(items.iter().cloned(), single_line_separator);
                let (left, right) = if bracket_space {
                    (
                        left.clone().append(arena.space()),
                        arena.space().append(right.clone()),
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
        let arena = Arena::new();
        let docs: Vec<_> = strs.iter().map(|s| arena.text(s.to_string())).collect();
        let outer = pretty_items(
            &arena,
            &docs,
            arena.text(",").append(arena.space()),
            arena.text(","),
            (arena.text("["), arena.text("]")),
            false,
            FoldStyle::Fit,
        );
        insta::assert_debug_snapshot!(outer);
        insta::assert_snapshot!(outer.pretty(10).to_string());
        insta::assert_snapshot!(outer.pretty(80).to_string());
    }
}
