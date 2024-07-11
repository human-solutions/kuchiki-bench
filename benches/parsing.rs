use criterion::{criterion_group, criterion_main, Criterion};
use kuchiki_bench::HtmlDoc;

pub fn parsing(c: &mut Criterion) {
    let path = "src/fr.serrer.html";

    c.bench_function("parse fr.serrer.html", |b| b.iter(|| HtmlDoc::parse(path)));
}

criterion_group!(benches, parsing,);
criterion_main!(benches);
