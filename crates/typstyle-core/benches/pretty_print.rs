use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use typst_syntax::Source;
use typstyle_core::{attr::AttrStore, Typstyle};

fn bench_attrs(c: &mut Criterion, id: &str, path: &str) {
    c.bench_function(id, |b| {
        let content = fs::read_to_string(path).unwrap();
        let source = Source::detached(content);
        b.iter(|| AttrStore::new(source.root()))
    });
}

fn bench_pretty(c: &mut Criterion, id: &str, path: &str) {
    fn pretty_print_source(source: Source) -> String {
        let t = Typstyle::new_with_src(source, 80);
        t.pretty_print()
    }

    c.bench_function(id, |b| {
        let content = fs::read_to_string(path).unwrap();
        let source = Source::detached(content);
        b.iter(|| pretty_print_source(source.clone()))
    });
}

/// (path, name)
const TEST_ASSETS: [(&str, &str); 8] = [
    ("articles/undergraduate-math", "undergraduate-math"),
    ("packages/cetz-manual", "cetz-manual"),
    ("packages/codly", "codly"),
    ("packages/fletcher-diagram", "fletcher-diagram"),
    ("packages/fletcher-draw", "fletcher-draw"),
    ("packages/tablex", "tablex"),
    ("packages/touying/core", "touying-core"),
    ("packages/touying/utils", "touying-utils"),
];

fn benchmark_attrs(c: &mut Criterion) {
    for (path, name) in TEST_ASSETS {
        bench_attrs(
            c,
            &format!("attrs-{name}"),
            &format!("../../tests/fixtures/{path}.typ"),
        );
    }
}

fn benchmark_pretty(c: &mut Criterion) {
    for (path, name) in TEST_ASSETS {
        bench_pretty(
            c,
            &format!("pretty-{name}"),
            &format!("../../tests/fixtures/{path}.typ"),
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmark_attrs, benchmark_pretty
}
criterion_main!(benches);
