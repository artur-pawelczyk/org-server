use std::sync::atomic::{AtomicU16, Ordering};

use org_server::{empty_doc::EmptyOrgSource, doc::{OrgSource, StaticOrgSource}};
use reqwest::StatusCode;
use scraper::{Html, Selector, ElementRef};


#[tokio::test]
async fn test_connect() {
    let TestServer { port } = prepare_server(EmptyOrgSource).await;

    let resp = reqwest::get(format!("http://0.0.0.0:{port}"))
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_doc_not_found() {
    let TestServer { port } = prepare_server(EmptyOrgSource).await;

    let resp = reqwest::get(format!("http://0.0.0.0:{port}/tasks.org"))
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_static_org_source() {
    let mut source = StaticOrgSource::default();
    source.add_doc("tasks.org", "the content");
    let TestServer { port } = prepare_server(source).await;

    let resp = reqwest::get(format!("http://0.0.0.0:{port}/")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = resp.text().await.unwrap();
    assert!(text.contains("tasks.org"));

    let resp = reqwest::get(format!("http://0.0.0.0:{port}/tasks.org")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = resp.text().await.unwrap();
    assert!(text.contains("the content"));
}

#[tokio::test]
async fn test_list_todo() {
    let mut source = StaticOrgSource::default();
    source.add_doc("tasks.org", "
* TODO Get stuff
* DONE Buy stuff
* TODO Do stuff
");
    let TestServer { port } = prepare_server(source).await;

    let resp = reqwest::get(format!("http://0.0.0.0:{port}/todo/TODO")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let html = Html::parse_fragment(&resp.text().await.unwrap());

    let selector = Selector::parse("ol > li").unwrap();
    let elements: Vec<String> = html.select(&selector)
        .map(element_to_text)
        .collect();

    assert_eq!(elements, ["TODO Get stuff", "TODO Do stuff"]);
}

static PORT_NUMBER: AtomicU16 = AtomicU16::new(8000);

struct TestServer {
    port: u16,
}

#[must_use]
async fn prepare_server(source: impl OrgSource + 'static) -> TestServer {
    let port = PORT_NUMBER.fetch_add(1, Ordering::Relaxed);
    println!("using port: {port}");
    let app = org_server::server::Server{ port };
    tokio::spawn(async move {
        app.start(source).await.unwrap();
    });

    let server = TestServer { port };
    wait_for_server(&server).await;
    server
}

async fn wait_for_server(TestServer { port }: &TestServer) {
    for _ in 0..5 {
        match reqwest::get(format!("http://0.0.0.0:{port}")).await {
            Ok(_) => return,
            Err(_) => continue,
        }
    }
}

fn element_to_text(element: ElementRef) -> String {
    element.text().collect()
}
