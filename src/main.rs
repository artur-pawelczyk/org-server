mod doc;
mod render;

use axum::{routing, Server, Router};

use crate::doc::StaticOrgDoc;
use crate::render::DocRender;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = StaticOrgDoc("* Main heading\n** Sub-heading");
    println!("{}", doc.render());

    let app = Router::new()
        .route("/", routing::get(render_index));

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn render_index() -> &'static str {
    "Hello world!"
}
