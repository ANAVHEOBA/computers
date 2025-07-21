use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use dotenv::dotenv;

mod app;
mod environment;
mod module;
mod service;
mod middleware;

use crate::module::user::crud::UserCrud;
use crate::module::user::controller::UserController;
use crate::middleware::Authentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Setting an environment variable is an unsafe operation because it can cause
    // data races if other threads are reading/writing environment variables at the
    // same time. However, it is safe to do here at the start of the `main`
    // function, before the server spawns any worker threads.
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // Connect to database
    let db = environment::connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");

    let user_crud = UserCrud::new(db.clone());
    let user_controller = web::Data::new(UserController::new(user_crud));
    let db_data = web::Data::new(db);

    println!("🚀 Server starting on http://127.0.0.1:8080");

    // Start server
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(Cors::permissive())
            .wrap(actix_middleware::NormalizePath::trim())
            .wrap(Authentication::new()) // Our custom auth middleware
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY")),
            )
            // App data
            .app_data(user_controller.clone())
            .app_data(db_data.clone())
            // Configure services/routes
            .configure(app::configure_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
