use std::{collections::HashMap, sync::Arc};

use axum::{
    Form, Router,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use clap::Parser;
use maud::Markup;

mod model;
mod view;

use model::{DishesDb, NewDishForm, now};
use view::{landing_page, new_dish_form};

type StateDb = State<Arc<DishesDb>>;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// TCP address to bind on
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    address: String,
    /// Path to database
    #[arg(short, long, default_value = "menu-ninja-db")]
    database: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    println!("Opening database at {}", args.database);
    let db = Arc::new(DishesDb::open(args.database).expect("Failed to open database"));
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/dishes") }))
        .route("/dishes", get(dishes))
        .route("/dishes/new", get(new_dish_get))
        .route("/dishes/new", post(new_dish_post))
        .route("/dishes/{name}/cooked", post(cooked))
        .route("/dishes/{name}/delete", post(delete_dish))
        .with_state(db);

    println!("Binding to address {}", args.address);
    let listener = tokio::net::TcpListener::bind(args.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn dishes(State(db): StateDb, Query(args): Query<HashMap<String, String>>) -> Markup {
    let query = args.get("q").cloned().unwrap_or_default();
    let mut dishes = db.search_dishes(&query);
    dishes.sort_unstable_by_key(|d| d.last_cooked());
    landing_page(&dishes, &query)
}

async fn new_dish_get() -> Markup {
    new_dish_form(&[])
}

async fn new_dish_post(State(db): StateDb, Form(input): Form<NewDishForm>) -> Response {
    if input.name.is_empty() {
        return new_dish_form(&["Wie heißt er?"]).into_response();
    }

    match db.new_dish(input.name).unwrap() {
        // TODO no unwrap
        Ok(_) => Redirect::to("/dishes").into_response(),
        Err(_) => new_dish_form(&["Zefix, des gibts scho!"]).into_response(),
    }
}

async fn cooked(State(db): StateDb, Path(name): Path<String>) -> Redirect {
    db.set_last_cooked(name, now()).unwrap(); // TODO no unwrap
    Redirect::to("/dishes")
}

async fn delete_dish(State(db): StateDb, Path(name): Path<String>) -> Redirect {
    db.remove_dish(name).unwrap(); // TODO no unwrap
    Redirect::to("/dishes")
}
