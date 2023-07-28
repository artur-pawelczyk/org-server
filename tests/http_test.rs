use org_server::empty_doc::EmptyOrgSource;
use reqwest::StatusCode;
use tokio::task::JoinHandle;

#[tokio::test]
async fn test_connect() {
    let handle = prepare_server().await;

    let resp = reqwest::get("http://0.0.0.0:8080")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // TODO: Provide a better setup/teardown logic
    handle.abort();
}

#[tokio::test]
async fn test_doc_not_found() {
    let handle = prepare_server().await;

    let resp = reqwest::get("http://0.0.0.0:8080/tasks.org")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    handle.abort();
}

#[must_use]
async fn prepare_server() -> JoinHandle<()> {
    let app = org_server::server::Server{ port: 8080 };
    let handle = tokio::spawn(async move {
        app.start(Box::new(EmptyOrgSource)).await.unwrap();
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
