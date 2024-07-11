use criterion::{criterion_group, criterion_main, Criterion};
use kuchiki_bench::HtmlDoc;

pub fn iteration(c: &mut Criterion) {
    let path = "src/fr.serrer.html";

    let html = HtmlDoc::parse(path);

    c.bench_function(
        "Iteration: forwards through all 5589 elements in fr.serrer.html",
        |b| b.iter(|| html.doc.descendants().count()),
    );
}

criterion_group!(benches, iteration,);
criterion_main!(benches);
