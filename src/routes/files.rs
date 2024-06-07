use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    routing::post,
    Router,
};
use futures_util::stream::StreamExt;

use super::routes::AppState;

/// Handles the upload of files to the application.
///
/// This function is called when a client sends a multipart form request to the `/upload` endpoint.
/// It iterates through the uploaded files, prints the name and length of each file, and stores the file data.
/// The file data is stored in the application state, which can be accessed by other parts of the application.
pub async fn upload_file(State(_data): State<Arc<AppState>>, mut multipart: Multipart) {
    while let Some( field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

#[cfg(test)]
mod tests{
    
}