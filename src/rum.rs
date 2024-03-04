use actix_web::{Error, HttpRequest, HttpResponse, post};
use actix_web::web::Bytes;
use crate::datadog_span::RUMSpans;
use crate::exporters::{ConsoleExporter, FileExporter, JSONExporter};
use crate::OUTPUT_DIR;

#[post("/api/v2/spans")]
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

    let mut console_exporter = ConsoleExporter {};
    console_exporter.export(pretty_json.clone());

    let mut file_exporter = FileExporter::new(format!("{}/api/v2/spans", cur_dir));
    file_exporter.export(pretty_json);

    Ok(HttpResponse::Ok().finish()) // <- send response
}
