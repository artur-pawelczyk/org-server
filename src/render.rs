use orgize::{Org, Event, Element};
use std::fmt::Write;

use crate::doc::{OrgDoc, StaticOrgDoc};

fn render(content: impl AsRef<str>) -> String {
    let org = Org::parse(content.as_ref());
    let mut out = String::new();

    for event in org.iter() {
        match event {
            Event::Start(Element::Title(title)) => {
                write!(out, "<h{level}>{}</h{level}>", title.raw, level = title.level).expect("Writing to string should never fail");
            },
            _ => {},
        }
    }

    out
}

pub(crate) trait DocRender: OrgDoc {
    fn render(&self) -> String;
}

impl DocRender for StaticOrgDoc {
    fn render(&self) -> String {
        render(self.content())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::StaticOrgDoc;

    #[test]
    fn test_render_emtpy_doc() {
        let doc = StaticOrgDoc("");
        assert_eq!(doc.render(), "");
    }

    #[test]
    fn test_render_heading() {
        let doc_1 = StaticOrgDoc("* Main heading");
        assert_eq!(doc_1.render(), "<h1>Main heading</h1>");

        let doc_2 = StaticOrgDoc("** Sub-heading");
        assert_eq!(doc_2.render(), "<h2>Sub-heading</h2>");
    }
}
