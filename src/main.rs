use actix_web::{server, App, HttpRequest};
use chrono::prelude::*;
use std::env;

fn beats(_req: &HttpRequest) -> String {
    let utc = Utc::now();
    let cet = utc.with_timezone(&FixedOffset::east(3600));
    let seconds = cet.num_seconds_from_midnight() as f32;
    let beats = (seconds * 0.011_574) as u32;
    let mut prefix = "";

    if beats < 10 {
        prefix = "00"
    }
    if beats > 10 && beats < 100 {
        prefix = "0"
    }

    format!("@{}{}.beats", prefix, beats)
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(beats))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}
