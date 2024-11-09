# Detect Faces

> [!NOTE]
> API Version 3.2

## About

In this example, the Analyze Image API is used to detect faces in an image.

For each detected face, the API returns a **bounding box** (face rectangle),
which defines a rectangle in the image.

This example draws the bounding boxes on the image for all detected faces and
saves it to `out.jpg`:

<p align="center">
  <img src="https://github.com/OTheDev/az_analyze_image/blob/detect_faces/examples/detect_faces/out.jpg?raw=true" />
</p>

## Image Source

https://images.pexels.com/photos/1367269/pexels-photo-1367269.jpeg
