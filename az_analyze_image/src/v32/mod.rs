/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

//! Provides a client for the Azure AI Services [Analyze Image API Version 3.2](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2&tabs=HTTP).
//!
//! This module provides types that map to the official [API definitions](https://learn.microsoft.com/en-us/rest/api/computervision/analyze-image/analyze-image?view=rest-computervision-v3.2&tabs=HTTP#definitions)
//! and a [`client::Client`].

mod api;
pub mod client;

pub use api::*;

use super::common::*;
