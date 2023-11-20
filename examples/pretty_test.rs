use pretty::BoxDoc;
use typst_geshihua::util::pretty_items;

fn main() {
    let strs = ["123", "12345", "1234", "1234567"];

    let docs: Vec<_> = strs.iter().map(|s| BoxDoc::text(s.to_string())).collect();
    let outer = pretty_items(
        &docs,
        BoxDoc::text(",").append(BoxDoc::space()),
        BoxDoc::text(","),
        (BoxDoc::text("["), BoxDoc::text("]")),
    );

    let res_10 = outer.pretty(10).to_string();
    let res_80 = outer.pretty(80).to_string();

    println!("10:\n{}", res_10);
    println!("80:\n{}", res_80);
}
