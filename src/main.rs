use askama_axum::Template;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

async fn index_handler() -> IndexTemplate {
    IndexTemplate {
        name: "World".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(index_handler));

    let listener = TcpListener::bind("127.0.0.1:5900").await?;
    println!("Listening on http://127.0.0.1:5900");

    axum::serve(listener, app).await?;
    Ok(())
}

