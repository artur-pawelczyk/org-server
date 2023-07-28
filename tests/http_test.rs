use org_server::{empty_doc::EmptyOrgSource, doc::{OrgSource, StaticOrgSource}};
use reqwest::StatusCode;
use tokio::task::JoinHandle;

#[tokio::test]
async fn test_connect() {
    let handle = prepare_server(EmptyOrgSource).await;

    let resp = reqwest::get("http://0.0.0.0:8080")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // TODO: Provide a better setup/teardown logic
    handle.abort();
}

#[tokio::test]
async fn test_doc_not_found() {
    let handle = prepare_server(EmptyOrgSource).await;

    let resp = reqwest::get("http://0.0.0.0:8080/tasks.org")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    handle.abort();
}

#[tokio::test]
async fn test_static_org_source() {
    let mut source = StaticOrgSource::default();
    source.add_doc("tasks.org", "the content");
    let handle = prepare_server(source).await;

    let resp = reqwest::get("http://0.0.0.0:8080/")
        .await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = resp.text().await.unwrap();
    assert!(text.contains("tasks.org"));

    let resp = reqwest::get("http://0.0.0.0:8080/tasks.org")
        .await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = resp.text().await.unwrap();
    assert!(text.contains("the content"));


    handle.abort();
}

#[must_use]
async fn prepare_server(source: impl OrgSource + 'static) -> JoinHandle<()> {
    let app = org_server::server::Server{ port: 8080 };
    let handle = tokio::spawn(async move {
        app.start(Box::new(source)).await.unwrap();
    });

    wait_for_server().await;

    handle
}

async fn wait_for_server() {
    for _ in 0..5 {
        match reqwest::get("http://0.0.0.0:8080").await {
            Ok(_) => return,
            Err(_) => continue,
        }
    }
}
