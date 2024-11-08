/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::*;

use az_analyze_image::v32::client::*;
use az_analyze_image::v32::*;

/* Validation Error */

#[tokio::test]
async fn test_validation_error_invalid_header_value() {
    let bad_key = "bad\nkey";
    let endpoint = std::env::var("CV_ENDPOINT").expect("no CV_ENDPOINT");

    let client = Client::new(bad_key.into(), &endpoint);
    match client {
        Err(Error::Validation(ValidationError::InvalidHeaderValue(_))) => {
            /* Passed */
        }
        other => {
            panic!("Expected InvalidHeaderValue, but got other {:?}", other);
        }
    };
}

/* API Error */

#[tokio::test]
async fn test_api_error_invalid_image_size() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Faces];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        ..Default::default()
    };

    match client
        .analyze_image_url(URL::GreaterThan4MB.into(), options)
        .await
    {
        Err(Error::API(err)) => {
            assert!(
                err.error.innererror.code
                    == ComputerVisionInnerErrorCodeValue::InvalidImageSize
            );
        }
        other => {
            panic!("Expected InvalidImageSize, but got other {:?}", other);
        }
    }
}

#[tokio::test]
async fn test_api_error_unsupported_celebrities_feature() {
    let client = get_client();

    let visual_features =
        vec![VisualFeatureTypes::Tags, VisualFeatureTypes::Description];

    let details = vec![Details::Celebrities];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        details: Some(&details),
        language: Some("en"),
        description_exclude: None,
        model_version: Some("latest"),
    };

    match client.analyze_image_url(URL::Default.into(), options).await {
        Err(Error::API(err)) => assert!(
            ComputerVisionInnerErrorCodeValue::NotSupportedFeature
                == err.error.innererror.code
        ),
        other => {
            panic!("Expected NotSupportedFeature, but got other {:?}", other);
        }
    }
}
