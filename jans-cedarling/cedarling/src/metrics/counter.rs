// This software is available under the Apache-2.0 license.
// See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
//
// Copyright (c) 2024, Gluu, Inc.

use serde::Serialize;

use super::Metric;

/// A cumulative metric that represents a monotonically increasing value which
/// can only be increased or be reset to zero on restart.
#[derive(Default)]
pub struct Counter(u64);

impl Counter {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn set(&mut self, value: u64) {
        self.0 = self.0.max(value);
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

impl Metric for Counter {
    type T = u64;

    /// Returns the total count
    fn value(&self) -> Self::T {
        self.0
    }
}

impl Serialize for Counter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.value())
    }
}
