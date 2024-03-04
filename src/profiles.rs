use std::char::decode_utf16;
use actix_web::web::Bytes;
use actix_web::{get, post, Error, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use protobuf::Message;
use std::io::{Cursor, Read, Write};
use futures::io::AsyncReadExt;
use actix_multipart::Multipart;
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
async fn profiling(mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("-- START PROFILING --");
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // boundary
        let boundary_name = field.name();
        println!("-- BOUNDARY: {:?}", boundary_name);

        if let Some(content_type_mime) = field.content_type() {
            match (content_type_mime.type_(), content_type_mime.subtype()) {
                (mime::APPLICATION, mime::JSON) => {
                    let mut body = Vec::new();
                    while let Some(chunk) = field.next().await {
                        let chunk = chunk?;
                        body.extend_from_slice(&chunk);
                    }
                    println!("-- JSON: \n{:?}", std::str::from_utf8(&body));
                }
                (mime::APPLICATION, mime::OCTET_STREAM) => {
                    let mut body = Vec::new();
                    while let Some(chunk) = field.next().await {
                        let chunk = chunk?;
                        body.extend_from_slice(&chunk);
                    }

                    // print bytes
                    // println!("-- OCTET_STREAM: \n{:?}", body);

                    // body is gzip compressed, so we need to decompress it
                    let mut decompressed = Vec::new();
                    let mut decoder = flate2::read::GzDecoder::new(&body[..]);
                    if let Err(e) = decoder.read_to_end(&mut decompressed) {
                        println!("-- DECOMPRESSED ERROR: \n{:?}", e);
                    }

                    // parse protobuf message
                    let message: Profile = Message::parse_from_bytes(&decompressed).unwrap();
                    println!("-- PROTOBUF: \n{:?}", message);
                }
                _ => {
                    panic!("Unexpected content type: {:?}", content_type_mime);
                }
            }
        }
    }
    print!("-- END PROFILING --");

    Ok(HttpResponse::Ok().into())
}