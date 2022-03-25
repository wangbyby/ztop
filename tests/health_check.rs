use zero2prod::run;
use reqwest;
use tokio;

fn spawn_app() {
    let server = run().expect("Failed to run");
    tokio::spawn(server);
}

#[tokio::test]
async fn health_check(){
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
    .get("http://127.0.0.1:9999/health_check")
    .send()
    .await 
    .expect("Failed to execute health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}