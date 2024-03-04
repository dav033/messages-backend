use actix::SyncArbiter;
use dotenv::dotenv;
use std::env;
mod actors;
mod db_messages;
mod db_utils;
mod insertables;
mod models;
mod schema;
mod services;
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};


use actix_web::{ App, HttpServer};

use services::send_messages;

use db_utils::{get_pool, AppState, DbActor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<MysqlConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(send_messages)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


