[<img alt="github" src="https://img.shields.io/badge/github-othedev/analyze_image-76e8b5?style=for-the-badge&labelColor=24292e&logo=github" height="20">](https://github.com/OTheDev/az_analyze_image)
[![Multi-Platform Test](https://github.com/OTheDev/az_analyze_image/actions/workflows/test_with_key.yml/badge.svg?branch=main)](https://github.com/OTheDev/az_analyze_image/actions/workflows/test_with_key.yml)
[![Static Analysis](https://github.com/OTheDev/az_analyze_image/actions/workflows/static.yml/badge.svg?branch=main)](https://github.com/OTheDev/az_analyze_image/actions/workflows/static.yml)

# az_analyze_image

Rust client library for Azure AI Services Analyze Image (Image Analysis) REST
APIs, versions
[3.2](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2&tabs=HTTP)
and
[4.0](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP) (`2023-04-01-preview`).

## See also

- [What is Image Analysis?](https://learn.microsoft.com/en-us/azure/ai-services/computer-vision/overview-image-analysis?tabs=3-2)
- Analyze Image
    - [API Version 3.2](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2&tabs=HTTP)
    - [API Version 4.0](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP) (`2023-04-01-preview`)

## License

This project is dual-licensed under either the [Apache License, Version 2.0](https://github.com/OTheDev/az_analyze_image/blob/main/LICENSE-APACHE)
or the [MIT License](https://github.com/OTheDev/az_analyze_image/blob/main/LICENSE-MIT),
at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you shall be dual-licensed as above, without
any additional terms or conditions.

## Example - Detect People in an Image

In the [detect people](https://github.com/OTheDev/az_analyze_image/tree/main/examples/detect_people)
example, the Analyze Image API is used to detect people in an image.

For each detected person, the API returns a bounding box (defines a rectangle in
the image) and a confidence level.

This example draws the bounding boxes on the image for detected persons with
confidence greater than 0.75 and saves it to `out.jpg`:

<p align="center">
  <img src="https://github.com/OTheDev/az_analyze_image/blob/main/examples/detect_people/out.jpg?raw=true" />
</p>

```rust
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

// Pixel thickness for rectangle boundaries.
const THICKNESS: u32 = 8;

// Colors for each rectangle boundary. If there's more people than COLORS.len(),
// then we wrap around.
const COLORS: &[Rgb<u8>] =
    &[Rgb([1, 255, 79]), Rgb([0, 237, 245]), Rgb([118, 232, 181])];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client: Client = get_client();

    // Call the Analyze Image API with the people detection feature enabled
    let people: Vec<DetectedPerson> = detect_people(&client, IMAGE_URL).await?;

    // Download and Load the image
    let img_bytes: Vec<u8> = download_image(IMAGE_URL).await?;
    let mut img: RgbImage =
        load_from_memory_with_format(&img_bytes, ImageFormat::Jpeg)?.to_rgb8();

    // Draw bounding boxes around detected persons with confidence > 0.75
    draw_bounding_boxes(&mut img, &people, COLORS, THICKNESS);

    img.save("out.jpg")?;

    Ok(())
}

fn get_client() -> Client {
    let key = var("CV_KEY").expect("no CV_KEY");
    let endpoint = var("CV_ENDPOINT").expect("no CV_ENDPOINT");

    Client::new(key, &endpoint).unwrap()
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

    Ok(analysis.people_result.unwrap().values)
}

async fn download_image(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = ReqwestClient::new();
    let response = client.get(url).send().await?.bytes().await?;

    Ok(response.to_vec())
}

fn draw_bounding_boxes(
    img: &mut RgbImage,
    people: &[DetectedPerson],
    colors: &[Rgb<u8>],
    thickness: u32,
) {
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
    thickness: u32,
) {
    for i in 0..thickness {
        let new_rect = Rect::at(rect.left() - i as i32, rect.top() - i as i32)
            .of_size(rect.width() + i * 2, rect.height() + i * 2);

        draw_hollow_rect_mut(img, new_rect, color);
    }
}
```
