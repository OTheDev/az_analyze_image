use az_analyze_image::v40::client::{AnalyzeImageOptions, Client};
use az_analyze_image::v40::{DetectedPerson, VisualFeature};

use image::{load_from_memory_with_format, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use reqwest::Client as ReqwestClient;

use std::env::var;
use std::error::Error;

const IMAGE_URL: &str =
    "https://images.pexels.com/photos/1367269/pexels-photo-1367269.jpeg";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client: Client = get_client();

    // Call the Analyze Image API with the people detection feature enabled
    let people: Vec<DetectedPerson> = detect_people(&client, IMAGE_URL).await?;

    // Download the image bytes
    let img_bytes: Vec<u8> = download_image(IMAGE_URL).await?;

    // Load the image
    let mut img: RgbImage =
        load_from_memory_with_format(&img_bytes, ImageFormat::Jpeg)?.to_rgb8();

    // Draw bounding boxes around detected persons with confidence > 0.75
    draw_people_bounding_boxes(&mut img, &people);

    // Save the new image
    img.save("out.jpg")?;

    Ok(())
}

fn get_client() -> Client {
    let key = var("CV_KEY").expect("no CV_KEY");
    let endpoint = var("CV_ENDPOINT").expect("no CV_ENDPOINT");

    Client::new(key, &endpoint).expect("failed to create client")
}

async fn detect_people(
    client: &Client,
    url: &str,
) -> Result<Vec<DetectedPerson>, Box<dyn Error>> {
    let features = vec![VisualFeature::People];
    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client.analyze_image_url(url, options).await?;

    Ok(analysis.people_result.expect("no people result").values)
}

async fn download_image(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = ReqwestClient::new();
    let response = client.get(url).send().await?.bytes().await?;
    Ok(response.to_vec())
}

fn draw_people_bounding_boxes(img: &mut RgbImage, people: &[DetectedPerson]) {
    let thickness = 16;
    let colors = [Rgb([1, 255, 79]), Rgb([0, 237, 245]), Rgb([118, 232, 181])];

    for (i, person) in people.iter().enumerate() {
        if person.confidence <= 0.75 {
            continue;
        }

        let bounding_box = Rect::at(
            person.bounding_box.x as i32,
            person.bounding_box.y as i32,
        )
        .of_size(person.bounding_box.w, person.bounding_box.h);

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
