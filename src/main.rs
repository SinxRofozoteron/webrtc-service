#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use settings::Settings;
use sqlx::postgres::PgPool;

use route_handlers::*;

mod auth;
mod constants;
mod errors;
mod models;
mod route_handlers;
mod settings;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("Unable to get configuration");
    let cookies_key = Key::from(config.keys.cookies_key.as_bytes());

    let config_copy = config.clone();
    let auth_clients = web::block(move || state::setup_auth_clients(&config_copy))
        .await
        .expect("Error happened during the process of setting up OAuth clients");

    let db_pool = PgPool::connect(&config.postgres.url)
        .await
        .expect("Failed to connect to postgres database");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookies_key.clone())
                    .cookie_name("webrtc-auth".to_owned())
                    // TODO: Configure for production
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/auth")
                            .app_data(web::Data::new(auth_clients.clone()))
                            .service(oauth_start)
                            .service(oauth_get_access_token),
                    )
                    .configure(users_service_config),
            )
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
