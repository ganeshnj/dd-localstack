use std::char::decode_utf16;
use std::fs;
use std::fs::ReadDir;
use actix_web::web::Bytes;
use actix_web::{get, post, Error, HttpRequest, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use protobuf::Message;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use futures::io::AsyncReadExt;
use actix_multipart::Multipart;
use actix_web::dev::RequestHead;
use actix_web::http::header::DispositionType;
use uuid::Uuid;
use crate::exporters::{FileExporter, JSONExporter};
use crate::OUTPUT_DIR;
use crate::profile::Profile;

/*
POST /api/v2/profile HTTP/1.1
Content-Type: multipart/form-data;boundary="boundary"

--boundary
Content-Disposition: form-data; name="event"; filename="event.json"
Content-Type: application/json # The request fails if the event Content-type is missing

<Event file payload>

--boundary
Content-Disposition: form-data; name="<ATTACHMENT1>"; filename="<ATTACHMENT1.PPROF>" # or <ATTACHMENT1.JFR>
Content-Type: text/plain

<attachment1 content>

--boundary
Content-Disposition: form-data; name="<ATTACHMENT2>"; filename="<ATTACHMENT2.PPROF>"
Content-Type: text/plain

<attachment2 content>

--boundary--

--boundary
Content-Disposition: form-data; name="<ATTACHMENT3>"; filename="<ATTACHMENT3.JSON>"
Content-Type: text/plain

<attachment3 content>

--boundary--
 */

/*
Event JSON schema
This JSON describes the schema used to format the event:
{
 "start":"<START DATE>",
 "end":"<END DATE>",
 "attachments":["<ATTACHMENT1>", "<ATTACHMENT2>"],
 "tags_profiler":"<PROFILER TAGS>",
 "family":"<FAMILY>",
 "version":"4"
}
 */

#[post("/profiling/v1/input")]
async fn profiling(mut payload: Multipart, request: HttpRequest, body: Bytes) -> Result<HttpResponse, Error> {
    // print request headers
    println!("-- REQUEST HEADERS --");
    /*
host: localhost:8126
user-agent: Go-http-client/1.1
accept-encoding: gzip
content-length: 3576
content-type: multipart/form-data; boundary=379fd95c772fe1a488552211dce31c4b15812dfdc18c106f901600cf695c
     */

    let mut request_id = Uuid::new_v4().to_string();
    // find request id from headers
    for (name, value) in request.headers().iter() {
        let sanitized_name = name.as_str().to_lowercase();
        if sanitized_name == "dd-request-id" {
            request_id = value.to_str().unwrap().to_string();
        }
    }

    let dir = format!("{}/profiling/v1/input/{}", OUTPUT_DIR, request_id);
    let mut file_writer = FileWriter::new(dir);

    // write headers to file
    let mut headers = String::new();
    for (name, value) in request.headers().iter() {
        headers.push_str(&format!("{}: {}\n", name, value.to_str().unwrap()));
    }
    file_writer.export_string("headers".to_string(), headers);

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_type().unwrap().clone();
        println!("-- CONTENT TYPE: {:?}", content_type);

        // content disposition
        println!("-- CONTENT DISPOSITION \n{:?}", field.content_disposition());

        match (field.content_disposition().disposition) {
            DispositionType::FormData => {
            //     good
            }
            _ => {
                println!("-- BAD CONTENT DISPOSITION");
                continue;
            }
        }

        let name = field.content_disposition().parameters[0].clone();
        let filename = field.content_disposition().parameters[1].clone();
        println!("-- NAME: {}", name);
        println!("-- FILENAME: {}", filename);


        // Field in turn is stream of *Bytes* object
        let mut bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            bytes.extend_from_slice(&data);
        }

        if filename.as_filename() == Option::from("cpu.pprof") {
            // this is gzip compressed
            let mut cursor = Cursor::new(bytes.clone());
            let mut decompressed = Vec::new();
            let mut decoder = flate2::read::GzDecoder::new(&mut cursor);
            decoder.read_to_end(&mut decompressed).unwrap();
            let profile = Profile::parse_from_bytes(&decompressed).unwrap();
            println!("-- PROFILE: {:?}", profile.to_string());
        }

        // write to file
        file_writer.export_bytes(&filename.as_filename().unwrap(), bytes);
    }

    Ok(HttpResponse::Ok().into())
}

pub struct FileWriter {
    base_path: String,
}

impl FileWriter {
    pub(crate) fn new(base_path: String) -> FileWriter {
        if !Path::new(&base_path).exists() {
            fs::create_dir_all(&base_path).unwrap();
        }

        FileWriter {
            base_path
        }
    }

    pub(crate) fn export_string(&mut self, file_name: String, s: String) {
        let file_name = format!("{}/{}", self.base_path, file_name);
        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }

    pub(crate) fn export_bytes(&mut self, file_name: &str, bytes: Vec<u8>) {
        let file_name = format!("{}/{}", self.base_path, file_name);
        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(&bytes).unwrap();
    }
}