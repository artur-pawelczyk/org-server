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
    let docs: Vec<_> = source.list().await.iter()
        .map(|doc| (source.doc_name(&doc), format!("{doc}")))
        .collect();

    html! {
        ul {
            @for doc in docs {
                li { a href = (doc.1) { (doc.0) } }
            }
        }
    }
}

async fn render_doc<D>(State(source): State<&'static dyn OrgSource<Doc = D>>,
                       extract::Path(filename): extract::Path<String>
) -> Result<String, StatusCode>
where D: OrgDoc
{
    let filename = format!("/{filename}");
    source.read(&filename).await
        .map(|doc| doc.content().to_string())
        .map_err(|_| StatusCode::NOT_FOUND)
}
