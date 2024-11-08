# Detect People

> [!NOTE]
> API Version 4.0 (`2023-04-01-preview`)

## About

In this example, the Analyze Image API is used to detect people in an image.

For each detected person, the API returns a **bounding box** (defines a
rectangle in the image) and a **confidence level**.

This example draws the bounding boxes on the image for detected persons with
confidence greater than 0.75 and saves it to `out.jpg`:

<p align="center">
  <img src="https://github.com/OTheDev/az_analyze_image/blob/main/examples/detect_people/out.jpg?raw=true" />
</p>

## Image Source

https://images.pexels.com/photos/1367269/pexels-photo-1367269.jpeg
