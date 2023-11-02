mod exporters;
mod datadog_span;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error, error};
use actix_web::web::Bytes;
use futures::StreamExt;
use exporters::{ConsoleExporter, Exporter, FileExporter};
use datadog_span::DatadogSpan;

#[post("v0.4/traces")]
async fn traces(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    let cur_dir = std::env::current_dir().unwrap();

    // check content-type
    let content_type = request.headers().get("content-type").unwrap();
    if content_type != "application/msgpack" {
        return Err(error::ErrorBadRequest("Content-Type is not application/msgpack"));
    }

    let buf = body.to_vec();
    if buf.len() == 1 {
        return Ok(HttpResponse::Ok().finish());
    }

    let spans: Vec<Vec<DatadogSpan>> = rmp_serde::from_slice(&buf[..]).unwrap();

    let mut console_exporter = ConsoleExporter {};
    console_exporter.export(spans.clone());

    let mut file_exporter = FileExporter::new(format!("{}/traces", cur_dir.to_str().unwrap()));
    file_exporter.export(spans);

    Ok(HttpResponse::Ok().finish()) // <- send response
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello from dd-localstack!"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_exporter = FileExporter::new("traces".to_string());
    HttpServer::new(|| {
        App::new()
            .service(traces)
            .service(index)

    })
        .bind(("127.0.0.1", 8126))?
        .run()
        .await
}