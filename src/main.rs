mod doc;
mod render;

use org_server::Server;

use crate::doc::StaticOrgDoc;
use crate::render::DocRender;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = StaticOrgDoc("* Main heading\n** Sub-heading");
    println!("{}", doc.render());

    let server = Server{ port: 8080 };
    server.start().await?;

    Ok(())
}
