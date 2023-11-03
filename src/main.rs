mod exporters;
mod datadog_span;

use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error, error};
use actix_web::web::Bytes;
use futures::StreamExt;
use serde_json::json;
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

#[get("/httpbin/get")]
async fn httpbin_get(request: HttpRequest) -> Result<HttpResponse, Error> {
    let query_string = request.query_string();
    let args: HashMap<String, String> = query_string
        .split("&")
        .filter_map(|pair| {
            if pair.is_empty() {
                return None;
            }
            let mut key_value = pair.split("=");
            Some((key_value.next().unwrap().to_string(), key_value.next().unwrap().to_string()))
        })
        .collect();

    let headers: HashMap<String, String> = request.headers()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
        .collect();

    let url = request.uri().to_string();

    Ok(HttpResponse::Ok().json(json!({
        "args": args,
        "headers": headers,
        "url": url,
    })))
}

#[post("/httpbin/post")]
async fn httpbin_post(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    let query_string = request.query_string();
    let args: HashMap<String, String> = query_string
        .split("&")
        .filter_map(|pair| {
            if pair.is_empty() {
                return None;
            }
            let mut key_value = pair.split("=");
            Some((key_value.next().unwrap().to_string(), key_value.next().unwrap().to_string()))
        })
        .collect();

    let headers: HashMap<String, String> = request.headers()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
        .collect();

    let url = request.uri().to_string();

    let string_data = String::from_utf8(body.to_vec()).unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "args": args,
        "data": string_data,
        "headers": headers,
        "url": url,
    })))
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
            .service(httpbin_get)
            .service(httpbin_post)

    })
        .bind(("127.0.0.1", 8126))?
        .run()
        .await
}