pub mod users;

use actix_web::web;

mod health;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(users::config)
            .configure(health::config),
    );
}
