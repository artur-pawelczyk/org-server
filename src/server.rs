use axum::{Router, routing, extract, extract::State};
use maud::{html, Markup};
use reqwest::StatusCode;

use crate::doc::{OrgDoc, OrgSource};

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start<D: OrgDoc + 'static>(&self, source: Box<dyn OrgSource<Doc = D>>) -> Result<(), Box<dyn std::error::Error>> {
        let state = Box::leak(source);
        let app = Router::new()
            .route("/", routing::get(render_index))
            .route("/:filename", routing::get(render_doc))
            .with_state(state);

        let addr = ([0, 0, 0, 0], self.port);
        axum::Server::bind(&addr.into())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

async fn render_index<D: OrgDoc>(State(source): State<&'static dyn OrgSource<Doc = D>>) -> Markup {
    let docs = source.list().await;

    html! {
        ul {
            @for doc in docs {
                li { a href = (doc) { (source.doc_name(&doc)) } }
            }
        }
    }
}

async fn render_doc<D>(State(source): State<&'static dyn OrgSource<Doc = D>>,
                       extract::Path(filename): extract::Path<String>
) -> Result<String, StatusCode>
where D: OrgDoc
{
    source.read(&filename).await
        .map(|doc| doc.content().to_string())
        .map_err(|_| StatusCode::NOT_FOUND)
}
