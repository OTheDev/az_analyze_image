/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

//! Provides a client for the Azure AI Services [Analyze Image API Version 4.0](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP)
//! (`2023-04-01-preview`).
//!
//! This module provides types that map to the official [API definitions](https://learn.microsoft.com/en-us/rest/api/computervision/image-analysis/analyze-image?view=rest-computervision-v4.0-preview%20(2023-04-01)&tabs=HTTP#definitions)
//! and a [`client::Client`].

mod api;
pub mod client;

pub use api::*;

use super::common::*;
