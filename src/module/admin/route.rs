use actix_web::{web, post};
use crate::module::admin::controller::AdminController;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(login)
    );
}

#[post("/login")]
pub async fn login(
    data: web::Json<serde_json::Value>,
    admin_controller: web::Data<AdminController>,
) -> web::Json<serde_json::Value> {
    admin_controller.handle_login(data.into_inner()).await
}
