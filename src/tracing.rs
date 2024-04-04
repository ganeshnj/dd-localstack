use actix_web::{Error, error, HttpRequest, HttpResponse, post};
use actix_web::web::Bytes;
use uuid::Uuid;
use crate::datadog_span::DatadogSpan;
use crate::exporters::{ConsoleExporter, FileExporter, JSONExporter};
use crate::{default_service, OUTPUT_DIR};
use crate::profiles::FileWriter;

#[post("v0.4/traces")]
async fn traces(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    // check content-type
    let content_type = request.headers().get("content-type").unwrap();
    if content_type != "application/msgpack" {
        return Err(error::ErrorBadRequest("Content-Type is not application/msgpack"));
    }

    let path = "v0.4/traces/";

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
    let response = format!("{{\"request_id\": \"{}\"}}", request_id);

    let dir = format!("{}{}/{}", OUTPUT_DIR, path, request_id);
    let mut file_writer = FileWriter::new(dir);
    file_writer.export_string("headers".to_string(), headers);

    let buf = body.to_vec();
    let spans: Vec<Vec<DatadogSpan>> = rmp_serde::from_slice(&buf[..]).unwrap();

    let json = serde_json::to_string(&spans).unwrap();
    file_writer.export_string("body".to_string(), json);

    // return request id as response

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}
