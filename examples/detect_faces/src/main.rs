/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use az_analyze_image::v32::client::{AnalyzeImageOptions, Client};
use az_analyze_image::v32::{FaceDescription, VisualFeatureTypes};
use image::{load_from_memory_with_format, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use reqwest::Client as ReqwestClient;
use std::env::var;
use std::error::Error;

const IMAGE_URL: &str =
    "https://images.pexels.com/photos/1367269/pexels-photo-1367269.jpeg";

// Pixel thickness for rectangle boundaries.
const THICKNESS: i32 = 16;

// Colors for each rectangle boundary. If there's more faces than COLORS.len(),
// then we wrap around.
const COLORS: &[Rgb<u8>] =
    &[Rgb([1, 255, 79]), Rgb([0, 237, 245]), Rgb([118, 232, 181])];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client: Client = get_client();

    // Call the Analyze Image API with the face detection feature enabled
    let faces: Vec<FaceDescription> = detect_faces(&client, IMAGE_URL).await?;

    // Download the image bytes
    let img_bytes: Vec<u8> = download_image(IMAGE_URL).await?;

    // Load the image
    let mut img: RgbImage =
        load_from_memory_with_format(&img_bytes, ImageFormat::Jpeg)?.to_rgb8();

    // Draw bounding boxes around all detected faces
    draw_face_bounding_boxes(&mut img, &faces, COLORS, THICKNESS);

    // Save the new image
    img.save("out.jpg")?;

    Ok(())
}

fn get_client() -> Client {
    let key = var("CV_KEY").expect("no CV_KEY");
    let endpoint = var("CV_ENDPOINT").expect("no CV_ENDPOINT");

    Client::new(key, &endpoint).expect("failed to create client")
}

async fn detect_faces(
    client: &Client,
    url: &str,
) -> Result<Vec<FaceDescription>, Box<dyn Error>> {
    let features = vec![VisualFeatureTypes::Faces];
    let options = AnalyzeImageOptions {
        visual_features: Some(&features),
        ..Default::default()
    };
    let analysis = client.analyze_image_url(url, options).await?;

    Ok(analysis.faces.unwrap())
}

async fn download_image(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = ReqwestClient::new();
    let response = client.get(url).send().await?.bytes().await?;

    Ok(response.to_vec())
}

fn draw_face_bounding_boxes(
    img: &mut RgbImage,
    faces: &[FaceDescription],
    colors: &[Rgb<u8>],
    thickness: i32,
) {
    for (i, face) in faces.iter().enumerate() {
        let bounding_box = Rect::at(
            face.face_rectangle.left as i32,
            face.face_rectangle.top as i32,
        )
        .of_size(face.face_rectangle.width, face.face_rectangle.height);
        let color = colors[i % colors.len()];

        draw_thick_rect(img, bounding_box, color, thickness);
    }
}

fn draw_thick_rect(
    img: &mut RgbImage,
    rect: Rect,
    color: Rgb<u8>,
    thickness: i32,
) {
    for i in 0..thickness {
        let new_rect = Rect::at(rect.left() - i, rect.top() - i)
            .of_size(rect.width() + i as u32 * 2, rect.height() + i as u32 * 2);

        draw_hollow_rect_mut(img, new_rect, color);
    }
}
