use std::fs::File;
use std::io::Write;
use maud::{html, Markup, Render, DOCTYPE};

fn main() {
    let index = page("mohamed fahmy", index());
    let mut output = String::new();
    index.render_to(&mut output);

    let path = "public/index.html";
    // std::fs::remove_file(path).unwrap();

    let mut file = File::create(path).unwrap();
    write!(file, "{}", output).unwrap();
}

fn index() -> Markup {
    let content = std::fs::read_to_string("content/blog/intro.md").unwrap();

    html! {
        h1 { "Lorem Ipsum" }
        p { (content) }
    }
}

fn page(title: &str, content: Markup) -> Markup {
    html! {
        (head(title))
        body {
            div ."wrapper" {
                div ."sidebar" { (header()) }

                main ."main" {
                    (content)
                }
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

fn header() -> Markup {
    let nav = vec![
        ("about", "About"),
        ("blog", "Blog"),
        ("projects", "Projects"),
        ("contact", "Contact"),
    ];

    html! {
        div ."sidebar-content" {
            div ."name" {
                a href="/" {
                    h1 { "Mohamed Fahmy" }
                }
            }

            nav {
                ul ."sidebar-nav" {
                    @for (link, title) in nav {
                        li ."nav-item" {
                            a href=(link) { (title) }
                        }
                    }
                }
            }

            input id="search" type="search" placeholder="Search";
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            p { "footer" }
        }
    }
}