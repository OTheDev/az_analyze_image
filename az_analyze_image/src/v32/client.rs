/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

//! Client module for the Azure AI Services [Analyze Image API Version 3.2](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2&tabs=HTTP)
//!
//! Image constraints imposed by the API:
//! - Must be JPEG, PNG, GIF, or BMP format.
//! - Must be less than 4 MiB (4,194,304 bytes).
//! - Dimensions must be greater than 50 x 50 pixels and less than
//!     16,000 x 16,000 pixels.

use crate::common::secret::Secret;
use crate::v32::*;
use serde::Serialize;

/// Maximum input image size allowed by the API.
pub const MAX_IMAGE_SIZE: usize = 4 * 1024 * 1024; // 4194304 bytes

pub type Result<T> = std::result::Result<T, Error>;

/// Represents the various errors that can occur while using the [`Client`].
///
/// When the Analyze Image API (v3.2) encounters an error, the API returns a
/// [`ComputerVisionErrorResponse`] object, which can be retrieved through the
/// [`Error::API`] variant.
///
/// Other variants cover errors originating from client logic, client
/// validation, or [`reqwest::Error`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents errors returned by the Analyze Image API.
    #[error("API error response: {0}")]
    API(#[from] ComputerVisionErrorResponse),

    /// Wrapper around [`reqwest::Error`].
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Wrapper around [`ValidationError`].
    ///
    /// Represents errors identified through client validation, before any
    /// interaction with external servers.
    #[error(transparent)]
    Validation(#[from] ValidationError),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Wrapper around [`reqwest::header::InvalidHeaderValue`].
    ///
    /// This error occurs specifically in [`Client::new`] when the provided key
    /// contains invalid characters for an HTTP header, implying that the key is
    /// invalid for use with the Azure API.
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

/// Image analysis parameters.
///
/// Note that in the case that no features are specified, by default, the API
/// returns a response as if [`VisualFeatureTypes::Categories`] were specified
/// in the request.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AnalyzeImageOptions<'a> {
    /// Turn off specified domain models when generating the description.
    pub description_exclude: Option<&'a [DescriptionExclude]>,

    /// A string indicating which domain-specific details to return. Multiple
    /// values should be comma-separated. Valid visual feature types include:
    ///
    /// - `Celebrities`: Identifies celebrities if detected in the image,
    /// - `Landmarks`: Identifies notable landmarks in the image.
    pub details: Option<&'a [Details]>,

    /// The desired language for output generation. If this parameter is not
    /// specified, the default value is "en". See <https://aka.ms/cv-languages>
    /// for list of supported languages.
    pub language: Option<&'a str>,

    /// Optional parameter to specify the version of the AI model. Accepted
    /// values are: "latest", "2021-04-01", "2021-05-01". Defaults to "latest".
    ///
    /// Regex pattern: `^(latest|\d{4}-\d{2}-\d{2})(-preview)?$`
    pub model_version: Option<&'a str>,

    /// A string indicating what visual feature types to return. Multiple values
    /// should be comma-separated. Valid visual feature types include:
    ///
    /// - `Categories`: Categorizes image content according to a taxonomy
    ///   defined in documentation.
    /// - `Tags`: Tags the image with a detailed list of words related to the
    ///   image content.
    /// - `Description`: Describes the image content with a complete English
    ///   sentence.
    /// - `Faces`: Detects if faces are present. If present, generate
    ///   coordinates, gender and age.
    /// - `ImageType`: Detects if image is clipart or a line drawing.
    /// - `Color`: Determines the accent color, dominant color, and whether an
    ///   image is black & white.
    /// - `Adult:` Detects if the image is pornographic in nature (depicts
    ///   nudity or a sex act), or is gory (depicts extreme violence or blood).
    ///   Sexually suggestive content (aka racy content) is also detected.
    /// - `Objects`: Detects various objects within an image, including the
    ///   approximate location. The `Objects` argument is only available in
    ///   English.
    /// - `Brands`: Detects various brands within an image, including the
    ///   approximate location. The `Brands` argument is only available in
    ///   English.
    pub visual_features: Option<&'a [VisualFeatureTypes]>,
}

/// Client for the [Analyze Image API v3.2](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2).
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    url: String,
}

impl Client {
    /// Create a new `Client`.
    ///
    /// # Parameters
    /// - `key`: Azure AI Services key.
    /// - `endpoint`: Azure AI Services Computer Vision endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use az_analyze_image::v32::client::Client;
    /// use std::env;
    ///
    /// let key = env::var("CV_KEY").expect("No CV_KEY");
    /// let endpoint = env::var("CV_ENDPOINT").expect("No CV_ENDPOINT");
    ///
    /// let client = Client::new(key, &endpoint).unwrap();
    /// ```
    pub fn new(key: String, endpoint: &str) -> Result<Self> {
        let secret = Secret::new(key);

        Ok(Client {
            client: Self::create_http_client(secret)?,
            url: format!("{}vision/v3.2/analyze", endpoint),
        })
    }

    /// Analyze the input image.
    ///
    /// # Parameters
    /// - `image_url`: Publicly reachable URL of an image.
    /// - `options`: Optional parameters to be passed to the Analyze Image API.
    ///
    /// # Example
    ///
    /// ```
    /// use az_analyze_image::v32::client::{AnalyzeImageOptions, Client};
    /// use az_analyze_image::v32::{
    ///     FaceDescription, ImageAnalysis, VisualFeatureTypes,
    /// };
    /// use std::env;
    ///
    /// const IMAGE_URL: &str =
    ///     "https://upload.wikimedia.org/wikipedia/commons/2/2a/Human_faces.jpg";
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let key = env::var("CV_KEY").expect("No CV_KEY");
    ///     let endpoint = env::var("CV_ENDPOINT").expect("No CV_ENDPOINT");
    ///
    ///     let client = Client::new(key, &endpoint).unwrap();
    ///
    ///     let visual_features = vec![VisualFeatureTypes::Faces];
    ///
    ///     let options = AnalyzeImageOptions {
    ///         visual_features: Some(&visual_features),
    ///         ..Default::default()
    ///     };
    ///
    ///     let analysis: ImageAnalysis =
    ///         client.analyze_image_url(IMAGE_URL, options).await.unwrap();
    ///
    ///     let faces = analysis.faces.expect("no faces");
    ///
    ///     print_faces(&faces);
    /// }
    ///
    /// fn print_faces(faces: &[FaceDescription]) {
    ///     for face_description in faces {
    ///         let face_rectangle = &face_description.face_rectangle;
    ///
    ///         println!(
    ///             "Detected face at rectangle\n\
    ///              (x, y, w, h) = ({}, {}, {}, {})\n",
    ///             face_rectangle.left,
    ///             face_rectangle.top,
    ///             face_rectangle.width,
    ///             face_rectangle.height,
    ///         );
    ///     }
    /// }
    /// ```
    pub async fn analyze_image_url(
        &self,
        image_url: &str,
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysis> {
        self.analyze_image_(ImageInput::Url(image_url), options)
            .await
    }

    /// Analyze the input image.
    ///
    /// # Parameters
    /// - `image_data`: Image bytes.
    /// - `options`: Optional parameters to be passed to the Analyze Image API.
    ///
    /// # Example
    ///
    /// ```
    /// use az_analyze_image::v32::client::{AnalyzeImageOptions, Client};
    /// use az_analyze_image::v32::{
    ///     FaceDescription, ImageAnalysis, VisualFeatureTypes,
    /// };
    /// use std::env;
    ///
    /// const IMAGE_PATH: &str = "./tests/images/people.jpg";
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let key = env::var("CV_KEY").expect("No CV_KEY");
    ///     let endpoint = env::var("CV_ENDPOINT").expect("No CV_ENDPOINT");
    ///
    ///     let client = Client::new(key, &endpoint).unwrap();
    ///
    ///     let image_bytes: Vec<u8> = std::fs::read(IMAGE_PATH).unwrap();
    ///
    ///     let visual_features = vec![VisualFeatureTypes::Faces];
    ///
    ///     let options = AnalyzeImageOptions {
    ///         visual_features: Some(&visual_features),
    ///         ..Default::default()
    ///     };
    ///
    ///     let analysis: ImageAnalysis =
    ///         client.analyze_image(&image_bytes, options).await.unwrap();
    ///
    ///     let faces = analysis.faces.expect("no faces");
    ///
    ///     print_faces(&faces);
    ///
    ///     assert!(faces.len() == 2);
    /// }
    ///
    /// fn print_faces(faces: &[FaceDescription]) {
    ///     for face_description in faces {
    ///         let face_rectangle = &face_description.face_rectangle;
    ///
    ///         println!(
    ///             "Detected face at rectangle\n\
    ///              (x, y, w, h) = ({}, {}, {}, {})\n",
    ///             face_rectangle.left,
    ///             face_rectangle.top,
    ///             face_rectangle.width,
    ///             face_rectangle.height,
    ///         );
    ///     }
    /// }
    /// ```
    pub async fn analyze_image(
        &self,
        image_data: &[u8],
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysis> {
        // TODO?: overload=stream should be a query parameter.
        self.analyze_image_(ImageInput::Data(image_data), options)
            .await
    }

    async fn analyze_image_(
        &self,
        input: ImageInput<'_>,
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysis> {
        let query_params = Self::build_query_params(&options);

        let request = self.client.post(&self.url).query(&query_params);

        let response = match input {
            ImageInput::Url(image_url) => {
                let image_url = ImageUrl {
                    url: image_url.to_string(),
                };
                request.json(&image_url).send().await?
            }
            ImageInput::Data(image_data) => {
                request.body(image_data.to_vec()).send().await?
            }
        };

        Self::handle_response(response).await
    }

    async fn handle_response(
        response: reqwest::Response,
    ) -> Result<ImageAnalysis> {
        if response.status().is_success() {
            let analysis: ImageAnalysis = response.json().await?;
            return Ok(analysis);
        }
        let err = response.json::<ComputerVisionErrorResponse>().await?;
        Err(Error::API(err))
    }

    // POST {Endpoint}/vision/v3.2/analyze?visualFeatures={visualFeatures}&details={details}&language={language}&descriptionExclude={descriptionExclude}&model-version={model-version}
    fn build_query_params<'a>(
        options: &AnalyzeImageOptions,
    ) -> Vec<(&'a str, String)> {
        let mut query_params: Vec<(&str, String)> = Vec::new();

        if let Some(visual_features) = options.visual_features {
            if !visual_features.is_empty() {
                query_params.push((
                    "visualFeatures",
                    visual_features
                        .iter()
                        .map(|f| format!("{:?}", f))
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }

        if let Some(details) = options.details {
            if !details.is_empty() {
                query_params.push((
                    "details",
                    details
                        .iter()
                        .map(|detail| format!("{:?}", detail))
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }

        if let Some(lang) = options.language {
            if !lang.is_empty() {
                query_params.push(("language", lang.to_string()));
            }
        }

        if let Some(exclude) = options.description_exclude {
            if !exclude.is_empty() {
                query_params.push((
                    "descriptionExclude",
                    exclude
                        .iter()
                        .map(|ex| format!("{:?}", ex))
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }

        if let Some(version) = options.model_version {
            if !version.is_empty() {
                query_params.push(("model-version", version.to_string()));
            }
        }

        query_params
    }

    fn create_http_client(key: Secret) -> Result<reqwest::Client> {
        let headers = Self::create_headers(&key)?;

        Ok(reqwest::Client::builder()
            .default_headers(headers)
            .build()?)
    }

    fn create_headers(
        key: &Secret,
    ) -> std::result::Result<reqwest::header::HeaderMap, ValidationError> {
        let mut headers = reqwest::header::HeaderMap::new();

        let mut header_key =
            reqwest::header::HeaderValue::from_str(key.value())?;
        header_key.set_sensitive(true);

        headers.insert("Ocp-Apim-Subscription-Key", header_key);
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static(
                "application/octet-stream",
            ),
        );

        Ok(headers)
    }
}

enum ImageInput<'a> {
    Url(&'a str),
    Data(&'a [u8]),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header_value_to_str(value: &reqwest::header::HeaderValue) -> &str {
        value.to_str().unwrap_or("")
    }

    #[test]
    fn test_create_headers() {
        let key = Secret::new("dummy_key".into());
        let headers = Client::create_headers(&key).unwrap();

        assert_eq!(
            header_value_to_str(
                headers.get("Ocp-Apim-Subscription-Key").unwrap()
            ),
            key.value()
        );

        assert_eq!(
            header_value_to_str(
                headers.get(reqwest::header::CONTENT_TYPE).unwrap()
            ),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_create_headers_returns_error_invalid_header_value() {
        let result = Client::create_headers(&Secret::new("dummy\nkey".into()));

        assert!(matches!(
            result,
            Err(ValidationError::InvalidHeaderValue(_))
        ))
    }

    #[test]
    fn test_create_http_client() {
        let result =
            Client::create_http_client(Secret::new("dummy_key".into()));

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_params_edge_case() {
        let options = AnalyzeImageOptions {
            visual_features: Some(&[]),
            details: Some(&[]),
            language: Some(""),
            model_version: Some(""),
            description_exclude: Some(&[]),
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params.len(), 0);
    }

    #[test]
    fn test_error_invalid_header_value() {
        let invalid_key = "mock\n_invalid_key";
        let endpoint = "mock_endpoint";

        let result = Client::new(invalid_key.into(), endpoint);

        match result {
            Ok(_) => panic!("Expected Error::InvalidHeaderValue, but got Ok"),
            Err(err) => match err {
                Error::Validation(ValidationError::InvalidHeaderValue(_)) => { /* Test passed */
                }
                other => panic!(
                    "Expected Error::InvalidHeaderValue, but got {:?}",
                    other
                ),
            },
        }
    }

    #[test]
    fn test_build_query_params_no_options() {
        let options = AnalyzeImageOptions::default();

        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params.len(), 0);
    }

    #[test]
    fn test_build_query_params_with_visual_features() {
        let visual_features = vec![VisualFeatureTypes::Description];
        let options = AnalyzeImageOptions {
            visual_features: Some(&visual_features),
            ..Default::default()
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(
            query_params,
            vec![("visualFeatures", "Description".to_string())]
        );
    }

    #[test]
    fn test_build_query_params_with_details() {
        let details = vec![Details::Landmarks];
        let options = AnalyzeImageOptions {
            details: Some(&details),
            ..Default::default()
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params, vec![("details", "Landmarks".to_string())]);
    }

    #[test]
    fn test_build_query_params_with_language() {
        let options = AnalyzeImageOptions {
            language: Some("es"),
            ..Default::default()
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params, vec![("language", "es".to_string())]);
    }

    #[test]
    fn test_build_query_params_with_model_version() {
        let options = AnalyzeImageOptions {
            model_version: Some("latest"),
            ..Default::default()
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params, vec![("model-version", "latest".to_string())]);
    }

    #[test]
    fn test_build_query_params_with_description_exclude() {
        let description_exclude = vec![DescriptionExclude::Celebrities];
        let options = AnalyzeImageOptions {
            description_exclude: Some(&description_exclude),
            ..Default::default()
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(
            query_params,
            vec![("descriptionExclude", "Celebrities".to_string())]
        );
    }

    #[test]
    fn test_build_query_params_all_options() {
        let visual_features =
            vec![VisualFeatureTypes::Faces, VisualFeatureTypes::Tags];
        let details = vec![Details::Celebrities];
        let description_exclude = vec![DescriptionExclude::Landmarks];

        let options = AnalyzeImageOptions {
            visual_features: Some(&visual_features),
            details: Some(&details),
            language: Some("fr"),
            model_version: Some("2021-04-01"),
            description_exclude: Some(&description_exclude),
        };

        let query_params = Client::build_query_params(&options);

        assert!(query_params
            .contains(&("visualFeatures", "Faces,Tags".to_string())));
        assert!(query_params.contains(&("details", "Celebrities".to_string())));
        assert!(query_params.contains(&("language", "fr".to_string())));
        assert!(
            query_params.contains(&("model-version", "2021-04-01".to_string()))
        );
        assert!(query_params
            .contains(&("descriptionExclude", "Landmarks".to_string())));
    }
}
