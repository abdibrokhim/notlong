// #[macro_use]

use actix_web::{web, middleware::Logger};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub mod db;
pub mod routes;
pub mod utils; // <--- so we can call get_pg_url()

// We'll alias this for convenience
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[shuttle_runtime::main]
async fn actix_main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // 1) Get DB URL from your keys.rs (which reads from .env)
    let database_url = utils::keys::get_pg_url();
    
    // 2) Create a Diesel r2d2 pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let diesel_pool = r2d2::Pool::builder()
        .max_size(1) // or higher if you prefer
        .build(manager)
        .map_err(|e| CustomError::new(e))?;

    // 3) Set up your Actix configuration
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            // This empty string "" means we apply it at the root.
            web::scope("")
                .wrap(Logger::default())
                .app_data(web::Data::new(diesel_pool.clone()))
                .service(routes::create_short_url)
                .service(routes::redirect_short)
                .service(routes::create_wallet_route)
                .service(routes::decrement_wallet_route),
        );
    };

    // 4) Return the config as Actix service
    Ok(config.into())
}
