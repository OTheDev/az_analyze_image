/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::*;
use az_analyze_image::v40::client::*;
use az_analyze_image::v40::*;

const MODEL_VERSION: &str = "2023-02-01-preview";

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=caption,denseCaptions,objects,people,read,smartCrops,tags&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/all_features.json
#[tokio::test]
async fn test_analyze_image_all_features() {
    let client = get_client();

    let features = vec![
        VisualFeature::Caption,
        VisualFeature::DenseCaptions,
        VisualFeature::Objects,
        VisualFeature::People,
        VisualFeature::Read,
        VisualFeature::SmartCrops,
        VisualFeature::Tags,
    ];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    // Check unspecified features
    assert!(analysis.adult_result.is_none());

    // Check mandatory output
    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    // Check caption result
    validate_caption(analysis.caption_result.expect("no caption result"));

    // Check dense caption result
    validate_dense_captions(
        analysis
            .dense_captions_result
            .expect("no dense captions result"),
    );

    // Check objects result
    validate_objects(analysis.objects_result.expect("no objects result"));

    // Check people result
    validate_people(analysis.people_result.expect("no people result"));

    // Check read result
    validate_read(analysis.read_result.expect("no read result"));

    // Check smart crops result
    validate_smart_crops(analysis.smart_crops_result.expect("no smart crops"));

    // Check tags result
    validate_tags(analysis.tags_result.expect("no tags result"));
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=caption&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/captions.json
fn validate_caption(caption: CaptionResult) {
    assert!(caption.text == "a man pointing at a screen");
    assert!(approx_eq_exp(caption.confidence, 0.7767, 4));
}

#[tokio::test]
async fn test_analyze_image_caption() {
    let client = get_client();

    let features = vec![VisualFeature::Caption];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_caption(analysis.caption_result.expect("no caption result"));
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=denseCaptions&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/dense_captions.json
fn validate_dense_captions(dcr: DenseCaptionsResult) {
    let dcr = dcr.values;

    assert!(dcr.len() == 10);

    let dcr_1 = dcr.get(0).unwrap();
    assert!(dcr_1.text == "a man pointing at a screen");
    assert!(approx_eq_exp(dcr_1.confidence, 0.7767, 4));
    assert!(dcr_1.bounding_box.x == 0);
    assert!(dcr_1.bounding_box.y == 0);
    assert!(dcr_1.bounding_box.w == 1038);
    assert!(dcr_1.bounding_box.h == 692);

    let dcr_9 = dcr.get(8).unwrap();
    assert!(dcr_9.text == "a person in a yellow coat");
    assert!(
        approx_eq_exp(dcr_9.confidence, 0.670, 3),
        "{}",
        dcr_9.confidence
    );
    assert!(dcr_9.bounding_box.x == 687);
    assert!(dcr_9.bounding_box.y == 199);
    assert!(dcr_9.bounding_box.w == 225);
    assert!(dcr_9.bounding_box.h == 356);
}

#[tokio::test]
async fn test_analyze_image_dense_captions() {
    let client = get_client();

    let features = vec![VisualFeature::DenseCaptions];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_dense_captions(
        analysis
            .dense_captions_result
            .expect("no dense captions result"),
    );
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=objects&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/objects.json
fn validate_objects(or: ObjectsResult) {
    let or = or.values;

    assert!(or.len() == 2);

    let or_1 = or.get(0).unwrap();
    assert!(or_1.bounding_box.x == 655);
    assert!(or_1.bounding_box.y == 83);
    assert!(or_1.bounding_box.w == 263);
    assert!(or_1.bounding_box.h == 605);
    assert!(or_1.tags[0].name == "person");
    assert!(or_1.tags[0].confidence == 0.905);
    assert!(or_1.id.is_none()); // TODO?: no test case for this

    let or_2 = or.get(1).unwrap();
    assert!(or_2.bounding_box.x == 75);
    assert!(or_2.bounding_box.y == 76);
    assert!(or_2.bounding_box.w == 678);
    assert!(or_2.bounding_box.h == 414);
    assert!(or_2.tags[0].name == "television");
    assert!(or_2.tags[0].confidence == 0.808);
    assert!(or_2.id.is_none());
}

#[tokio::test]
async fn test_analyze_image_objects() {
    let client = get_client();

    let features = vec![VisualFeature::Objects];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_objects(analysis.objects_result.expect("no objects result"));
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=people&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/people.json
fn validate_people(pr: PeopleResult) {
    let pr = pr.values;

    assert!(pr.len() == 4);

    let pr_1 = pr.get(0).unwrap();
    assert!(pr_1.bounding_box.x == 659);
    assert!(pr_1.bounding_box.y == 82);
    assert!(pr_1.bounding_box.w == 256);
    assert!(pr_1.bounding_box.h == 594);
    assert!(approx_eq_exp(pr_1.confidence, 0.959, 3));

    let pr_4 = pr.get(3).unwrap();
    assert!(pr_4.bounding_box.x == 0);
    assert!(pr_4.bounding_box.y == 0);
    assert!(pr_4.bounding_box.w == 203);
    assert!(pr_4.bounding_box.h == 141);
    assert!(approx_eq_exp(pr_4.confidence, 0.001100298948585987, 4),);
}

#[tokio::test]
async fn test_analyze_image_people() {
    let client = get_client();

    let features = vec![VisualFeature::People];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_people(analysis.people_result.expect("no people result"));
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=read&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/read.json
fn validate_read(rr: ReadResult) {
    assert!(rr.string_index_type == "TextElements");
    assert!(rr.content == "9:35 AM\nE Conference room 154584354\n#: 555-173-4547\nTown Hall\n9:00 AM - 10:00 AM\nAaron Buaion\nDaily SCRUM\n10:00 AM 11:00 AM\nChurlette de Crum\nQuarterly NI Hands\n11.00 AM-12:00 PM\nBebek Shaman\nWeekly stand up\n12:00 PM-1:00 PM\nDelle Marckre\nProduct review");
    assert!(rr.styles.is_empty());
    // API returns this but it is not documented.
    // assert!(rr.model_version == "2022-04-30");

    let rr_page_1 = &rr.pages[0];
    assert!(rr_page_1.spans.len() == 1);
    assert!(rr_page_1.spans[0].length == 253);
    assert!(rr_page_1.spans[0].offset == 0);

    assert!(rr_page_1.height == 692.0);
    assert!(rr_page_1.width == 1038.0);
    assert!(rr_page_1.angle == 0.3048);
    assert!(rr_page_1.page_number == 1);

    let rr_word_1 = &rr_page_1.words[0];
    assert!(rr_word_1.content == "9:35");
    assert!(
        rr_word_1.bounding_box
            == &[131.0, 130.0, 171.0, 130.0, 171.0, 149.0, 130.0, 149.0,]
    );
    assert!(rr_word_1.confidence == 0.993);
    assert!(rr_word_1.span.offset == 0);
    assert!(rr_word_1.span.length == 4);

    let rr_line_2 = &rr_page_1.lines[1];
    assert!(rr_line_2.content == "E Conference room 154584354");
    assert!(
        rr_line_2.bounding_box
            == &[130.0, 153.0, 224.0, 154.0, 224.0, 161.0, 130.0, 161.0]
    );
    assert!(rr_line_2.spans[0].offset == 8);
    assert!(rr_line_2.spans[0].length == 27);
}

#[tokio::test]
async fn test_analyze_image_read() {
    let client = get_client();

    let features = vec![VisualFeature::Read];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_read(analysis.read_result.expect("no read result"));
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=smartCrops&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/smart_crops.json
fn validate_smart_crops(scr: SmartCropsResult) {
    let scr = scr.values;

    assert!(scr[0].aspect_ratio == 1.09);
    assert!(scr[0].bounding_box.x == 303);
    assert!(scr[0].bounding_box.y == 29);
    assert!(scr[0].bounding_box.w == 692);
    assert!(scr[0].bounding_box.h == 634);
}

#[tokio::test]
async fn test_analyze_image_smart_crops() {
    let client = get_client();

    let features = vec![VisualFeature::SmartCrops];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_smart_crops(
        analysis.smart_crops_result.expect("no smart crops result"),
    );
}

// curl "${CV_ENDPOINT}/computervision/imageanalysis:analyze?features=tags&api-version=2023-04-01-preview" \
//     -H "Ocp-Apim-Subscription-Key: ${CV_KEY}" \
//     -H "Content-Type: application/json" \
//     -d "{'url':'https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png'}" \
//     | jq > tests/samples/v40/tags.json
fn validate_tags(tr: TagsResult) {
    let tr = tr.values;

    assert!(tr.len() == 22);

    assert!(tr[0].name == "text");
    assert!(approx_eq_exp(tr[0].confidence, 0.996, 3));

    assert!(tr[21].name == "design");
    assert!(approx_eq_exp(tr[21].confidence, 0.404, 3));
}

#[tokio::test]
async fn test_analyze_image_tags() {
    let client = get_client();

    let features = vec![VisualFeature::Tags];

    let options = AnalyzeImageOptions {
        features: Some(&features),
        ..Default::default()
    };

    let analysis = client
        .analyze_image_url(URL::Quickstart.into(), options)
        .await
        .unwrap();

    assert!(analysis.model_version == MODEL_VERSION);
    assert!(analysis.metadata.width == 1038);
    assert!(analysis.metadata.height == 692);

    validate_tags(analysis.tags_result.expect("no tags result"));
}
