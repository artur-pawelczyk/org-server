use crate::doc::StaticOrgDoc;
use crate::render::DocRender;

mod doc;
mod render;

pub fn main() {
    let doc = StaticOrgDoc("* Main heading\n** Sub-heading");
    println!("{}", doc.render());
}
