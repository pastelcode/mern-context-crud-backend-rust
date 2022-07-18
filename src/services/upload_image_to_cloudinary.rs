use chrono::Utc;
use reqwest::{
    multipart::{Form, Part},
    Body, Client,
};
use rocket::tokio::fs::File;
use sha256::digest;
use std::env;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::models::{cloudinary_response::CloudinaryResponse, post_model::Image};

pub async fn upload_image_to_cloudinary(image_path: &String, public_id: String) -> Image {
    // <- Get Cloudinary api key and secret
    let cloudinary_api_secret = match env::var("CLOUDINARY_API_SECRET") {
        Ok(value) => value,
        Err(_) => format!(""),
    };
    let cloudinary_api_key = match env::var("CLOUDINARY_API_KEY") {
        Ok(value) => value,
        Err(_) => format!(""),
    };
    let cloudinary_cloud = match env::var("CLOUDINARY_CLOUD") {
        Ok(value) => value,
        Err(_) => format!(""),
    };
    // ->

    // <- Parameters creation
    let timestamp = Utc::now().timestamp().to_string();
    let signature = digest(format!(
        "public_id={}&timestamp={}{}",
        public_id, timestamp, cloudinary_api_secret
    ));
    let open_file = File::open(&image_path).await.unwrap();
    let url_to_post_image = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        cloudinary_cloud
    );
    // ->

    // Read file body stream
    let image_body = Body::wrap_stream(FramedRead::new(open_file, BytesCodec::new()));
    // Make form part of file
    let image_to_post = Part::stream(image_body).file_name(public_id.to_owned());

    // <- Create the multipart form
    let form = Form::new()
        .text("api_key", cloudinary_api_key)
        .text("public_id", public_id)
        .text("timestamp", timestamp)
        .text("signature", signature)
        .part("file", image_to_post);
    // ->
    // Post the request
    let response = Client::new()
        .post(url_to_post_image)
        .multipart(form)
        .send()
        .await
        .unwrap();
    // ->
    let cloudinary_response = response.json::<CloudinaryResponse>().await.unwrap();

    let image = Image {
        url: cloudinary_response.secure_url,
        public_id: cloudinary_response.public_id,
    };

    image
}
