use pretty::BoxDoc;

fn main() {
    let strs = ["123", "12345", "1234", "1234567"];
    
    let inner_flat: BoxDoc<'_, ()> = BoxDoc::intersperse(
        strs.into_iter().map(BoxDoc::text),
        BoxDoc::text(",").append(BoxDoc::space()),
    );
    
    let inner_multi = {
        let mut inner = BoxDoc::nil();
        for s in strs {
            inner = inner
                .append(BoxDoc::text(s))
                .append(BoxDoc::text(",").append(BoxDoc::hardline()));
        }
        BoxDoc::line().append(inner)
    }
    .nest(2);
  
    let outer = BoxDoc::text("[")
        .append(inner_multi.flat_alt(inner_flat).group())
        .append(BoxDoc::text("]"));

    let res_10 = outer.pretty(10).to_string();
    let res_80 = outer.pretty(80).to_string();
    
    println!("10:\n{}", res_10);
    println!("80:\n{}", res_80);
}
