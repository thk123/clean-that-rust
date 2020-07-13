use actix_web::{web, App, HttpServer, Responder};
use crate::store::store::Store;

mod store;

async fn get_clenliness(info: web::Path<String>, store: web::Data<Store>) -> impl
Responder
{
    match store.get_ref().score_of(info.as_str())
    {
        None => { format!("area not found: {}", info.as_str()) }
        Some(score) => { format!("{}", score) }
    }
}

async fn add_room(info: web::Path<String>, store: web::Data<Store>) -> impl
Responder
{
    match store.get_ref().declare_area(info.as_str())
    {
        Ok(_) => { format!("Area {} added", info.as_str())}
        Err(error_string) => {format!("Error: {}", error_string)}
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(Store::initialize_from("clean_database"))
            .service(
                web::resource("/clenliness/{room}").to(get_clenliness)
            )
            .service(web::resource("/add/{room}").to(add_room))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
