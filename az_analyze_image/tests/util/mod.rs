/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

pub const MSG_NO_ENV: &'static str =
    "tests require that CV_KEY and CV_ENDPOINT are set";

pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

pub fn approx_eq_exp(a: f64, b: f64, x: u32) -> bool {
    let tolerance = 10f64.powi(-(x as i32));
    approx_eq(a, b, tolerance)
}

pub enum URL {
    Default,
    GreaterThan4MB,
    GreaterThan20MB,
    PersonWithHat,
    Quickstart,
    Macys,
    BigBen,
    ForbiddenCity,
}

impl Into<&'static str> for URL {
    fn into(self) -> &'static str {
        match self {
            Self::Default => "https://upload.wikimedia.org/wikipedia/commons/2/2a/Human_faces.jpg",
            Self::GreaterThan4MB => "https://images.unsplash.com/photo-1730386114785-9f44db925f46?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=perry-merrity-ii-V1-qLQc1SG0-unsplash.jpg",
            Self::Quickstart => "https://learn.microsoft.com/azure/ai-services/computer-vision/media/quickstarts/presentation.png",
            Self::PersonWithHat => "https://cdn.pixabay.com/photo/2015/04/27/04/42/people-741431_1280.jpg",
            Self::Macys => "https://cdn.pixabay.com/photo/2016/04/20/00/41/mcdonalds-1340199_1280.jpg",
            Self::BigBen => "https://cdn.pixabay.com/photo/2016/01/16/17/13/big-ben-1143631_1280.jpg",
            Self::ForbiddenCity => "https://cdn.pixabay.com/photo/2013/11/28/10/37/forbidden-city-220099_1280.jpg",
            Self::GreaterThan20MB => "https://svs.gsfc.nasa.gov/vis/a030000/a030800/a030877/frames/5760x3240_16x9_01p/BlackMarble_2016_928m_canada_s_labeled.png",
        }
    }
}
