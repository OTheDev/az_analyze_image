/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

//! Client module for the Azure AI Services [Analyze Image API Version 4.0](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP)
//! (`2023-04-01-preview`).
//!
//! Image constraints imposed by the API:
//! - Must be JPEG, PNG, GIF, BMP, WEBP, ICO, TIFF, or MPO format.
//! - Must be less than 20 MiB (20,971,520 bytes).
//! - Must have dimensions greater than 50 x 50 pixels and less than 16,000 x
//!     16,000 pixels.

use crate::common::secret::Secret;
use crate::v40::*;
use serde::Serialize;

const DEFAULT_API_VERSION: &str = "2023-04-01-preview";

/// Maximum input image size allowed by the API.
pub const MAX_IMAGE_SIZE: usize = 20 * 1024 * 1024; // 20971520 bytes

pub type Result<T> = std::result::Result<T, Error>;

/// Represents the various errors that can occur while using the [`Client`].
///
/// When the Analyze Image API (v4.0) encounters an error, the API returns an
/// [`ErrorResponse`] object, which can be retrieved through the [`Error::API`]
/// variant.
///
/// Other variants cover errors originating from client logic, client
/// validation, or [`reqwest::Error`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents errors returned by the Analyze Image API.
    #[error("API error response: {0}")]
    API(#[from] ErrorResponse),

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

    /// The Analyze Image API v4.0 expects that either [`features`] or
    /// [`model_name`] are specified. The client validates this requirement
    /// before sending a request for an Analyze Image operation.
    ///
    /// [`features`]: self::AnalyzeImageOptions#structfield.features
    /// [`model_name`]: self::AnalyzeImageOptions#structfield.model_name
    #[error("Either `features` or `model_name` must be specified.")]
    NoFeaturesOrModelName,
}

/// Image analysis parameters.
///
/// Note that the Analyze Image API requires that either `features` or
/// `model_name` is specified. The client will return [`NoFeaturesOrModelName`]
/// in analyze image operations in the event that none of these parameters are
/// provided.
///
/// [`NoFeaturesOrModelName`]: self::ValidationError#variant.NoFeaturesOrModelName
// URI Parameters. Only api_version is mandatory.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AnalyzeImageOptions<'a> {
    /// Requested API version.
    // api_version: &'a str, // "api-version"

    /// The visual features requested: `tags`, `objects`, `caption`,
    /// `denseCaptions`, `read`, `smartCrops`, `people`. This parameter needs to
    /// be specified if the parameter `model-name` is not specified.
    pub features: Option<&'a [VisualFeature]>,

    /// Boolean flag for enabling gender-neutral captioning for `caption` and
    /// `denseCaptions` features. If this parameter is not specified, the
    /// default value is `false`.
    pub gender_neutral_caption: Option<bool>, // "gender-neutral-caption"

    /// The desired language for output generation. If this parameter is not
    /// specified, the default value is `"en"`. See
    /// <https://aka.ms/cv-languages> for a list of supported languages.
    pub language: Option<&'a str>,

    /// The name of the custom trained model. This parameter needs to be
    /// specified if the parameter `features` is not specified.
    pub model_name: Option<&'a str>, // "model-name"

    /// A list of aspect ratios to use for `smartCrops` feature. Aspect ratios
    /// are calculated by dividing the target crop width by the height.
    /// Supported values are between 0.75 and 1.8 (inclusive). Multiple values
    /// should be comma-separated. If this parameter is not specified, the
    /// service will return one crop suggestion with an aspect ratio it sees fit
    /// between 0.5 and 2.0 (inclusive).
    pub smartcrops_aspect_ratios: Option<&'a str>, // "smartcrops-aspect-ratios"
}

/// Client for the [Analyze Image API v4.0](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP) (`2023-04-01-preview`).
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
    /// use az_analyze_image::v40::client::Client;
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
            url: format!("{}computervision/imageanalysis:analyze", endpoint),
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
    /// use az_analyze_image::v40::{
    ///     client::{AnalyzeImageOptions, Client},
    ///     DetectedPerson, ImageAnalysisResult, VisualFeature,
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
    ///     let features = vec![VisualFeature::People];
    ///
    ///     let options = AnalyzeImageOptions {
    ///         features: Some(&features),
    ///         ..Default::default()
    ///     };
    ///
    ///     let analysis: ImageAnalysisResult =
    ///         client.analyze_image_url(IMAGE_URL, options).await.unwrap();
    ///
    ///     let people_result = analysis.people_result.expect("no people result");
    ///
    ///     print_people_result_values(&people_result.values);
    /// }
    ///
    /// fn print_people_result_values(detected_persons: &[DetectedPerson]) {
    ///     for dp in detected_persons {
    ///         println!(
    ///             "Detected person with confidence: {:.4} at rectangle\n\
    ///              (x, y, w, h) = ({}, {}, {}, {})\n",
    ///             dp.confidence,
    ///             dp.bounding_box.x,
    ///             dp.bounding_box.y,
    ///             dp.bounding_box.w,
    ///             dp.bounding_box.h,
    ///         );
    ///     }
    /// }
    /// ```
    pub async fn analyze_image_url(
        &self,
        image_url: &str,
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysisResult> {
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
    /// use az_analyze_image::v40::{
    ///     client::{AnalyzeImageOptions, Client},
    ///     DetectedPerson, VisualFeature,
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
    ///     let features = vec![VisualFeature::People];
    ///
    ///     let options = AnalyzeImageOptions {
    ///         features: Some(&features),
    ///         ..Default::default()
    ///     };
    ///
    ///     let analysis =
    ///         client.analyze_image(&image_bytes, options).await.unwrap();
    ///
    ///     let people_result = analysis.people_result.expect("no people result");
    ///
    ///     print_people_result_values(&people_result.values);
    ///
    ///     assert!(people_result.values.len() >= 2);
    /// }
    ///
    /// fn print_people_result_values(detected_persons: &[DetectedPerson]) {
    ///     for dp in detected_persons {
    ///         println!(
    ///             "Detected person with confidence: {:.4} at rectangle\n\
    ///              (x, y, w, h) = ({}, {}, {}, {})\n",
    ///             dp.confidence,
    ///             dp.bounding_box.x,
    ///             dp.bounding_box.y,
    ///             dp.bounding_box.w,
    ///             dp.bounding_box.h,
    ///         );
    ///     }
    /// }
    /// ```
    pub async fn analyze_image(
        &self,
        image_data: &[u8],
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysisResult> {
        // TODO?: overload=stream should be a query parameter.
        self.analyze_image_(ImageInput::Data(image_data), options)
            .await
    }

    async fn analyze_image_(
        &self,
        input: ImageInput<'_>,
        options: AnalyzeImageOptions<'_>,
    ) -> Result<ImageAnalysisResult> {
        self.validate_parameters(&options)?;

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

    fn validate_parameters(
        &self,
        options: &AnalyzeImageOptions,
    ) -> std::result::Result<(), ValidationError> {
        if options.features.is_none() && options.model_name.is_none() {
            return Err(ValidationError::NoFeaturesOrModelName);
        }
        Ok(())
    }

    async fn handle_response(
        response: reqwest::Response,
    ) -> Result<ImageAnalysisResult> {
        if response.status().is_success() {
            let analysis: ImageAnalysisResult = response.json().await?;
            return Ok(analysis);
        }
        let err = response.json::<ErrorResponse>().await?;
        Err(Error::API(err))
    }

    // POST {Endpoint}/imageanalysis:analyze?features={features}&model-name={model-name}&language={language}&smartcrops-aspect-ratios={smartcrops-aspect-ratios}&gender-neutral-caption={gender-neutral-caption}&api-version=2023-04-01-preview
    fn build_query_params<'a>(
        options: &AnalyzeImageOptions,
    ) -> Vec<(&'a str, String)> {
        let mut query_params: Vec<(&str, String)> = Vec::new();

        query_params.push(("api-version", DEFAULT_API_VERSION.to_string()));

        if let Some(features) = options.features {
            if !features.is_empty() {
                query_params.push((
                    "features",
                    features
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }

        if let Some(gender_neutral_caption) = options.gender_neutral_caption {
            query_params.push((
                "gender-neutral-caption",
                gender_neutral_caption.to_string(),
            ));
        }

        if let Some(language) = options.language {
            if !language.is_empty() {
                query_params.push(("language", language.to_string()));
            }
        }

        if let Some(model_name) = options.model_name {
            if !model_name.is_empty() {
                query_params.push(("model-name", model_name.to_string()));
            }
        }

        if let Some(smartcrops_aspect_ratios) = options.smartcrops_aspect_ratios
        {
            if !smartcrops_aspect_ratios.is_empty() {
                query_params.push((
                    "smartcrops-aspect-ratios",
                    smartcrops_aspect_ratios.to_string(),
                ));
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
        ));
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
            features: Some(&[]),
            gender_neutral_caption: None,
            language: Some(""),
            model_name: Some(""),
            smartcrops_aspect_ratios: Some(""),
        };
        let query_params = Client::build_query_params(&options);

        assert_eq!(query_params.len(), 1);
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

        assert_eq!(query_params.len(), 1);
        assert_eq!(
            query_params[0],
            ("api-version", DEFAULT_API_VERSION.to_string())
        );
    }

    #[test]
    fn test_build_query_params_with_features() {
        let features = vec![VisualFeature::Tags, VisualFeature::Objects];
        let options = AnalyzeImageOptions {
            features: Some(&features),
            ..Default::default()
        };

        let query_params = Client::build_query_params(&options);

        assert!(
            query_params.contains(&("features", "tags,objects".to_string()))
        );
    }

    #[test]
    fn test_build_query_params_with_gender_neutral_caption() {
        let options = AnalyzeImageOptions {
            gender_neutral_caption: Some(true),
            ..Default::default()
        };

        let query_params = Client::build_query_params(&options);

        assert!(query_params
            .contains(&("gender-neutral-caption", "true".to_string())));
    }

    #[test]
    fn test_build_query_params_with_language() {
        let options = AnalyzeImageOptions {
            language: Some("fr"),
            ..Default::default()
        };

        let query_params = Client::build_query_params(&options);

        assert!(query_params.contains(&("language", "fr".to_string())));
    }

    #[test]
    fn test_build_query_params_with_smartcrops_aspect_ratios() {
        let options = AnalyzeImageOptions {
            smartcrops_aspect_ratios: Some("1.0,1.5"),
            ..Default::default()
        };

        let query_params = Client::build_query_params(&options);

        assert!(query_params
            .contains(&("smartcrops-aspect-ratios", "1.0,1.5".to_string())));
    }

    #[test]
    fn test_build_query_params_all_options() {
        let features = vec![VisualFeature::Tags];

        let options = AnalyzeImageOptions {
            features: Some(&features),
            gender_neutral_caption: Some(false),
            language: Some("en"),
            model_name: Some("my-model"),
            smartcrops_aspect_ratios: Some("1.0"),
        };

        let query_params = Client::build_query_params(&options);

        assert!(query_params
            .contains(&("api-version", DEFAULT_API_VERSION.to_string())));
        assert!(query_params.contains(&("features", "tags".to_string())));
        assert!(query_params
            .contains(&("gender-neutral-caption", "false".to_string())));
        assert!(query_params.contains(&("language", "en".to_string())));
        assert!(query_params.contains(&("model-name", "my-model".to_string())));
        assert!(query_params
            .contains(&("smartcrops-aspect-ratios", "1.0".to_string())));
    }
}
