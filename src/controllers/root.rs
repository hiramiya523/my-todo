use axum::{response::IntoResponse, Router, routing};
use std::sync::Arc;

use crate::database;
use crate::response;
use crate::views::{Home, Tweet};
use crate::controllers::tweets;


// pub fn app() -> Router {
//   Router::new()
//   .route("/", routing::get(get))
//   .nest("/tweets", tweets::tweets())
// }

pub async fn app() -> Router {

  let connection_pool = Arc::new(database::use_connection_pool().await);

  Router::new()
      .route("/", routing::get(get))
      .with_state(connection_pool.clone())
      .nest("/tweets", tweets::tweets(connection_pool.clone()))
}


async fn get() -> impl IntoResponse {
    let tweets = (1..=20)
        .into_iter()
        .map(|_| Tweet {
            name: "太郎".to_string(),
            message: "こんにちは！".to_string(),
            posted_at: "2020-01-01 12:34".to_string(),
        })
        .collect();
    let home = Home { tweets };
    response::from_template(home)
}