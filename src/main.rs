use askama::Template;
use axum::{http::StatusCode, response::{IntoResponse, Html}, routing::get, Router};
use maud::html;
use tokio;

#[derive(Template)]
#[template(path = "index.html")]

struct HelloTmpl<'a> {
    name: &'a str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/askama", get(askama_handler))
        .route("/maud", get(maud_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn maud_handler() -> impl IntoResponse {
    let hello = HelloTmpl { name: "world" };
    (StatusCode::OK, Html(hello.render().unwrap())) 
}

async fn askama_handler() -> impl IntoResponse {
    let name = "Roman";
    let markup = html! {
        p { "Hi " (name) }
    };
    (StatusCode::OK, Html(markup.into_string())) 
}
