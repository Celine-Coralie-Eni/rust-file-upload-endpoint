use axum::{extract::Multipart, response::Html, routing::get, Router};
use std::{fs::File, io::Write};

async fn index() -> Html<&'static str> {
    Html(std::include_str!("public/first.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        // Grab the name
        let file_name = field.file_name().unwrap();

        // Create a path for the soon-to-be file
        let file_path = format!("files/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");
        println!("Got file!");
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030")
        .await
        .expect("Failed to start listener!");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}