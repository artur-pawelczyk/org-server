mod doc;
mod empty_doc;
mod render;
pub mod server;

use crate::doc::StaticOrgDoc;
use crate::empty_doc::EmptyOrgSource;
use crate::render::DocRender;
use crate::server::Server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = StaticOrgDoc("* Main heading\n** Sub-heading");
    println!("{}", doc.render());

    let server = Server{ port: 8080 };
    server.start(Box::new(EmptyOrgSource)).await?;

    Ok(())
}
