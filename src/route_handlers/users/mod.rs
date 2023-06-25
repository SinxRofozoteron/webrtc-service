use actix_web::web;

mod create;

use create::create_user;

pub fn users_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(create_user));
}
