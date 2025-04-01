use std::sync::Arc;

use axum::{
    Form, Router,
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use maud::{Markup, html};

mod model;
mod view;

use model::{DishesDb, NewDishForm};
use view::{dish_table, new_dish_form};

type StateDb = State<Arc<DishesDb>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Arc::new(DishesDb::open("test-db").expect("Failed to open database"));
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/dishes") }))
        .route("/dishes", get(dishes))
        .route("/dishes/new", get(new_dish_get))
        .route("/dishes/new", post(new_dish_post))
        .route("/dishes/{name}/delete", post(delete_dish))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn dishes(State(db): StateDb) -> Markup {
    let mut dishes = db.search_dishes("");
    dishes.sort_unstable_by_key(|d| d.last_cooked());
    html! {
        h1 { "Wos kochmer denn heut?" }
        p { a href="/dishes/new" {"Wos neus!"} }
        (dish_table(&dishes))
    }
}

async fn new_dish_get() -> Markup {
    new_dish_form(&[])
}

async fn new_dish_post(State(db): StateDb, Form(input): Form<NewDishForm>) -> Response {
    if input.name.is_empty() {
        return new_dish_form(&["Wie heißt er?"]).into_response();
    }

    // TODO blocked: as soon as `new_dish` returns a Result, use that
    if db.dish_exists(&input.name) {
        new_dish_form(&["Zefix, des gibts scho!"]).into_response()
    } else {
        db.new_dish(input.name).unwrap(); // TODO no unwrap
        Redirect::to("/dishes").into_response()
    }
}

async fn delete_dish(State(db): StateDb, Path(name): Path<String>) -> Redirect {
    db.remove_dish(name).unwrap(); // TODO no unwrap
    Redirect::to("/dishes")
}
