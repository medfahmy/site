use axum::{Router, serve};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();
    serve_dir().await;
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "site=debug,tower_http=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn serve_dir() {
    let addr = "127.0.0.1:6969";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);

    let router = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .layer(TraceLayer::new_for_http());

    serve(listener, router).await.unwrap();
}