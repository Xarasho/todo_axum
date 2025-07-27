use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let router = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
    
async fn root() -> &'static str {
    "TODO API!!!"
}

    

