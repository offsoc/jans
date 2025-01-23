// This software is available under the Apache-2.0 license.
// See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
//
// Copyright (c) 2024, Gluu, Inc.

use std::sync::Arc;

use super::LogStrategy;
use crate::common::app_types::{ApplicationName, PdpID};
use crate::log::LogType;
use crate::log::{LogEntry, interface::LogWriter};

pub struct Logger {
    log_service: Arc<LogStrategy>,
    pdp_id: PdpID,
    app_id: Option<ApplicationName>,
}

impl Logger {
    pub fn new(
        log_service: Arc<LogStrategy>,
        pdp_id: PdpID,
        app_id: Option<ApplicationName>,
    ) -> Self {
        Self {
            log_service,
            pdp_id,
            app_id,
        }
    }

    pub fn system_debug(&self, msg: impl ToString) {
        self.log_service.log(
            LogEntry::new_with_data(self.pdp_id, self.app_id.clone(), LogType::System)
                .set_level(crate::LogLevel::DEBUG)
                .set_message(msg.to_string()),
        );
    }
}
