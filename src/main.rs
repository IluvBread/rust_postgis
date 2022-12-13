use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv;
use tokio_postgres::NoTls;

mod db;
mod models;
mod config;
mod errors;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("C:\\rust\\postgres\\src\\.env").ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: config::ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users")
                .route(web::post().to(handlers::add_user))
                .route(web::get().to(handlers::get_users)))
            .service(web::resource("/test")
                .route(web::get().to(handlers::test)))
            .service(web::resource("/location")
                .route(web::post().to(handlers::add_location))
                .route(web::get().to(handlers::get_locations)))
            .service(web::resource("/location/timeslot")
                .route(web::post().to(handlers::add_timestamp_to_location))
                .route(web::delete().to(handlers::delete_timeslot)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
