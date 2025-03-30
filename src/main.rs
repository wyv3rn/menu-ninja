use std::sync::Arc;

use axum::{Router, extract::State, response::Redirect, routing::get};
use maud::{Markup, html};

mod model;
mod view;

use model::DishesDb;
use view::dish_table;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Arc::new(DishesDb::open("test-db").expect("Failed to open database"));
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/overview") }))
        .route("/overview", get(overview))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn overview(State(db): State<Arc<DishesDb>>) -> Markup {
    let mut dishes = db.search_dishes("");
    dishes.sort_unstable_by_key(|d| d.last_cooked());
    html! {
        h1 { "Wos kochmer denn heut?" }
        (dish_table(&dishes))
    }
}
