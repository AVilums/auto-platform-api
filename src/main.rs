use actix_web::{get, post, patch, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Data, Json, Path};
use validator::Validate;
use crate::models::{BuyPizzaRequest, UpdatePizzaURL };
use crate::db::Database;

mod config;
mod models;
mod db;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizas().await;
    match pizzas {
        Some(found_pizzas) =>
            HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None =>
            HttpResponse::Unauthorized().body("No pizzas"),
    }
}

#[post("/buy_pizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        },
        Err(_) => HttpResponse::BadRequest().body("Pizza name invalid"),
    }
}

#[patch("/update_pizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid: String = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating the pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // initialize db
    let db = Database::init().await.expect("Error connecting to database");
    let db_data = Data::new(db);

    // initialize http server
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
