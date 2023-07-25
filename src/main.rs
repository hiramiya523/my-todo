use axum::{response::{Html, IntoResponse}, Router, routing::get};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use askama::Template;


#[tokio::main]  // main関数を非同期関数にするために必要
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustwi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let app = Router::new().route("/", get(handler));
    // ! プロジェクト名と一致させる必要あり
    // let app = rustwi::app();
    let app = rustwi::app().await;


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// async fn handler() -> Html<&'static str> {
//     Html("<h1>Hello, World!</h1>")
// }

// async fn handler() -> impl IntoResponse {
//   let tweets = (1..=20)
//       .into_iter()
//       .map(|_| TweetView {
//           name: "太郎".to_string(),
//           message: "こんにちは！".to_string(),
//           posted_at: "2020-01-01 12:34".to_string(),
//       })
//       .collect();
//   let home = Home { tweets };
//   Html(home.render().unwrap()).into_response()
// }


// #[derive(Template)]
// #[template(path = "home.html")]
// struct Home {
//     tweets: Vec<TweetView>,
// }

// struct TweetView {
//     name: String,
//     message: String,
//     posted_at: String,
// }