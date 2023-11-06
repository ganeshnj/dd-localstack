mod exporters;
mod datadog_span;

use std::collections::HashMap;
use std::fmt::format;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error, error};
use actix_web::web::Bytes;
use futures::StreamExt;
use serde_json::json;
use datadog_span::DatadogSpan;
use crate::datadog_span::RUMSpans;
use crate::exporters::{ConsoleStringExporter, FileStringExporter, StringExporter};

static OUTPUT_DIR: &str = "requests";

#[post("v0.4/traces")]
async fn traces(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    let cur_dir = format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), OUTPUT_DIR);

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

    let mut console_exporter = ConsoleStringExporter {};
    let pretty_json = serde_json::to_string_pretty(&spans).unwrap();
    console_exporter.export(pretty_json.clone());

    let mut file_exporter = FileStringExporter::new(format!("{}/traces", cur_dir));
    file_exporter.export(pretty_json);

    Ok(HttpResponse::Ok().finish()) // <- send response
}

#[post("/rum/api/v2/spans")]
async fn rum_spans(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    let cur_dir = format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), OUTPUT_DIR);
    let lines = String::from_utf8(body.to_vec()).unwrap();
    let spans: Vec<RUMSpans> = lines.split("\n").filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        let span = serde_json::from_str::<RUMSpans>(line).unwrap();
        Some(span)
    }).collect();
    let pretty_json = serde_json::to_string_pretty(&spans).unwrap();

    let mut console_exporter = ConsoleStringExporter {};
    console_exporter.export(pretty_json.clone());

    let mut file_exporter = FileStringExporter::new(format!("{}/rum/api/v2/spans", cur_dir));
    file_exporter.export(pretty_json);

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
    let file_exporter = FileStringExporter::new("traces".to_string());
    HttpServer::new(|| {
        App::new()
            .service(traces)
            .service(index)
            .service(httpbin_get)
            .service(httpbin_post)
            .service(rum_spans)

    })
        .bind(("127.0.0.1", 8126))?
        .run()
        .await
}