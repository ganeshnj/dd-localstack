use actix_web::{Error, HttpRequest, HttpResponse, post};
use actix_web::web::Bytes;
use crate::datadog_span::RUMSpans;
use crate::exporters::{ConsoleExporter, FileExporter, JSONExporter};
use crate::{default_service, OUTPUT_DIR};

#[post("/api/v2/spans")]
async fn rum_spans(body: Bytes, request: HttpRequest) -> Result<HttpResponse, Error> {
    default_service(request, body).await
}
