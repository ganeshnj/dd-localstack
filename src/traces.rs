use actix_web::{Error, error, HttpRequest, HttpResponse, post};
use actix_web::web::Bytes;
use crate::datadog_span::DatadogSpan;
use crate::exporters::{ConsoleExporter, FileExporter, JSONExporter};
use crate::OUTPUT_DIR;

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

    let mut console_exporter = ConsoleExporter {};
    let pretty_json = serde_json::to_string_pretty(&spans).unwrap();
    console_exporter.export(pretty_json.clone());

    let mut file_exporter = FileExporter::new(format!("{}/traces", cur_dir));
    file_exporter.export(pretty_json);

    Ok(HttpResponse::Ok().finish()) // <- send response
}
