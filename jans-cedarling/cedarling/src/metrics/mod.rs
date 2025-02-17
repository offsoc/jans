// This software is available under the Apache-2.0 license.
// See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
//
// Copyright (c) 2024, Gluu, Inc.

mod counter;
mod gauge;

use chrono::{DateTime, Utc};
use counter::Counter;
use gauge::{Gauge, RateGauge};
use serde::{Serialize, Serializer};
use std::collections::HashMap;

pub trait Metric: Serialize {
    type T;

    fn value(&self) -> Self::T;
}

#[derive(Serialize)]
pub struct Meter {
    #[serde(rename = "uptime_secs", serialize_with = "serialize_uptime")]
    started_at: DateTime<Utc>,
    #[serde(flatten)]
    counters: HashMap<MetricCounter, Counter>,
    #[serde(flatten)]
    gauges: HashMap<MetricGauge, Gauge>,
    #[serde(flatten)]
    rates: HashMap<MetricRate, RateGauge>,
}

impl Default for Meter {
    fn default() -> Self {
        let started_at = Utc::now();
        let counters = HashMap::from([
            (MetricCounter::TotalAuthzRequests, Counter::default()),
            (MetricCounter::TotalJwtsValidated, Counter::default()),
        ]);
        let gauges = HashMap::from([(MetricGauge::AvgDecisionMs, Gauge::default())]);
        let rates = HashMap::from([
            (MetricRate::AuthzAllowRate, RateGauge::default()),
            (MetricRate::ValidJwtRate, RateGauge::default()),
        ]);
        Self {
            started_at,
            counters,
            gauges,
            rates,
        }
    }
}

impl Meter {
    pub fn increment_counter(&mut self, metric: MetricCounter) {
        if let Some(counter) = self.counters.get_mut(&metric) {
            counter.increment();
        }
    }

    /// Sets the value of the given counter.
    ///
    /// Note that counters can only go up.
    pub fn set_counter(&mut self, metric: MetricCounter, value: u64) {
        if let Some(counter) = self.counters.get_mut(&metric) {
            counter.set(value);
        }
    }

    pub fn set_gauge(&mut self, metric: MetricGauge, value: f32) {
        if let Some(gauge) = self.gauges.get_mut(&metric) {
            gauge.set(value);
        }
    }

    pub fn increment_rate(&mut self, metric: MetricRate) {
        if let Some(counter) = self.rates.get_mut(&metric) {
            counter.increment();
        }
    }

    pub fn decrement_rate(&mut self, metric: MetricRate) {
        if let Some(counter) = self.rates.get_mut(&metric) {
            counter.decrement();
        }
    }
}

fn serialize_uptime<S: Serializer>(
    value: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let now = Utc::now();
    let uptime = now - value;
    serializer.serialize_i64(uptime.num_seconds())
}

#[derive(Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetricCounter {
    TotalAuthzRequests,
    TotalJwtsValidated,
}

#[derive(Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetricGauge {
    AvgDecisionMs,
}

#[derive(Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetricRate {
    AuthzAllowRate,
    ValidJwtRate,
}

#[cfg(test)]
mod test {
    use crate::metrics::{Meter, MetricGauge, MetricRate};

    use super::MetricCounter;
    use serde_json::json;

    #[test]
    fn test_meter() {
        let mut meter = Meter::default();
        meter.increment_counter(MetricCounter::TotalAuthzRequests);
        meter.set_counter(MetricCounter::TotalJwtsValidated, 3);
        meter.set_gauge(MetricGauge::AvgDecisionMs, 1.0);
        meter.set_gauge(MetricGauge::AvgDecisionMs, 1.0);
        meter.set_gauge(MetricGauge::AvgDecisionMs, 7.0);
        meter.increment_rate(MetricRate::ValidJwtRate);
        meter.increment_rate(MetricRate::ValidJwtRate);
        meter.increment_rate(MetricRate::ValidJwtRate);
        meter.decrement_rate(MetricRate::ValidJwtRate);
        meter.decrement_rate(MetricRate::AuthzAllowRate);
        meter.decrement_rate(MetricRate::AuthzAllowRate);

        let mut serialized_meter = serde_json::to_value(&meter).expect("should serialize meter");

        // we edit the uptime so we can assert the other fields
        // easily but we first check if it gets serialized before
        // editing
        assert!(serialized_meter.get("uptime_secs").is_some());
        serialized_meter["uptime_secs"] = json!(0);

        assert_eq!(
            serialized_meter,
            json!({
                "uptime_secs": 0,
                "total_authz_requests": 1,
                "total_jwts_validated": 3,
                "avg_decision_ms": 3.0,
                "valid_jwt_rate": 0.75,
                "authz_allow_rate": 0.0,
            })
        );
    }
}
