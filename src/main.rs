mod exporters;
mod datadog_span;
mod traces;
mod httpbin;
mod rum;
mod profiles;

mod profile;

use actix_web::{App, get, HttpServer, Responder};
use futures::StreamExt;
use crate::exporters::{FileExporter, JSONExporter};
use crate::httpbin::{httpbin_get, httpbin_post};
use crate::profiles::profiling;
use crate::rum::rum_spans;
use crate::traces::traces as other_traces;

static OUTPUT_DIR: &str = "requests";

#[get("/")]
async fn index() -> impl Responder {
    "Hello from dd-localstack!"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_exporter = FileExporter::new("traces".to_string());
    HttpServer::new(|| {
        App::new()
            .service(other_traces)
            .service(index)
            .service(httpbin_get)
            .service(httpbin_post)
            .service(rum_spans)
            .service(profiling)

    })
        .bind(("127.0.0.1", 8126))?
        .run()
        .await
}
