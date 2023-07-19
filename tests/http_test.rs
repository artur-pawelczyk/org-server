use org_server::empty_doc::EmptyOrgSource;
use reqwest::StatusCode;

#[tokio::test]
async fn test_connect() {
    prepare_server().await;

    let resp = reqwest::get("http://0.0.0.0:8080")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_doc_not_found() {
    prepare_server().await;

    let resp = reqwest::get("http://0.0.0.0:8080/tasks.org")
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

async fn prepare_server() {
    let app = org_server::server::Server{ port: 8080 };
    tokio::spawn(async move {
        app.start(EmptyOrgSource).await.unwrap();
    });

    wait_for_server().await;
}

async fn wait_for_server() {
    for _ in 0..5 {
        match reqwest::get("http://0.0.0.0:8080").await {
            Ok(_) => return,
            Err(_) => continue,
        }
    }
}