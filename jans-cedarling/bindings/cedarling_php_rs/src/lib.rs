#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use cedarling::{
    BootstrapConfig, Cedarling as RustCedarling, JwtConfig, LogConfig, LogTypeConfig, PolicyStoreConfig, 
    PolicyStoreSource, Request, ResourceData,
};


#[php_class]
pub struct Cedarling {
    cedarling: RustCedarling, // Wrap the Rust Cedarling instance
}

#[php_impl]
impl Cedarling {
    // Define the __construct method that PHP can use to instantiate the object with dynamic parameters
    #[php_method]
    pub fn __construct(policy_store_raw: &str, application_name: &str, log_type_arg: &str) -> PhpResult<Self> {
        // Initialize the Cedarling instance with the BootstrapConfig using the provided parameters
        let log_type = match log_type_arg {
        "off" => LogTypeConfig::Off,
        "stdout" => LogTypeConfig::StdOut,
        "lock" => LogTypeConfig::Lock,
        _ => {
            eprintln!("Invalid log type, defaulting to StdOut.");
            LogTypeConfig::StdOut
        },
    };
        let cedarling = RustCedarling::new(BootstrapConfig {
            application_name: application_name.to_string(),
            log_config: LogConfig {
                log_type: log_type,
            },
            policy_store_config: PolicyStoreConfig {
                source: PolicyStoreSource::Json(policy_store_raw.to_string()),
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
        // Example of resource data
        let resource_json = serde_json::json!({
            "id": "random_id",
            "type": "Jans::Issue",
            "org_id": org_id,
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
                    .map_err(|err| format!("could not parse ResourceData: {err}"))?,
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

