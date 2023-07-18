use std::net::SocketAddr;

use axum::{Router, routing};

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/", routing::get(render_index));

        let addr = ([0, 0, 0, 0], self.port);
        axum::Server::bind(&addr.into())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

async fn render_index() -> &'static str {
    "Hello world!"
}
