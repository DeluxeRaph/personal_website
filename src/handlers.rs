use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use crate::{content, views};

pub(crate) async fn healthz() -> &'static str {
    "ok"
}

pub(crate) async fn home() -> Html<String> {
    Html(views::layout("Raphael Nembhard", views::home()))
}

pub(crate) async fn error_redirect() -> Redirect {
    Redirect::to("/")
}

pub(crate) async fn blog_index() -> Html<String> {
    Html(views::layout(
        "Blog - Raphael Nembhard",
        views::blog_index(),
    ))
}

pub(crate) async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(views::layout("Page not found", views::not_found())),
    )
}

pub(crate) async fn blog_post(Path(slug): Path<String>) -> impl IntoResponse {
    let Some(post) = content::find_blog_post(&slug) else {
        return (
            StatusCode::NOT_FOUND,
            Html(views::layout("Post not found", views::blog_not_found())),
        );
    };

    (
        StatusCode::OK,
        Html(views::layout(post.title, views::blog_post(post))),
    )
}
