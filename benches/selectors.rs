use criterion::{criterion_group, criterion_main, Criterion};
use kuchiki_bench::HtmlDoc;

pub fn selectors(c: &mut Criterion) {
    let html = HtmlDoc::parse("src/fr.serrer.html");

    // html.root_element().select(selector);
    c.bench_function("select divs in fr.serrer.html", |b| {
        b.iter(|| html.doc.select("div").unwrap().count())
    });

    c.bench_function("select class in fr.serrer.html", |b| {
        b.iter(|| html.doc.select(".vector-menu-content").unwrap().count())
    });
}

criterion_group!(benches, selectors,);
criterion_main!(benches);
