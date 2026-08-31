use actix_web::{HttpRequest, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;

// actix_files handles Range, 206, 416 and If-Range for you,
// driven by the file's modtime and an optional ETag.
async fn download(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let path: PathBuf = "big.zip".parse().unwrap();
    Ok(NamedFile::open(path)?.into_response(&req))
}
