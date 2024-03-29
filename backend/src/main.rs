use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub mod api;
pub mod app_state;
pub mod repository;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db_connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections = std::env::var("MAX_CONNECTIONS").expect("MAX_CONNECTIONS must be set");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections.parse::<u32>().unwrap())
        .connect(&db_connection_url)
        .await?;

    // migrate the db
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Start the http server
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::permissive();

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(web::Data::new(app_state::AppState { pool: pool.clone() }))
            .route("/", web::get().to(HttpResponse::Ok))
            .service(
                web::scope("/api")
                    .service(api::default_route)
                    .configure(api::notes::service_config),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    return Ok(());
}
