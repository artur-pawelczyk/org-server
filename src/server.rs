use axum::{Router, routing, extract, extract::State, http::StatusCode, response::Html};
use maud::{html, Markup};

use crate::{doc::{OrgDoc, OrgSource}, parser::{self, TodoItem, ParserConfig}};

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
    pub async fn start<D, S>(&self, source: S) -> Result<(), Box<dyn std::error::Error>>
    where D: OrgDoc + 'static,
          S: OrgSource<Doc = D> + 'static
    {
        let state = Box::leak(Box::new(source));
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

async fn list_todos<D, S>(State(source): State<&S>,
                          extract::Path(keyword): extract::Path<String>) -> Result<Html<String>, StatusCode>
where D: OrgDoc,
      S: OrgSource<Doc = D>
{
    // TODO: Get the config from state
    let parser_conf = ParserConfig::with_keywords(&["TODO", "NEW", "NEXT"], &["DONE"]);

    let mut items = String::new();
    for path in source.list().await {
        let doc = source.read(&path).await.unwrap();
        let content = doc.content();
        parser::doc_to_items(content, &parser_conf, |item| {
            dbg!(&item);
            if item.keyword() == keyword {
                items.push_str(&format!("<li><strong>{}</strong> {}</li>", item.keyword(), item.heading()));
            }
        });
    }

    Ok(Html::from(format!("<ol>{}</ol>", items)))
}
