/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::{Number, PixelCount};
use serde::{Deserialize, Serialize};

/// An object describing adult content match.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdultMatch {
    /// A value indicating the confidence level of matched adult content.
    pub confidence: Number,

    /// A value indicating if the image is matched adult content.
    #[serde(rename = "isMatch")]
    pub is_match: bool,
}

/// An object describing whether the image contains adult-oriented content
/// and/or is racy.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdultResult {
    /// An object describing adult content match.
    pub adult: AdultMatch,

    /// An object describing adult content match.
    pub gore: AdultMatch,

    /// An object describing adult content match.
    pub racy: AdultMatch,
}

/// A bounding box for an area inside an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    /// Height measured from the top-left point of the area, in pixels.
    pub h: PixelCount,

    /// Width measured from the top-left point of the area, in pixels.
    pub w: PixelCount,

    /// Left-coordinate of the top left point of the area, in pixels.
    pub x: PixelCount,

    /// Top-coordinate of the top left point of the area, in pixels.
    pub y: PixelCount,
}

/// A brief description of what the image depicts.
#[derive(Debug, Deserialize, Serialize)]
pub struct CaptionResult {
    /// The level of confidence the service has in the caption.
    pub confidence: Number,

    /// The text of the caption.
    pub text: String,
}

/// A region identified for smart cropping. There will be one region returned
/// for each requested aspect ratio.
#[derive(Debug, Deserialize, Serialize)]
pub struct CropRegion {
    /// The aspect ratio of the crop region.
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Number,

    /// A bounding box for an area inside an image.
    #[serde(rename = "boundingBox")]
    pub bounding_box: BoundingBox,
}

/// A brief description of what the image depicts.
#[derive(Debug, Deserialize, Serialize)]
pub struct DenseCaption {
    /// A bounding box for an area inside an image.
    #[serde(rename = "boundingBox")]
    pub bounding_box: BoundingBox,

    /// The level of confidence the service has in the caption.
    pub confidence: Number,

    /// The text of the caption.
    pub text: String,
}

/// A list of captions.
#[derive(Debug, Deserialize, Serialize)]
pub struct DenseCaptionsResult {
    /// A list of captions.
    pub values: Vec<DenseCaption>,
}

/// Describes a detected object in an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct DetectedObject {
    /// A bounding box for an area inside an image.
    #[serde(rename = "boundingBox")]
    pub bounding_box: BoundingBox,

    /// Id of the detected object.
    // pub id: String,
    // TODO: When is this ever provided?
    pub id: Option<String>,

    /// Classification confidences of the detected object.
    pub tags: Vec<Tag>,
}

/// A person detected in an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct DetectedPerson {
    /// A bounding box for an area inside an image.
    #[serde(rename = "boundingBox")]
    pub bounding_box: BoundingBox,

    /// Confidence score of having observed the person in the image, as a value
    /// ranging from 0 to 1.
    pub confidence: Number,
}

/// A content line object consisting of an adjacent sequence of content
/// elements, such as words and selection marks.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentLine {
    /// Bounding box of the line.
    #[serde(rename = "boundingBox")]
    pub bounding_box: Vec<Number>, // number[]. TODO: maybe Vec<PixelCount>?

    /// Concatenated content of the contained elements in reading order.
    pub content: String,

    /// Location of the line in the reading order concatenated content.
    pub spans: Vec<DocumentSpan>,
}

/// The content and layout elements extracted from a page from the input.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentPage {
    /// The general orientation of the content in clockwise direction, measured
    /// in degrees between (-180, 180].
    pub angle: Number, // number

    /// The height of the image/PDF in pixels/inches, respectively.
    pub height: Number, // PixelCount more ideal, but needs to be a float

    /// Extracted lines from the page, potentially containing both textual and
    /// visual elements.
    pub lines: Vec<DocumentLine>,

    /// 1-based page number in the input document.
    #[serde(rename = "pageNumber")]
    pub page_number: usize,

    /// Location of the page in the reading order concatenated content.
    pub spans: Vec<DocumentSpan>,

    /// The width of the image/PDF in pixels/inches, respectively.
    pub width: Number,

    /// Extracted words from the page.
    pub words: Vec<DocumentWord>,
}

/// Contiguous region of the concatenated content property, specified as an
/// offset and length.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentSpan {
    /// Number of characters in the content represented by the span.
    pub length: usize,

    /// Zero-based index of the content represented by the span.
    pub offset: usize,
}

/// An object representing observed text styles.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentStyle {
    /// Confidence of correctly identifying the style.
    pub confidence: Number,

    /// Is content handwritten or not.
    #[serde(rename = "isHandwritten")]
    pub is_handwritten: bool,

    /// Location of the text elements in the concatenated content the style
    /// applies to.
    pub spans: Vec<DocumentSpan>,
}

/// A word object consisting of a contiguous sequence of characters. For
/// non-space delimited languages, such as Chinese, Japanese, and Korean, each
/// character is represented as its own word.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentWord {
    /// Bounding box of the word.
    #[serde(rename = "boundingBox")]
    pub bounding_box: Vec<Number>, // number[]. TODO: maybe Vec<PixelCount>?

    /// Confidence of correctly extracting the word.
    pub confidence: Number,

    /// Text content of the word.
    pub content: String,

    /// Contiguous region of the concatenated content property, specified as an
    /// offset and length.
    pub span: DocumentSpan,
}

/// Response returned when an error occurs.
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
#[error("{:#?}", error)] // TODO.
pub struct ErrorResponse {
    /// Error info.
    pub error: ErrorResponseDetails,
}

/// Error info.
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponseDetails {
    /// Error code.
    pub code: String,

    /// List of detailed errors. TODO: must be optional?
    pub details: Option<Vec<ErrorResponseDetails>>,

    /// Detailed error. TODO: must be optional?
    pub innererror: Option<ErrorResponseInnerError>,

    /// Error message.
    pub message: String,

    /// Target of the error. TODO: must be optional?
    pub target: Option<String>,
}

/// Detailed error.
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponseInnerError {
    /// Error code.
    pub code: String,

    /// Detailed error.
    pub innererror: Box<ErrorResponseInnerError>,

    /// Error message.
    pub message: String,
}

/// Describe the combined results of different types of image analysis.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageAnalysisResult {
    /// An object describing whether the image contains adult-oriented content
    /// and/or is racy.
    // TODO: This is not currently documented as a VisualFeature in their
    // definitions, but it is documented for an ImageAnalysisResult. Moreover,
    // adding the VisualFeature::Adult variant causes an OperationBlocked error
    // code. It may not be supported yet.
    #[serde(rename = "adultResult")]
    pub adult_result: Option<AdultResult>,

    /// A brief description of what the image depicts.
    #[serde(rename = "captionResult")]
    pub caption_result: Option<CaptionResult>,

    /// Describes the prediction result of an image.
    #[serde(rename = "customModelResult")]
    pub custom_model_result: Option<ImagePredictionResult>,

    /// A list of captions.
    #[serde(rename = "denseCaptionsResult")]
    pub dense_captions_result: Option<DenseCaptionsResult>,

    /// The image metadata information such as height and width.
    pub metadata: ImageMetadataApiModel,

    /// Model Version.
    #[serde(rename = "modelVersion")]
    pub model_version: String,

    /// Describes detected objects in an image.
    #[serde(rename = "objectsResult")]
    pub objects_result: Option<ObjectsResult>,

    /// An object describing whether the image contains people.
    #[serde(rename = "peopleResult")]
    pub people_result: Option<PeopleResult>,

    /// The results of an Read operation.
    #[serde(rename = "readResult")]
    pub read_result: Option<ReadResult>,

    /// Smart cropping result.
    #[serde(rename = "smartCropsResult")]
    pub smart_crops_result: Option<SmartCropsResult>,

    /// A list of tags with confidence level.
    #[serde(rename = "tagsResult")]
    pub tags_result: Option<TagsResult>,
}

/// The image metadata information such as height and width.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageMetadataApiModel {
    /// The height of the image in pixels.
    pub height: PixelCount,

    /// The width of the image in pixels.
    pub width: PixelCount,
}

/// Describes the prediction result of an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImagePredictionResult {
    /// Describes detected objects in an image.
    #[serde(rename = "objectsResult")]
    pub objects_result: ObjectsResult,

    /// A list of tags with confidence level.
    #[serde(rename = "tagsResult")]
    pub tags_result: TagsResult,
}

/// A JSON document with a URL pointing to the image that is to be analyzed.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageUrl {
    /// Publicly reachable URL of an image.
    pub url: String,
}

/// Describes detected objects in an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectsResult {
    /// An array of detected objects.
    pub values: Vec<DetectedObject>,
}

/// An object describing whether the image contains people.
#[derive(Debug, Deserialize, Serialize)]
pub struct PeopleResult {
    /// An array of detected people.
    pub values: Vec<DetectedPerson>,
}

/// The results of an Read operation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadResult {
    /// Concatenate string representation of all textual and visual elements in
    /// reading order.
    pub content: String,

    /// A list of analyzed pages.
    pub pages: Vec<DocumentPage>,

    /// The method used to compute string offset and length, possible values
    /// include: 'textElements', 'unicodeCodePoint', 'utf16CodeUnit' etc.
    #[serde(rename = "stringIndexType")]
    pub string_index_type: String,

    /// Extracted font styles.
    pub styles: Vec<DocumentStyle>,
}

/// Smart cropping result.
#[derive(Debug, Deserialize, Serialize)]
pub struct SmartCropsResult {
    /// Recommended regions for cropping the image.
    pub values: Vec<CropRegion>,
}

/// An entity observation in the image, along with the confidence score.
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    /// The level of confidence that the entity was observed.
    pub confidence: Number,

    /// Name of the entity.
    pub name: String,
}

/// A list of tags with confidence level.
#[derive(Debug, Deserialize, Serialize)]
pub struct TagsResult {
    /// A list of tags with confidence level.
    pub values: Vec<Tag>,
}

/// The visual features requested: `tags`, `objects`, `caption`, `denseCaptions`
/// , `read`, `smartCrops`, `people`. This parameter needs to be specified if
/// the parameter "model-name" is not specified.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum VisualFeature {
    #[serde(rename = "caption")]
    Caption,

    #[serde(rename = "denseCaptions")]
    DenseCaptions,

    #[serde(rename = "objects")]
    Objects,

    #[serde(rename = "people")]
    People,

    #[serde(rename = "read")]
    Read,

    #[serde(rename = "smartCrops")]
    SmartCrops,

    #[serde(rename = "tags")]
    Tags,
}

impl std::fmt::Display for VisualFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let feature_str = match self {
            VisualFeature::Caption => "caption",
            VisualFeature::DenseCaptions => "denseCaptions",
            VisualFeature::Objects => "objects",
            VisualFeature::People => "people",
            VisualFeature::Read => "read",
            VisualFeature::SmartCrops => "smartCrops",
            VisualFeature::Tags => "tags",
        };
        write!(f, "{}", feature_str)
    }
}
