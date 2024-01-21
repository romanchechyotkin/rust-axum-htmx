use askama::Template;
use maud::html;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "index.html")]

struct HelloTmpl<'a> {
    name: &'a str,
}

pub async fn maud_handler() -> impl IntoResponse {
    let hello = HelloTmpl { name: "world" };
    (StatusCode::OK, Html(hello.render().unwrap()))
}

pub async fn askama_handler() -> impl IntoResponse {
    let name = "Roman";
    let markup = html! {
        p { "Hi " (name) }
    };
    (StatusCode::OK, Html(markup.into_string()))
}
