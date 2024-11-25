use criterion::{criterion_group, criterion_main, Criterion};
use kuchiki_bench::HtmlDoc;

pub fn writing(c: &mut Criterion) {
    let path = "src/fr.serrer.html";
    let doc = HtmlDoc::parse(path);

    c.bench_function("write fr.serrer.html to string", |b| b.iter(|| doc.write()));
}

criterion_group!(benches, writing,);
criterion_main!(benches);
