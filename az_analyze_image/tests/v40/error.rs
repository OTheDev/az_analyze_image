/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::*;

use az_analyze_image::v40::client::*;
use az_analyze_image::v40::*;

/* Validation Error */

// Invalid API key supplied
#[tokio::test]
async fn test_validation_error() {
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

#[tokio::test]
async fn test_validation_error_no_features_or_model_name() {
    let client = get_client();

    let f =
        std::fs::read("./tests/images/people.jpg").expect("Error reading file");

    let options = AnalyzeImageOptions::default();

    match client.analyze_image(&f, options).await {
        Err(Error::Validation(ValidationError::NoFeaturesOrModelName)) => {
            /* Passed */
        }
        other => {
            println!(
                "Expected NoFeaturesOrModelName, but got other {:?}",
                other
            );
        }
    }
}

/* API Error */

#[tokio::test]
async fn test_api_error_image_too_large() {
    let client = get_client();

    let features = vec![VisualFeature::People];
    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    match client
        .analyze_image_url(URL::GreaterThan20MB.into(), options)
        .await
    {
        Err(Error::API(err)) => {
            assert!(err.error.code == "InvalidRequest");
            assert!(
                err.error.message
                    == "The image size is not allowed to be zero or larger \
                 than 20971520 bytes."
            )
        }
        other => {
            panic!("Expected Error::API, but got other {:?}", other);
        }
    }
}
