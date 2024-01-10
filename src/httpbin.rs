use actix_web::{Error, get, HttpRequest, HttpResponse, post};
use std::collections::HashMap;
use actix_web::web::Bytes;
use serde_json::json;

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
