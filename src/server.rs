use axum::{Router, routing, extract, extract::State};
use maud::{html, Markup};
use reqwest::StatusCode;
use crate::doc::OrgSource;

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn start(&self, source: Box<dyn OrgSource>) -> Result<(), Box<dyn std::error::Error>> {
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

async fn render_index(State(source): State<&'static dyn OrgSource>) -> Markup {
    // let mut store = StaticOrgSource::default();
    // store.add_doc("tasks.org", "* TODO First task\n* TODO Next task");
    // store.add_doc("reference.org", "* Links\n** Interesting articles");
    
    let docs = source.list().await;

    html! {
        ul {
            @for doc in docs {
                li { (doc) }
            }
        }
    }
}

async fn render_doc(State(source): State<&'static dyn OrgSource>, extract::Path(filename): extract::Path<String>) -> Result<String, StatusCode> {
    if let Some(doc_ref) = source.list().await.iter().find(|doc| doc == &&filename) {
        Ok(String::from(source.read(&doc_ref).await.content()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
