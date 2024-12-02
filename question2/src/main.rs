use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use actix_web_actors::ws;

mod utils;
mod routes;

use crate::utils::app_state::AppState;
use crate::routes::handler::event_handlers::MyWebSocket;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();

    env_logger::init();

    let port: u16 = (utils::constants::PORT).clone();
    let address: String = utils::constants::ADDRESS.clone();
    let database_url: String = utils::constants::DATABASE_URL.clone();
    println!("Starting server at http://{}:{}", address, port);

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db: db.clone() }))
        .route("/ws/", web::get().to(|req, db| {
            ws::start(MyWebSocket { db: db.get_ref().clone() }, &req)
        }))
        .wrap(Logger::default())
    
        
        
    })
    .bind((address, port))?
    .run()
    .await
}