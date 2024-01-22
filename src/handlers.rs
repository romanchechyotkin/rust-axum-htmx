use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use maud::html;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTmpl<'a> {
    name: &'a str,
}

pub async fn index() -> impl IntoResponse {
    (StatusCode::OK, Html(include_str!("../templates/base.html")))
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
