// This software is available under the Apache-2.0 license.
// See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
//
// Copyright (c) 2024, Gluu, Inc.

use super::Metric;
use serde::Serialize;
use std::collections::VecDeque;

/// The number of samples used to calculate the average value in a gauge
const DEFAULT_SAMPLE_SIZE: usize = 100;

/// A metric that can go up and down, arbitrarily, over time.
pub struct Gauge {
    values: VecDeque<f32>,
    sum: f32,
    capacity: usize,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            values: VecDeque::with_capacity(DEFAULT_SAMPLE_SIZE),
            sum: 0.0,
            capacity: DEFAULT_SAMPLE_SIZE,
        }
    }
}

impl Gauge {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: VecDeque::with_capacity(capacity),
            sum: 0.0,
            capacity,
        }
    }

    pub fn set(&mut self, value: f32) {
        self.values.push_back(value);
        if self.values.len() > self.capacity {
            self.sum += value - self.values.pop_front().unwrap_or_default();
        } else {
            self.sum += value;
        }
    }
}

impl Metric for Gauge {
    type T = f32;

    /// Returns the average value over the given samples
    fn value(&self) -> Self::T {
        let divisor = self.capacity.min(self.values.len()) as f32;
        if divisor == 0.0 {
            self.sum
        } else {
            self.sum / divisor
        }
    }
}

impl Serialize for Gauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.value())
    }
}

/// A metric that tracks success rate
pub struct RateGauge {
    success: f32,
    fail: f32,
}

impl Default for RateGauge {
    fn default() -> Self {
        Self {
            success: 0.0,
            fail: 0.0,
        }
    }
}

impl RateGauge {
    pub fn increment(&mut self) {
        self.success += 1.0;
    }

    pub fn decrement(&mut self) {
        self.fail += 1.0;
    }
}

impl Metric for RateGauge {
    type T = f32;

    /// Returns the success rate
    fn value(&self) -> Self::T {
        let divisor = self.success + self.fail;
        if divisor == 0.0 {
            return 0.0;
        }
        self.success / divisor
    }
}

impl Serialize for RateGauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.value())
    }
}

#[cfg(test)]
mod test {
    use super::super::Metric;
    use super::{Gauge, RateGauge};

    #[test]
    pub fn test_gauge_rolling_value() {
        let mut gauge = Gauge::new(3);
        assert_eq!(gauge.value(), 0.0);
        gauge.set(1.0);
        assert_eq!(gauge.value(), 1.0);
        gauge.set(2.0);
        assert_eq!(gauge.value(), 1.5);
        gauge.set(3.0);
        assert_eq!(gauge.value(), 2.0);
        gauge.set(4.0);
        assert_eq!(gauge.value(), 3.0);
    }

    #[test]
    pub fn test_rate_gauge() {
        let mut gauge = RateGauge::default();
        assert_eq!(gauge.value(), 0.0);
        gauge.increment();
        assert_eq!(gauge.value(), 1.0);
        gauge.decrement();
        assert_eq!(gauge.value(), 0.5);
        gauge.increment();
        gauge.increment();
        gauge.increment();
        assert_eq!(gauge.value(), 0.8);
    }
}
