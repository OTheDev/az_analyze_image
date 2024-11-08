/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0 OR MIT
*/

use serde::{Deserialize, Serialize};
use std::fmt;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

const SECRET_DEBUG_OUTPUT: &str = "Secret { * }";

/// Represents a secret value.
///
/// - The `Debug` trait implementation ensures that the secret is not exposed
///     when formatted using `{:?}` or `{:#?}`, instead displaying
///     "Secret { * }".
///
/// - The `PartialEq` trait is implemented using constant-time equality.
///
/// - The `Drop` trait implementation uses the [`zeroize`] crate to zero the
///     secret value's memory when a `Secret` instance is dropped.
#[derive(Clone, Eq, Deserialize, Serialize)]
pub struct Secret {
    value: String,
}

impl Secret {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SECRET_DEBUG_OUTPUT)
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_bytes().ct_eq(other.value.as_bytes()).into()
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALUE: &str = "f73ff2c2addc4ab7b2480278c8c6ff90";

    #[test]
    fn test_secret_new() {
        let _secret = Secret::new(TEST_VALUE.to_string());
    }

    #[test]
    fn test_secret_get_value() {
        let secret = Secret::new(TEST_VALUE.to_string());
        assert_eq!(secret.value(), TEST_VALUE);
    }

    #[test]
    fn test_secret_debug() {
        let secret = Secret::new(TEST_VALUE.to_string());

        assert_eq!(format!("{secret:?}"), SECRET_DEBUG_OUTPUT);
        assert_eq!(format!("{secret:#?}"), SECRET_DEBUG_OUTPUT);
    }

    #[test]
    fn test_secret_from_string() {
        let _secret: Secret = TEST_VALUE.to_string().into();
    }

    #[test]
    fn test_secret_partial_equality() {
        let secret_1 = Secret::new(TEST_VALUE.to_string());
        let secret_2 = Secret::new(TEST_VALUE.to_string());
        let secret_3 = Secret::new("different_value".to_string());

        assert_eq!(secret_1, secret_2);
        assert_ne!(secret_1, secret_3);
    }
}
