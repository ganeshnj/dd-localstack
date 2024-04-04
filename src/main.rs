use std::io::Read;

use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::Bytes;
use futures::StreamExt;
use uuid::Uuid;

use crate::exporters::{FileExporter, JSONExporter};
use crate::profiles::{FileWriter, profiling};
use crate::rum::rum_spans;
use crate::tracing::traces;

mod exporters;
mod datadog_span;
mod tracing;
mod httpbin;
mod rum;
mod profiles;

mod profile;

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
            .service(traces)
            .service(profiling)
            .service(rum_spans)

    })
        .bind(("127.0.0.1", 8126))?
        .run()
        .await
}

async fn default_service(request: HttpRequest, body: Bytes) -> Result<HttpResponse, Error> {
    let path = request.path();

    // headers
    let mut headers = std::string::String::new();
    for (name, value) in request.headers().iter() {
        headers.push_str(&format!("{}: {}\n", name, value.to_str().unwrap()));
    }
    let mut request_id = Uuid::new_v4().to_string();
    // find request id from headers
    for (name, value) in request.headers().iter() {
        let sanitized_name = name.as_str().to_lowercase();
        if sanitized_name == "dd-request-id" {
            request_id = value.to_str().unwrap().to_string();
        }
    }

    println!("Processing request: {}", request_id);

    let dir = format!("{}{}/{}", OUTPUT_DIR, path, request_id);
    let mut file_writer = FileWriter::new(dir);
    file_writer.export_string("headers".to_string(), headers);

    // write body to file
    // check if body is gzipped
    let mut is_gzipped = false;
    for (name, value) in request.headers().iter() {
        let sanitized_name = name.as_str().to_lowercase();
        if sanitized_name == "content-encoding" {
            if value.to_str().unwrap() == "gzip" {
                is_gzipped = true;
            }
        }
    }

    let content_type = request.headers().get("content-type").unwrap().to_str().unwrap();
    if content_type == "application/msgpack" {
        file_writer.export_bytes("body", body.to_vec());
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{\"request_id\": \"{}\"}}", request_id)));
    }

    if is_gzipped {
        let body = body.to_vec();
        let mut decoder = flate2::read::GzDecoder::new(&body[..]);
        let mut s = std::string::String::new();
        decoder.read_to_string(&mut s).unwrap();
        file_writer.export_string("body".to_string(), s);
    } else {
        file_writer.export_bytes("body", body.to_vec());
    }

    // return request id as response
    let response = format!("{{\"request_id\": \"{}\"}}", request_id);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}
