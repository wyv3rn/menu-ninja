use axum::{Router, routing::get};
use maud::{Markup, html};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    let greetings = "Hello!";
    html! {
        h1 { (greetings)}
        p { "Hier könnte ihr Paragraph stehen!" }
        h2 { "Next one, please!" }
        p { "Gracias!" }
    }
}
