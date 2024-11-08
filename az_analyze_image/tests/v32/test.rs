/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::*;
use az_analyze_image::v32::client::*;
use az_analyze_image::v32::*;

const MODEL_VERSION: &str = "2021-05-01";

// curl "${CV_ENDPOINT}vision/v3.2/analyze" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v32/no_query_parameters.json
#[tokio::test]
async fn test_no_parameters() {
    let client = get_client();

    let options = AnalyzeImageOptions::default();

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.metadata.height == 692);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.format == "Png");
    assert!(!analysis.request_id.is_empty());
    assert!(!analysis.model_version.is_empty());

    // By default, with no features specified, image categories are returned in
    // the response.
    let categories = analysis.categories.expect("no categories");

    let c0 = &categories[0];
    let c1 = &categories[1];

    assert!(c0.name == "others_");
    assert!(c0.score == 0.0234375);

    assert!(c1.name == "object_screen");
    assert!(c1.score == 0.91796875);
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Adult,Brands,Categories,Color,Description,Faces,ImageType,Objects,Tags" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v32/all_features.json
#[tokio::test]
async fn test_analyze_image_all_features() {
    let client = get_client();

    let visual_features = vec![
        VisualFeatureTypes::Adult,
        VisualFeatureTypes::Brands,
        VisualFeatureTypes::Categories,
        VisualFeatureTypes::Color,
        VisualFeatureTypes::Description,
        VisualFeatureTypes::Faces,
        VisualFeatureTypes::ImageType,
        VisualFeatureTypes::Objects,
        VisualFeatureTypes::Tags,
    ];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    // Check model_version
    assert!(analysis.model_version == MODEL_VERSION);

    // Check metadata
    assert!(analysis.metadata.height == 692);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.format == "Png");

    // Check request id
    assert!(!analysis.request_id.is_empty());

    // Check adult
    let adult = analysis.adult.expect("no adult");
    assert!(adult.is_adult_content == false);
    assert!(adult.is_racy_content == false);
    assert!(adult.is_gory_content == false);
    assert!(approx_eq_exp(adult.adult_score, 0.00326, 5));
    assert!(approx_eq_exp(adult.racy_score, 0.01739, 5));
    assert!(approx_eq_exp(adult.gore_score, 0.00132, 5));

    // Check brands
    let brands = analysis.brands.expect("no brands");
    assert!(brands.is_empty());

    // Check categories
    let categories = analysis.categories.expect("no categories");
    assert!(categories[0].name == "others_");
    assert!(approx_eq_exp(categories[0].score, 0.02343, 5));
    assert!(categories[1].name == "object_screen");
    assert!(approx_eq_exp(categories[1].score, 0.91796, 5));

    // Check color
    let color = analysis.color.expect("no color");
    assert!(color.dominant_color_background == "White");
    assert!(color.dominant_color_foreground == "White");
    assert!(color.dominant_colors[0] == "White");
    assert!(color.accent_color == "1E6A8C");
    assert!(color.is_bw_img == false);

    // Check description
    let description = analysis.description.expect("no description");
    assert!(
        description.tags
            == &[
                "text",
                "person",
                "indoor",
                "electronics",
                "television",
                "standing",
                "display"
            ]
    );
    assert!(description.captions[0].text == "a man pointing at a screen");
    assert!(approx_eq_exp(
        description.captions[0].confidence,
        0.50986,
        5
    ));

    // Check faces
    let faces = analysis.faces.expect("no faces");
    assert!(faces.is_empty());

    // Check image type
    let image_type = analysis.image_type.expect("no image type");
    assert!(image_type.clipart_type == 0);
    assert!(image_type.line_drawing_type == 0);

    // Check objects
    let objects = analysis.objects.expect("no objects");

    assert!(objects.len() == 2);

    let object_0 = &objects[0];
    let object_1 = &objects[1];

    assert!(object_0.rectangle.x == 655);
    assert!(object_0.rectangle.y == 83);
    assert!(object_0.rectangle.w == 263);
    assert!(object_0.rectangle.h == 605);
    assert!(object_0.object == "person");
    assert!(object_0.confidence == 0.905);

    assert!(object_1.rectangle.x == 75);
    assert!(object_1.rectangle.y == 76);
    assert!(object_1.rectangle.w == 678);
    assert!(object_1.rectangle.h == 414);
    assert!(object_1.object == "television");
    assert!(object_1.confidence == 0.808);
    let parent = object_1.parent.as_ref().unwrap();
    assert!(parent.object == "display");
    assert!(parent.confidence == 0.851);

    // Check tags
    let tags = analysis.tags.unwrap();
    assert!(tags.len() == 22);

    let tag_0 = &tags[0];
    assert!(tag_0.name == "text");
    assert!(approx_eq_exp(tag_0.confidence, 0.99660, 5));

    let tag_21 = &tags[21];
    assert!(tag_21.name == "design");
    assert!(approx_eq_exp(tag_21.confidence, 0.40424, 5));
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Adult" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/adult.json
#[tokio::test]
async fn test_analyze_image_adult() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Adult];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let adult = analysis.adult.expect("no adult");
    assert_eq!(adult.is_adult_content, false);
    assert_eq!(adult.is_racy_content, false);
    assert_eq!(adult.is_gory_content, false);
    assert!(approx_eq_exp(adult.adult_score, 0.00094, 5));
    assert!(approx_eq_exp(adult.racy_score, 0.00230, 5));
    assert!(approx_eq_exp(adult.gore_score, 0.00211, 5));
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Brands" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2016/04/20/00/41/mcdonalds-1340199_1280.jpg'}" \
//     | jq > tests/samples/v32/brands.json
#[tokio::test]
async fn test_analyze_image_brands() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Brands];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Macys.into(), options)
        .await
        .unwrap();

    assert!(analysis.metadata.height == 854);
    assert!(analysis.metadata.width == 1280);
    assert!(analysis.metadata.format == "Jpeg");
    assert!(analysis.model_version == MODEL_VERSION);
    assert!(!analysis.request_id.is_empty());

    let brands = analysis.brands.expect("no brands");
    assert!(brands.len() == 1);
    assert!(brands[0].name == "Macy's");
    assert!(brands[0].confidence == 0.667);
    assert!(brands[0].rectangle.x == 272);
    assert!(brands[0].rectangle.y == 19);
    assert!(brands[0].rectangle.w == 200);
    assert!(brands[0].rectangle.h == 239);
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Categories&details=Landmarks&modelVersion=2021-05-01" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/categories_1.json

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Categories&details=Landmarks&modelVersion=2021-05-01" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2016/01/16/17/13/big-ben-1143631_1280.jpg'}" \
//     | jq > tests/samples/v32/categories_2.json

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Categories&details=Landmarks&modelVersion=2021-05-01" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2013/11/28/10/37/forbidden-city-220099_1280.jpg'}" \
//     | jq > tests/samples/v32/categories_3.json
#[tokio::test]
async fn test_analyze_image_categories() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Categories];
    let details = vec![Details::Landmarks];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        details: Some(&details),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();

    assert!(analysis.metadata.height == 853);
    assert!(analysis.metadata.width == 1280);
    assert!(analysis.metadata.format == "Jpeg");
    assert!(analysis.model_version == MODEL_VERSION);
    assert!(!analysis.request_id.is_empty());

    let categories = analysis.categories.expect("no categories");

    assert!(categories[0].name == "abstract_");
    assert!(categories[0].score == 0.00390625);
    assert!(categories[0].detail.is_none());

    assert!(categories[1].name == "others_");
    assert!(categories[1].score == 0.01953125);
    assert!(categories[1].detail.is_none());

    assert!(categories[2].name == "people_portrait");
    assert!(categories[2].score == 0.390625);
    assert!(categories[2].detail.is_none());

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        details: Some(&details),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::BigBen.into(), options)
        .await
        .unwrap();

    let categories = analysis.categories.expect("no categories");

    assert!(categories[0].name == "building_");
    assert!(categories[0].score == 0.51953125);
    assert!(categories[0]
        .detail
        .as_ref()
        .unwrap()
        .landmarks
        .as_ref()
        .unwrap()
        .is_empty());

    assert!(categories[1].name == "building_pillar");
    assert!(categories[1].score == 0.34765625);
    assert!(categories[1]
        .detail
        .as_ref()
        .unwrap()
        .landmarks
        .as_ref()
        .unwrap()
        .is_empty());

    assert!(categories[2].name == "outdoor_");
    assert!(categories[2].score == 0.00390625);
    assert!(categories[2]
        .detail
        .as_ref()
        .unwrap()
        .landmarks
        .as_ref()
        .unwrap()
        .is_empty());

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        details: Some(&details),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::ForbiddenCity.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.height == 859);
    assert!(analysis.metadata.width == 1280);
    assert!(analysis.metadata.format == "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let categories = analysis.categories.expect("no categories");

    assert!(categories[0].name == "building_");
    assert!(categories[0].score == 0.6015625);

    let d0 = categories[0].detail.as_ref().unwrap();
    assert!(d0.celebrities.is_none());

    let l0 = d0.landmarks.as_ref().unwrap().get(0).unwrap();
    assert!(l0.name == "Forbidden City");
    assert!(approx_eq_exp(l0.confidence, 0.99453, 5));

    assert!(categories[1].name == "outdoor_");
    assert!(categories[1].score == 0.01953125);

    let d1 = categories[1].detail.as_ref().unwrap();
    assert!(d1.celebrities.is_none());

    let l1 = d1.landmarks.as_ref().unwrap().get(0).unwrap();
    assert!(l1.name == "Forbidden City");
    assert!(approx_eq_exp(l1.confidence, 0.99453, 5));
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Color" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/color.json
#[tokio::test]
async fn test_analyze_image_color() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Color];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let color = analysis.color.expect("no color");
    assert_eq!(color.dominant_color_foreground, "Grey");
    assert_eq!(color.dominant_color_background, "Black");
    assert_eq!(color.dominant_colors[0], "Black");
    assert_eq!(color.dominant_colors[1], "Grey");
    assert_eq!(color.accent_color, "666666");
    assert_eq!(color.is_bw_img, true);
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Description" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/description.json
#[tokio::test]
async fn test_analyze_image_description() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Description];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let description = analysis.description.expect("no description");
    assert_eq!(
        description.tags,
        &["man", "person", "hat", "wearing", "old"]
    );
    assert_eq!(description.captions.len(), 1);
    assert_eq!(description.captions[0].text, "a man wearing a hat");
    assert!(approx_eq_exp(
        description.captions[0].confidence,
        0.58684,
        5
    ));
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Faces" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/faces.json
#[tokio::test]
async fn test_analyze_image_faces() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Faces];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();

    assert!(analysis.metadata.height == 853);
    assert!(analysis.metadata.width == 1280);
    assert!(analysis.metadata.format == "Jpeg");
    assert!(analysis.model_version == MODEL_VERSION);
    assert!(!analysis.request_id.is_empty());

    let faces = analysis.faces.expect("no faces");

    assert!(faces.len() == 1);

    let rect = &faces[0].face_rectangle;
    assert!(rect.left == 352);
    assert!(rect.top == 285);
    assert!(rect.width == 229);
    assert!(rect.height == 229);

    assert!(&faces[0].age.is_none());
    assert!(&faces[0].gender.is_none());
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=ImageType" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/image_type.json
#[tokio::test]
async fn test_analyze_image_image_type() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::ImageType];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let image_type = analysis.image_type.expect("no image type");
    assert_eq!(image_type.clipart_type, 0);
    assert_eq!(image_type.line_drawing_type, 0);
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Objects" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/objects.json
#[tokio::test]
async fn test_analyze_image_objects() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Objects];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let objects = analysis.objects.expect("no objects");

    assert!(objects.len() == 2);

    let o1 = &objects[0];
    assert_eq!(o1.rectangle.x, 210);
    assert_eq!(o1.rectangle.y, 65);
    assert_eq!(o1.rectangle.w, 518);
    assert_eq!(o1.rectangle.h, 308);

    assert_eq!(o1.object, "hat");
    assert_eq!(o1.confidence, 0.61);

    let o1_p = o1.parent.as_ref().unwrap();
    assert_eq!(o1_p.object, "headwear");
    assert_eq!(o1_p.confidence, 0.784);

    let o2 = &objects[1];
    assert_eq!(o2.rectangle.x, 35);
    assert_eq!(o2.rectangle.y, 52);
    assert_eq!(o2.rectangle.w, 918);
    assert_eq!(o2.rectangle.h, 795);

    assert_eq!(o2.object, "person");
    assert_eq!(o2.confidence, 0.908);

    assert!(o2.parent.is_none());
}

// curl "${CV_ENDPOINT}vision/v3.2/analyze?visualFeatures=Tags" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg'}" \
//     | jq > tests/samples/v32/tags.json
#[tokio::test]
async fn test_analyze_image_tags() {
    let client = get_client();

    let visual_features = vec![VisualFeatureTypes::Tags];

    let options = AnalyzeImageOptions {
        visual_features: Some(&visual_features),
        model_version: Some(MODEL_VERSION),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::PersonWithHat.into(), options)
        .await
        .unwrap();
    assert_eq!(analysis.model_version, MODEL_VERSION);
    assert_eq!(analysis.metadata.height, 853);
    assert_eq!(analysis.metadata.width, 1280);
    assert_eq!(analysis.metadata.format, "Jpeg");
    assert!(!analysis.request_id.is_empty());

    let tags = analysis.tags.expect("no tags");

    let expected = [
        ImageTag {
            name: "black and white".into(),
            confidence: 0.9927051067352295,
            hint: None,
        },
        ImageTag {
            name: "headdress".into(),
            confidence: 0.9894881248474121,
            hint: None,
        },
        ImageTag {
            name: "human face".into(),
            confidence: 0.983238935470581,
            hint: None,
        },
        ImageTag {
            name: "person".into(),
            confidence: 0.9734217524528503,
            hint: None,
        },
        ImageTag {
            name: "clothing".into(),
            confidence: 0.9706429243087769,
            hint: None,
        },
        ImageTag {
            name: "outdoor".into(),
            confidence: 0.9667158722877502,
            hint: None,
        },
        ImageTag {
            name: "hat".into(),
            confidence: 0.966546893119812,
            hint: None,
        },
        ImageTag {
            name: "sun hat".into(),
            confidence: 0.9661399722099304,
            hint: None,
        },
        ImageTag {
            name: "building".into(),
            confidence: 0.9522091746330261,
            hint: None,
        },
        ImageTag {
            name: "fashion accessory".into(),
            confidence: 0.9202934503555298,
            hint: None,
        },
        ImageTag {
            name: "cowboy hat".into(),
            confidence: 0.9152283668518066,
            hint: None,
        },
        ImageTag {
            name: "street".into(),
            confidence: 0.9075048565864563,
            hint: None,
        },
        ImageTag {
            name: "man".into(),
            confidence: 0.9019761681556702,
            hint: None,
        },
        ImageTag {
            name: "fedora".into(),
            confidence: 0.880980372428894,
            hint: None,
        },
        ImageTag {
            name: "headgear".into(),
            confidence: 0.8412641286849976,
            hint: None,
        },
        ImageTag {
            name: "monochrome".into(),
            confidence: 0.8008297681808472,
            hint: None,
        },
        ImageTag {
            name: "ground".into(),
            confidence: 0.7592607140541077,
            hint: None,
        },
        ImageTag {
            name: "wearing".into(),
            confidence: 0.6703507900238037,
            hint: None,
        },
    ];

    assert!(tags.len() == 18);
    assert!(tags.len() == expected.len());

    for (tag, expected_tag) in tags.iter().zip(expected.iter()) {
        assert_eq!(tag.name, expected_tag.name);
        assert!((tag.confidence - expected_tag.confidence).abs() < 0.001);
        assert_eq!(tag.hint, expected_tag.hint);
    }
}
