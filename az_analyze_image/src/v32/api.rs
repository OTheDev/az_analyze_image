/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use super::{Number, PixelCount};
use serde::{Deserialize, Serialize};

/// An object describing whether the image contains adult-oriented content
/// and/or is racy.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdultInfo {
    /// Score from 0 to 1 that indicates how much the content is considered
    /// adult-oriented within the image.
    #[serde(rename = "adultScore")]
    pub adult_score: Number,

    /// Score from 0 to 1 that indicates how gory is the image.
    #[serde(rename = "goreScore")]
    pub gore_score: Number,

    /// A value indicating if the image contains adult-oriented content.
    #[serde(rename = "isAdultContent")]
    pub is_adult_content: bool,

    /// A value indicating if the image is gory.
    #[serde(rename = "isGoryContent")]
    pub is_gory_content: bool,

    /// A value indicating if the image is racy.
    #[serde(rename = "isRacyContent")]
    pub is_racy_content: bool,

    /// Score from 0 to 1 that indicates how suggestive is the image.
    #[serde(rename = "racyScore")]
    pub racy_score: Number,
}

/// A bounding box for an area inside an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingRect {
    /// Height measured from the top-left point of the area, in pixels.
    pub h: PixelCount,

    /// Width measured from the top-left point of the area, in pixels.
    pub w: PixelCount,

    /// X-coordinate of the top left point of the area, in pixels.
    pub x: PixelCount,

    /// Y-coordinate of the top left point of the area, in pixels.
    pub y: PixelCount,
}

/// An object describing identified category.
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    /// Details of the identified category.
    pub detail: Option<CategoryDetail>, // m.b.o

    /// Name of the category.
    pub name: String,

    /// Scoring of the category.
    pub score: Number,
}

/// An object describing additional category details.
#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryDetail {
    /// An array of celebrities if any identified.
    pub celebrities: Option<Vec<CelebritiesModel>>, // m.b.o

    /// An array of landmarks if any identified.
    pub landmarks: Option<Vec<LandmarksModel>>, // m.b.o
}

/// An object describing possible celebrity identification.
#[derive(Debug, Deserialize, Serialize)]
pub struct CelebritiesModel {
    /// Confidence level for the celebrity recognition as a value ranging from
    /// 0 to 1.
    pub confidence: Number,

    /// Location of the identified face in the image.
    #[serde(rename = "faceRectangle")]
    pub face_rectangle: FaceRectangle,

    /// Name of the celebrity.
    pub name: String,
}

/// An object providing additional metadata describing color attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct ColorInfo {
    // TODO: should the possibles use Option? Need test case.
    /// Possible accent color.
    #[serde(rename = "accentColor")]
    pub accent_color: String,

    /// Possible dominant background color.
    #[serde(rename = "dominantColorBackground")]
    pub dominant_color_background: String,

    /// Possible dominant foreground color.
    #[serde(rename = "dominantColorForeground")]
    pub dominant_color_foreground: String,

    /// An array of possible dominant colors.
    #[serde(rename = "dominantColors")]
    pub dominant_colors: Vec<String>,

    /// A value indicating if the image is black and white.
    #[serde(rename = "isBWImg")]
    pub is_bw_img: bool,
}

/// The API request error.
#[derive(Debug, Deserialize, Serialize)]
pub struct ComputerVisionError {
    /// The error code.
    pub code: ComputerVisionErrorCodes,

    /// Inner error contains more specific information.
    pub innererror: ComputerVisionInnerError,

    /// A message explaining the error reported by the service.
    pub message: String,
}

/// The error code.
#[derive(Debug, Deserialize, Serialize)]
pub enum ComputerVisionErrorCodes {
    InternalServerError,
    InvalidArgument,
    InvalidRequest,
    ServiceUnavailable,
}

impl std::fmt::Display for ComputerVisionErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InternalServerError => "Internal Server Error",
            Self::InvalidArgument => "Invalid Argument",
            Self::InvalidRequest => "Invalid Request",
            Self::ServiceUnavailable => "Service Unavailable",
        };

        write!(f, "{}", s)
    }
}

/// The API error response.
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
#[error("{:?}", error)] // TODO.
pub struct ComputerVisionErrorResponse {
    /// Error contents.
    pub error: ComputerVisionError,
}

/// Details about the API request error.
#[derive(Debug, Deserialize, Serialize)]
pub struct ComputerVisionInnerError {
    /// The error code.
    pub code: ComputerVisionInnerErrorCodeValue,

    /// Error message.
    pub message: String,
}

/// The error code.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ComputerVisionInnerErrorCodeValue {
    BadArgument,
    CancelledRequest,
    DetectFaceError,
    FailedToProcess,
    InternalServerError,
    InvalidDetails,
    InvalidImageFormat,
    InvalidImageSize,
    InvalidImageUrl,
    InvalidModel,
    InvalidThumbnailSize,
    // TODO: API seems to return UnsupportedFeature instead.
    #[serde(rename = "UnsupportedFeature")]
    NotSupportedFeature,
    NotSupportedImage,
    NotSupportedLanguage,
    NotSupportedVisualFeature,
    StorageException,
    Timeout,
    Unspecified,
    UnsupportedMediaType,
}

/// Turn off specified domain models when generating the description.
#[derive(Debug, Deserialize, Serialize)]
pub enum DescriptionExclude {
    Celebrities,
    Landmarks,
}

/// A string indicating which domain-specific details to return.
///
/// Multiple values should be comma-separated.
///
/// Valid visual feature types include:
///
/// - `Celebrities`: identifies celebrities if detected in the image.
/// - `Landmarks`: identifies notable landmarks in the image.
#[derive(Debug, Deserialize, Serialize)]
pub enum Details {
    Celebrities,
    Landmarks,
}

/// A brand detected in an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct DetectedBrand {
    /// Confidence score of having observed the brand in the image, as a value
    /// ranging from 0 to 1.
    pub confidence: Number,

    /// Label for the brand.
    pub name: String,

    /// Approximate location of the detected brand.
    pub rectangle: BoundingRect,
}

/// An object detected in an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct DetectedObject {
    /// Confidence score of having observed the object in the image, as a value
    /// ranging from 0 to 1.
    pub confidence: Number,

    /// Label for the object.
    pub object: String,

    /// The parent object, from a taxonomy perspective. The parent object is a
    /// more generic form of this object. For example, a 'bulldog' would have a
    /// parent of 'dog'.
    pub parent: Option<ObjectHierarchy>, // m.b.o

    /// Approximate location of the detected object.
    pub rectangle: BoundingRect,
}

/// An object describing a face identified in the image.
#[derive(Debug, Deserialize, Serialize)]
pub struct FaceDescription {
    /// Possible age of the face.
    pub age: Option<u32>, // m.b.o

    /// Rectangle in the image containing the identified face.
    #[serde(rename = "faceRectangle")]
    pub face_rectangle: FaceRectangle,

    /// Possible gender of the face.
    pub gender: Option<Gender>, // m.b.o
}

/// An object describing face rectangle.
#[derive(Debug, Deserialize, Serialize)]
pub struct FaceRectangle {
    /// Height measured from the top-left point of the face, in pixels.
    pub height: PixelCount,

    /// X-coordinate of the top left point of the face, in pixels.
    pub left: PixelCount,

    /// Y-coordinate of the top left point of the face, in pixels.
    pub top: PixelCount,

    /// Width measured from the top-left point of the face, in pixels.
    pub width: PixelCount,
}

/// Possible gender of the face.
#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
}

/// Result of AnalyzeImage operation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageAnalysis {
    /// An object describing whether the image contains adult-oriented content
    /// and/or is racy.
    pub adult: Option<AdultInfo>,

    /// Array of brands detected in the image.
    pub brands: Option<Vec<DetectedBrand>>,

    /// An array indicating identified categories.
    pub categories: Option<Vec<Category>>,

    /// An object providing additional metadata describing color attributes.
    pub color: Option<ColorInfo>,

    /// A collection of content tags, along with a list of captions sorted by
    /// confidence level, and image metadata.
    pub description: Option<ImageDescriptionDetails>,

    /// An array of possible faces within the image.
    pub faces: Option<Vec<FaceDescription>>,

    /// An object providing possible image types and matching confidence levels.
    #[serde(rename = "imageType")]
    pub image_type: Option<ImageType>,

    /// Image metadata.
    pub metadata: ImageMetadata,

    /// Version of the AI model.
    #[serde(rename = "modelVersion")]
    pub model_version: String,

    /// Array of objects describing what was detected in the image.
    pub objects: Option<Vec<DetectedObject>>,

    /// Id of the REST API request.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// A list of tags with confidence level.
    pub tags: Option<Vec<ImageTag>>,
}

/// An image caption, i.e. a brief description of what the image depicts.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageCaption {
    /// The level of confidence the service has in the caption.
    pub confidence: Number,

    /// The text of the caption.
    pub text: String,
}

/// A collection of content tags, along with a list of captions sorted by
/// confidence level, and image metadata.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageDescriptionDetails {
    /// A list of captions, sorted by confidence level.
    pub captions: Vec<ImageCaption>,

    /// A collection of image tags.
    pub tags: Vec<String>,
}

/// Image metadata.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageMetadata {
    /// Image format.
    pub format: String,

    /// Image height, in pixels.
    pub height: PixelCount,

    /// Image width, in pixels.
    pub width: PixelCount,
}

/// An entity observation in the image, along with the confidence score.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageTag {
    /// The level of confidence that the entity was observed.
    pub confidence: Number,

    /// Optional hint/details for this tag.
    pub hint: Option<String>,

    /// Name of the entity.
    pub name: String,
}

/// An object providing possible image types and matching confidence levels.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageType {
    /// Confidence level that the image is a clip art.
    ///
    /// Possible values:
    ///
    /// - `0`: none
    /// - `1`: ambiguous
    /// - `2`: normal
    /// - `3`: good
    #[serde(rename = "clipArtType")]
    pub clipart_type: u8,

    /// Confidence level that the image is a line drawing.
    ///
    /// Possible values: `0` (none), `1`.
    #[serde(rename = "lineDrawingType")]
    pub line_drawing_type: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageUrl {
    /// Publicly reachable URL of an image.
    pub url: String,
}

/// A landmark recognized in the image.
#[derive(Debug, Deserialize, Serialize)]
pub struct LandmarksModel {
    /// Confidence level for the landmark recognition as a value ranging from 0
    /// to 1.
    pub confidence: Number,

    /// Name of the landmark.
    pub name: String,
}

/// An object detected inside an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectHierarchy {
    /// Confidence score of having observed the object in the image, as a value
    /// ranging from 0 to 1.
    pub confidence: Number,

    /// Label for the object.
    pub object: String,

    /// The parent object, from a taxonomy perspective. The parent object is a
    /// more generic form of this object. For example, a 'bulldog' would have a
    /// parent of 'dog'.
    pub parent: Option<Box<ObjectHierarchy>>, // m.b.o
}

/// A string indicating what visual feature types to return.
///
/// Multiple values should be comma-separated.
///
/// Valid visual feature types include:
///
/// - `Categories`: Categorizes image content according to a taxonomy defined
///   in documentation.
/// - `Tags`: tags the image with a detailed list of words related to the
///   image content.
/// - `Description`: Describes the image content with a complete English
///   sentence.
/// - `Faces`: Detects if faces are present. If present, generate coordinates,
///   gender and age.
/// - `ImageType`: Detects if image is clipart or a line drawing.
/// - `Color`: Determines the accent color, dominant color, and whether an
///   image is black & white.
/// - `Adult`: Detects if the image is pornographic in nature (depicts nudity
///   or a sex act), or is gory (depicts extreme violence or blood). Sexually
///   suggestive content (aka racy content) is also detected.
/// - `Objects`: Detects various objects within an image, including the
///   approximate location. The `Objects` argument is only available in English.
/// - `Brands`: Detects various brands within an image, including the
///   approximate location. The `Brands` argument is only available in English.
#[derive(Debug, Deserialize, Serialize)]
pub enum VisualFeatureTypes {
    Adult,
    Brands,
    Categories,
    Color,
    Description,
    Faces,
    ImageType,
    Objects,
    Tags,
}
