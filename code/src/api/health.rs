use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
