use axum::{Router, routing, extract, extract::State, http::{StatusCode, Request}, response::{Html, Response}, middleware::{Next, self}, body::{Body, HttpBody}};
use maud::{html, Markup, PreEscaped};

use crate::{doc::{OrgDoc, OrgSource}, parser::{self, ParserConfig}};

pub struct Server {
    pub port: u16,
    pub parser_config: ParserConfig,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8080,
            parser_config: ParserConfig::with_keywords(&["NEW", "NEXT"], &["DONE"])
        }
    }
}

impl Server {
    pub async fn start<D, S>(self, source: S) -> Result<(), Box<dyn std::error::Error>>
    where D: OrgDoc + 'static,
          S: OrgSource<Doc = D> + 'static
    {
        let state = Box::leak(Box::new(ServerState{
            source, parser_config: self.parser_config,
        }));

        let app = Router::new()
            .route("/", routing::get(render_index))
            .route("/:filename", routing::get(render_doc))
            .route("/todo/:keyword", routing::get(list_todos))
            .with_state(state);

        let addr = ([0, 0, 0, 0], self.port);
        axum::Server::bind(&addr.into())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

struct ServerState<D, S>
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    source: S,
    parser_config: ParserConfig,
}

async fn render_index<D, S>(State(state): State<&ServerState<D, S>>) -> Markup
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    let paths = state.source.list().await;
    let docs: Vec<_> = paths.iter()
        .map(|path| (state.source.doc_name(path), path))
        .collect();

    html! {
        ul {
            @for doc in docs {
                li { a href = (doc.1) { (doc.0) } }
            }
        }
    }
}

async fn render_doc<D, S>(State(state): State<&ServerState<D, S>>,
                       extract::Path(filename): extract::Path<String>) -> Result<Markup, StatusCode>
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    let filename = format!("/{filename}");
    state.source.read(&filename).await
        .map(|doc| html!{ pre { (doc.content()) } })
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn list_todos<D, S>(State(state): State<&ServerState<D, S>>,
                          extract::Path(keyword): extract::Path<String>) -> Result<Markup, StatusCode>
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    let mut items = String::new();
    for path in state.source.list().await {
        let doc = state.source.read(&path).await.unwrap();
        let content = doc.content();
        parser::doc_to_items(content, &state.parser_config, |item| {
            if item.keyword() == keyword {
                items.push_str(&format!("<li><strong>{}</strong> {}</li>", item.keyword(), item.heading()));
            }
        });
    }

    Ok(html! {
        ol {
            (PreEscaped(items))
        }
    })
}
