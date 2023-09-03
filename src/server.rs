use axum::{Router, routing, extract, extract::State};
use maud::{html, Markup};
use reqwest::StatusCode;

use crate::doc::{OrgDoc, OrgSource};

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start<D, S>(&self, source: S) -> Result<(), Box<dyn std::error::Error>>
    where D: OrgDoc + 'static,
          S: OrgSource<Doc = D> + 'static
    {
        let state = Box::leak(Box::new(source));
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

async fn render_index<D, S>(State(source): State<&S>) -> Markup
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
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

async fn render_doc<D, S>(State(source): State<&S>,
                       extract::Path(filename): extract::Path<String>) -> Result<String, StatusCode>
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    let filename = format!("/{filename}");
    source.read(&filename).await
        .map(|doc| doc.content().to_string())
        .map_err(|_| StatusCode::NOT_FOUND)
}
