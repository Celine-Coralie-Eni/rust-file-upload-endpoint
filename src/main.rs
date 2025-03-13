use axum::{
    body::Bytes,
    extract::Multipart,
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{fs::File, io::Write, path::PathBuf};
use axum_server::Server;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/upload", post(upload_file));

    // Run our app on localhost:3000
    println!("Listening on http://localhost:3000");
    Server::bind("0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    // Iterate over the parts of the multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("file").to_string();
        let filepath = PathBuf::from(format!("./uploads/{}", filename));

        // Create the uploads directory if it doesn't exist
        if !filepath.parent().unwrap().exists() {
            fs::create_dir_all(filepath.parent().unwrap()).await.unwrap();
        }

        // Write the file to disk
        let mut file = File::create(filepath).unwrap();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).unwrap();
    }

    // Respond with a success message
    "File uploaded successfully".into_response()
}
