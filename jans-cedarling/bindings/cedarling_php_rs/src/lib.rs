#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use cedarling::{
    BootstrapConfig, Cedarling as RustCedarling, JwtConfig, LogConfig, LogTypeConfig, PolicyStoreConfig,
    PolicyStoreSource, Request, ResourceData,
};
use std::collections::HashMap;

static POLICY_STORE_RAW: &str = include_str!("policy-store_ok.json");

#[php_class]
pub struct Cedarling {
    cedarling: RustCedarling, // Wrap the Rust Cedarling instance
}

#[php_impl]
impl Cedarling {
    // Define the __construct method that PHP can use to instantiate the object
    #[php_method]
    pub fn __construct() -> PhpResult<Self> {
        // Initialize the Cedarling instance with the BootstrapConfig
        let cedarling = RustCedarling::new(BootstrapConfig {
            application_name: "test_app".to_string(),
            log_config: LogConfig {
                log_type: LogTypeConfig::StdOut,
            },
            policy_store_config: PolicyStoreConfig {
                source: PolicyStoreSource::Json(POLICY_STORE_RAW.to_string()),
            },
            jwt_config: JwtConfig::Disabled,
        }).map_err(|e| format!("Failed to initialize Cedarling: {:?}", e))?;

        Ok(Cedarling { cedarling })
    }

    // PHP-exposed authorization method
    pub fn authz(
        &mut self,
        access_token: &str,
        id_token: &str,
        org_id: &str,
        userinfo_token: &str,
    ) -> PhpResult<String> {
   /* let userinfo_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjb3VudHJ5IjoiVVMiLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20iLCJ1c2VybmFtZSI6IlVzZXJOYW1lRXhhbXBsZSIsInN1YiI6ImJvRzhkZmM1TUtUbjM3bzdnc2RDZXlxTDhMcFdRdGdvTzQxbTFLWndkcTAiLCJpc3MiOiJodHRwczovL2FkbWluLXVpLXRlc3QuZ2x1dS5vcmciLCJnaXZlbl9uYW1lIjoiQWRtaW4iLCJtaWRkbGVfbmFtZSI6IkFkbWluIiwiaW51bSI6IjhkMWNkZTZhLTE0NDctNDc2Ni1iM2M4LTE2NjYzZTEzYjQ1OCIsImNsaWVudF9pZCI6IjViNDQ4N2M0LThkYjEtNDA5ZC1hNjUzLWY5MDdiODA5NDAzOSIsImF1ZCI6IjViNDQ4N2M0LThkYjEtNDA5ZC1hNjUzLWY5MDdiODA5NDAzOSIsInVwZGF0ZWRfYXQiOjE3MjQ3Nzg1OTEsIm5hbWUiOiJEZWZhdWx0IEFkbWluIFVzZXIiLCJuaWNrbmFtZSI6IkFkbWluIiwiZmFtaWx5X25hbWUiOiJVc2VyIiwianRpIjoiZmFpWXZhWUlUMGNEQVQ3Rm93MHBRdyIsImphbnNBZG1pblVJUm9sZSI6WyJhcGktYWRtaW4iXSwiZXhwIjoxNzI0OTQ1OTc4fQ.3LTc8YLvEeb7ONZp_FKA7yPP7S6e_VTzwhvAWUJrL4M".to_string();
    */
    // usually ResourceData will be parse from json
    let resource_json = serde_json::json!({
            "id": "random_id",
            "type": "Jans::Issue",
            "org_id": "some_long_id",
            "country": "US",
    });
    
        // Perform the authorization logic
        let result = self.cedarling.authorize(Request {
            access_token: access_token.to_string(),
            id_token: id_token.to_string(),
            userinfo_token: userinfo_token.to_string(),
            action: "Jans::Action::\"Update\"".to_string(),
            context: serde_json::json!({}),
            resource: ResourceData {
                id: "random_id".to_string(),
                resource_type: "Jans::Issue".to_string(),
                payload: serde_json::from_value(resource_json)
                .map_err(|err| format!("could not parse ResourceData:{err}"))?,
            },
        });

        // Return the result of authorization to PHP
        match result {
            Ok(auth_result) => Ok(format!("Authorization success: {}", auth_result.is_allowed())),
            Err(e) => Err(format!("Authorization failed: {:?}", e).into()),
        }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

