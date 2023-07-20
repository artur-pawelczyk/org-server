use std::sync::{Arc, RwLock};

use axum::{Router, routing, extract::{State, Path, self}, response::Response};
use maud::{html, Markup};
use reqwest::StatusCode;
use crate::{doc::OrgSource, empty_doc::EmptyOrgSource};

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start(&self, source: EmptyOrgSource) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(source);
        let app = Router::new()
            .route("/", routing::get(render_index))
            .route("/:filename", routing::get(render_doc))
            .with_state(Arc::clone(&state));

        let addr = ([0, 0, 0, 0], self.port);
        axum::Server::bind(&addr.into())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

async fn render_index(State(source): State<Arc<EmptyOrgSource>>) -> Markup {
    // let mut store = StaticOrgSource::default();
    // store.add_doc("tasks.org", "* TODO First task\n* TODO Next task");
    // store.add_doc("reference.org", "* Links\n** Interesting articles");
    
    let docs = source.list().await;

    html! {
        ul {
            @for doc in docs {
                li { (doc.path) }
            }
        }
    }
}

async fn render_doc(State(source): State<Arc<EmptyOrgSource>>, extract::Path(filename): extract::Path<String>) -> StatusCode {
    StatusCode::NOT_FOUND
}
