use crate::error::{AppError, AppJson};
use axum::{
    BoxError,
    body::Bytes,
    extract::{Path, Request},
    http::StatusCode,
};
use futures::{Stream, TryStreamExt};
use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

/// Healthcheck endpoint
///
/// Endpoint for k8s healthcheck functionality
#[utoipa::path(
    get,
    path = "/healthcheck",
    responses((status = OK, body = String)),
    tag = "Utils"
)]
pub async fn healthcheck() -> Result<AppJson<String>, AppError> {
    Ok(AppJson("ok".into()))
}
/// Handler that streams the request body to a file.
///
/// POST'ing to `/upload/foo.txt` will create a file called `foo.txt`.
/// For example: curl -i -X POST http://localhost:3000/api/upload/README.md --data-binary "@README.md"
#[utoipa::path(
    post,
    path = "/upload/{file_name}",
    tag = "Utils",
    params(
        ("file_name" = String, Path, description = "file name to uploaded")
    ),
    responses(
        (status = 201, description = "pdf upload successfully.", body = String),
        (status = 400, description = "pdf upload error", body = String),
    )
)]
pub async fn save_request_body(
    Path(file_name): Path<String>,
    request: Request,
) -> Result<(), (StatusCode, String)> {
    stream_to_file(&file_name, request.into_body().into_data_stream()).await
}

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let upload = "uploads".to_string();
        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(&upload).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
