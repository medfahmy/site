use maud::{html, Markup, Render, DOCTYPE};
use std::fs::File;
use std::io::Write;
use axum::{Router, serve};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();

    let index = page("mohamed fahmy", index());
    let mut output = String::new();
    index.render_to(&mut output);

    let path = "public/index.html";
    std::fs::remove_file(path).unwrap();
    let mut file = File::create(path).unwrap();
    write!(file, "{}", output).unwrap();

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


fn index() -> Markup {
    html! {
        h1 { "welcome to my website" }
    }
}

fn page(title: &str, content: Markup) -> Markup {
    html! {
        (head(title))
        body ."theme-base-0e" {
            (sidebar())
            main ."content container" {
                (content)
            }
        }
    }
}

fn head(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en"
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" type="text/css" href="/style.css";
            title { (title) }
        }
    }
}

fn sidebar() -> Markup {
    html! {
        aside ."sidebar" {
            div ."container sidebar-sticky" {
                div ."sidebar-about" {
                    a href="/" {
                        h1 { "med fahmy" }
                    }
                }

                @let nav = [
                    ("about", "About Me"),
                    ("blog", "Blog"),
                    ("projects", "Projects"),
                ];

                ul ."sidebar-nav" {
                    @for (link, title) in nav {
                        li ."sidebar-nav-item" {
                            a href=(link) { (title) }
                        }
                    }
                }

                div ."search-container" {
                    input id="search" type="search" placeholder="search" disabled;
                }

                div ."copyright" {
                    "Copyright &copy;2024 Mohamed Fahmy"
                    br;
                    a href="contact" { "Contact" }
                    "&mdash;"
                    a href="rss" { "RSS" }
                }
            }
        }
    }
}
