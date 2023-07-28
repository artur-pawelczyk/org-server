use axum::{Router, routing, extract, extract::State};
use maud::{html, Markup};
use reqwest::StatusCode;
use crate::{doc::OrgSource, empty_doc::{EmptyOrgSource, EmptyDoc}};

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start(&self, source: Box<dyn OrgSource<Doc = EmptyDoc>>) -> Result<(), Box<dyn std::error::Error>> {
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

async fn render_index(State(source): State<&'static dyn OrgSource<Doc = EmptyDoc>>) -> Markup {
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

async fn render_doc(State(_source): State<&'static dyn OrgSource<Doc = EmptyDoc>>, extract::Path(_filename): extract::Path<String>) -> StatusCode {
    StatusCode::NOT_FOUND
}
