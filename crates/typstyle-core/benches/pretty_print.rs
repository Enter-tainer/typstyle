use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Duration;
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

const TEST_ASSETS: [&str; 8] = [
    "tablex",
    "cetz-manual",
    "undergraduate-math",
    "packages/codly",
    "packages/fletcher-diagram",
    "packages/fletcher-draw",
    "packages/touying/core",
    "packages/touying/utils",
];

fn benchmark_attrs(c: &mut Criterion) {
    for name in TEST_ASSETS {
        bench_attrs(
            c,
            &format!("attrs-{name}"),
            &format!("../../tests/assets/{name}.typ"),
        );
    }
}

fn benchmark_pretty(c: &mut Criterion) {
    for name in TEST_ASSETS {
        bench_pretty(
            c,
            &format!("pretty-{name}"),
            &format!("../../tests/assets/{name}.typ"),
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(8))  // 只增加测量时间
        .sample_size(50);                          // 减少采样数量
    targets = benchmark_attrs, benchmark_pretty
}
criterion_main!(benches);
