use actix_web::{web, App, HttpResponse, HttpServer, Result};
use csv::{ReaderBuilder, StringRecordsIntoIter};
use serde::Deserialize;

//[derive(Deserialize)]
struct CsvPreviewRequest {
    file: String,
}

async fn preview_csv(req: web::Json<CsvPreviewRequest>) -> Result<HttpResponse> {
    let file = req.file.as_str();

    let csv_str = match web::block(move || std::fs::read_to_string(file)) {
        Ok(csv_str) => csv_str,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let mut reader = ReaderBuilder::new().from_reader(csv_str.as_bytes());
    let headers = reader.headers().unwrap().iter().map(String::from).collect::<Vec<_>>();
    let records = reader.records().map(Result::unwrap).collect::<StringRecordsIntoIter>();

    let preview = PreviewResponse { headers, records };

    Ok(HttpResponse::Ok().json(preview))
}

struct PreviewResponse {
    headers: Vec<String>,
    records: Vec<Vec<String>>,
}

async fn upload_csv(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut data = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        data.extend_from_slice(&chunk);
    }

    let csv_str = match String::from_utf8(data.to_vec()) {
        Ok(csv_str) => csv_str,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let mut reader = ReaderBuilder::new().from_reader(csv_str.as_bytes());
    let headers = reader.headers().unwrap().iter().map(String::from).collect::<Vec<_>>();
    let records = reader.records().map(Result::unwrap).collect::<StringRecordsIntoIter>();

    let preview = PreviewResponse { headers, records };

    Ok(HttpResponse::Ok().json(preview))
}

//[actix_web::main]
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/preview_csv", web::post().to(preview_csv))
            .route("/upload_csv", web::post().to(upload_csv))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
