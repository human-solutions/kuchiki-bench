use kuchiki::traits::*;
use kuchiki::NodeRef;

pub struct HtmlDoc {
    pub doc: NodeRef,
}

impl HtmlDoc {
    pub fn parse(path: &str) -> Self {
        let html = std::fs::read_to_string(path).unwrap();
        let doc = kuchiki::parse_html().one(html);
        HtmlDoc { doc }
    }

    pub fn write(&self) -> String {
        self.doc.to_string()
    }

    // pub fn select(&self, selectors) {
    //     self.doc.select("div")
    // }
}

#[test]
fn parse_serrer() {
    let _serrer = HtmlDoc::parse("src/fr.serrer.html");
}

#[test]
fn count_divs() {
    let serrer = HtmlDoc::parse("src/fr.serrer.html");
    let divs = serrer.doc.select("div").unwrap();
    assert_eq!(divs.count(), 159);
}
