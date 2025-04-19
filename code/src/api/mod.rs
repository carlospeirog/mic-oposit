pub mod primary_teachers;

use actix_web::web;

mod health;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(primary_teachers::config)
            .configure(health::config),
    );
}
