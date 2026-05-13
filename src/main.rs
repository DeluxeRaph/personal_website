use personal_website::app;

#[tokio::main]
async fn main() {
    let app = app::router();
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let addr = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|error| panic!("failed to bind {addr}: {error}"));

    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.expect("serve app");
}
