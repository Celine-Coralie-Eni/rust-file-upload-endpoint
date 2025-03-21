use axum::{extract::Multipart, response::{Html, Json}, routing::{get, post}, Router};
use std::{fs::File, io::Write};
use tokio::fs;
use serde::Serialize;

#[derive(Serialize)]
struct UploadResponse {
    message: String,
    files: Vec<String>,
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("public/first.html"))
}

async fn upload(mut multipart: Multipart) -> Json<UploadResponse> {
    let mut uploaded_files = Vec::new(); // To store the names of successfully uploaded files

    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        // Grab the name of the file
        let file_name = field.file_name().unwrap().to_string();

        // Create a path for the file
        let file_path = format!("files/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.expect("Failed to read bytes!");

        // Ensure the file's parent directory exists
        let file_dir = "files"; // The directory where files will be saved
        fs::create_dir_all(file_dir).await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(&file_path).expect("Failed to open file handle!");

        // Write the incoming data to the file
        file_handle.write_all(&data).expect("Failed to write data!");

        // Add the file name to the list of uploaded files
        uploaded_files.push(file_name.to_string());

        println!("Got file: {}", file_name);
    }

    // Return an aggregated result with all uploaded files
    Json(UploadResponse {
        message: "Files uploaded successfully".to_string(),
        files: uploaded_files,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))  // Serve the index HTML page
        .route("/upload", post(upload));  // Handle the file upload

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030")
        .await
        .expect("Failed to start listener!");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}
