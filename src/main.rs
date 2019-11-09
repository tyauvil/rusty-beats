use ::rusty_beats::beats;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use std::env;

fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(beats()))
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("0.0.0.0", port))?
    .run()
}
