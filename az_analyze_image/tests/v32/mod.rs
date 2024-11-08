/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

mod error;
mod test;

pub use crate::util::*;
use az_analyze_image::v32::client::Client;

pub fn get_client() -> Client {
    let key = std::env::var("CV_KEY").expect(MSG_NO_ENV);
    let endpoint = std::env::var("CV_ENDPOINT").expect(MSG_NO_ENV);

    Client::new(key, &endpoint).unwrap()
}
