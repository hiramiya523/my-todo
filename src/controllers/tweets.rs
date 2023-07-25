use axum::{extract::Form, response::IntoResponse, Router, routing};
use serde::Deserialize;
use std::sync::Arc;


use crate::database::ConnectionPool;
use crate::response;
use crate::views::{Home, Tweet};

pub fn tweets(pool: Arc<ConnectionPool>) -> Router {
    Router::new().route("/new", routing::post(post))
      .with_state(pool)
}

async fn post(form: Form<TweetForm>) -> impl IntoResponse {
    let tweets = vec![Tweet {
        name: "太郎".to_string(),
        message: form.message.clone(),
        posted_at: "2020-01-01 12:34".to_string(),
    }];
    let home = Home { tweets };
    response::from_template(home)
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}