use super::{
    super::cedar_schema::CedarSchemaJson, trusted_issuer_metadata::TrustedIssuerMetadata,
    CedarSchema,
};
use base64::prelude::*;
use cedar_policy::{Policy, PolicyId, Schema};
use semver::Version;
use serde::{de, Deserialize};
use std::{collections::HashMap, str::FromStr};

// Policy Stores from the Agama Policy Designer
#[derive(Debug)]
#[allow(dead_code)]
pub struct AgamaPolicyStore {
    pub name: String,
    pub description: Option<String>,
    pub cedar_version: Option<Version>,
    pub policies: HashMap<String, AgamaPolicyContent>,
    pub cedar_schema: CedarSchema,
    pub trusted_issuers: HashMap<String, TrustedIssuerMetadata>,
}

impl PartialEq for AgamaPolicyStore {
    fn eq(&self, other: &Self) -> bool {
        // We need to implement this custom check since cedar_policy::Schema
        // does not implement PartialEq
        //
        // TODO: update this if ever cedar policy implements PartialEq
        // on cedar_policy::Schema since this is too difficult to check
        // right now... comparing the debug strings doesn't work either.
        self.name == other.name
            && self.description == other.description
            && self.cedar_version == other.cedar_version
            && self.policies == other.policies
            && self.trusted_issuers == other.trusted_issuers
            && self.cedar_schema.json == other.cedar_schema.json
    }
}

// Policy Store from the Agama Policy Designer
#[derive(Debug, PartialEq)]
pub struct AgamaPolicyContent {
    pub description: String,
    pub creation_date: String,
    pub policy_content: Policy,
}

#[derive(Deserialize)]
struct RawAgamaPolicyStores {
    cedar_version: Option<String>,
    policy_stores: HashMap<String, RawAgamaPolicyStore>,
}

#[derive(Deserialize)]
struct RawAgamaPolicyStore {
    pub name: String,
    pub description: Option<String>,
    policies: HashMap<String, RawAgamaCedarPolicy>,
    /// Base64 encoded JSON Cedar Schema
    #[serde(rename = "schema")]
    encoded_schema: String,
    trusted_issuers: HashMap<String, TrustedIssuerMetadata>,
}

#[derive(Deserialize)]
struct RawAgamaCedarPolicy {
    description: String,
    creation_date: String,
    /// Base64 encoded JSON Cedar Policy
    #[serde(rename = "policy_content")]
    encoded_policy: String,
}

fn decode_b64_string<'de, D>(value: String) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let buf = BASE64_STANDARD
        .decode(value)
        .map_err(|e| de::Error::custom(format!("Failed to decode Base64 encoded string: {}", e)))?;
    String::from_utf8(buf)
        .map_err(|e| de::Error::custom(format!("Failed to decode Base64 encoded string: {}", e)))
}

impl<'de> Deserialize<'de> for AgamaPolicyStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let raw_policy_stores = RawAgamaPolicyStores::deserialize(deserializer)?;
        let mut policies = HashMap::new();

        // We use a loop here to get the first item in the HashMap.
        for (_policy_store_id, policy_store) in raw_policy_stores.policy_stores {
            for (policy_id, policy) in policy_store.policies {
                // parse the decoded policy string into a cedar_policy::Policy struct
                let decoded_policy = decode_b64_string::<D>(policy.encoded_policy)?;
                let cedar_policy = Policy::parse(Some(PolicyId::new(&policy_id)), decoded_policy)
                    .map_err(de::Error::custom)?;

                let agama_policy = AgamaPolicyContent {
                    description: policy.description,
                    creation_date: policy.creation_date,
                    policy_content: cedar_policy,
                };

                policies.insert(policy_id, agama_policy);
            }

            let name = policy_store.name;
            let description = policy_store.description.filter(|v| v != "");

            // Convert String to Semver
            let cedar_version = match raw_policy_stores.cedar_version {
                Some(version) => {
                    let version = version.strip_prefix('v').unwrap_or(&version);
                    Some(Version::from_str(version).map_err(de::Error::custom)?)
                },
                None => None,
            };

            let decoded_schema = decode_b64_string::<D>(policy_store.encoded_schema)?;
            let schema = Schema::from_json_str(&decoded_schema).map_err(de::Error::custom)?;
            let json = serde_json::from_str::<CedarSchemaJson>(&decoded_schema)
                .map_err(de::Error::custom)?;
            let cedar_schema = CedarSchema { schema, json };

            // We return early since should only be getting one policy
            // store from Agama and Cedarling only supports using
            // one policy store at a time.
            return Ok(AgamaPolicyStore {
                name,
                description,
                cedar_version,
                policies,
                cedar_schema,
                trusted_issuers: policy_store.trusted_issuers,
            });
        }

        return Err(de::Error::custom(
            "Failed to deserialize Agama Policy Store: No policy store found in the `policies` field.",
        ));
    }
}

#[cfg(test)]
mod test {
    use super::super::super::{cedar_schema::CedarSchemaJson, policy_store::CedarSchema};
    use super::{AgamaPolicyContent, AgamaPolicyStore};
    use cedar_policy::{Policy, PolicyId, Schema};
    use semver::Version;
    use std::collections::HashMap;

    #[test]
    fn can_parse_agama_policy_store() {
        let policy_store_json = include_str!("./test_agama_policy_store.json");

        let parsed = serde_json::from_str::<AgamaPolicyStore>(policy_store_json)
            .expect("should parse Agama Policy Store");

        // Create Expected policies
        let mut policies = HashMap::new();
        let policy_id = "fbd921a51b8b78b3b8af5f93e94fbdc57f3e2238b29f".to_string();
        policies.insert(
            policy_id.clone(),
            AgamaPolicyContent {
                description: "Admin".to_string(),
                creation_date: "2024-11-07T07:49:11.813002".to_string(),
                policy_content: Policy::parse(
                    Some(PolicyId::new(policy_id)),
                    r#"@id("Admin")
permit
(
 principal == somecompany::store::Role::"Admin",
 action in [somecompany::store::Action::"DELETE",somecompany::store::Action::"GET",somecompany::store::Action::"PUT"],
 resource == somecompany::store::HTTP_Request::"root"
)
;"#.to_string(),
                )
                .expect("should parse cedar policy"),
            },
        );
        let policy_id = "1a2dd16865cf220ea9807608c6648a457bdf4057c4a4".to_string();
        policies.insert(
            policy_id.clone(),
            AgamaPolicyContent {
                description: "Member".to_string(),
                creation_date: "2024-11-07T07:50:05.520757".to_string(),
                policy_content: Policy::parse(
                    Some(PolicyId::new(policy_id)),
                    r#"@id("Member")
permit
(
 principal == somecompany::store::Role::"Member",
 action in [somecompany::store::Action::"PUT"],
 resource == somecompany::store::HTTP_Request::"root"
)
;"#
                    .to_string(),
                )
                .expect("should parse cedar policy"),
            },
        );

        let schema_json = include_str!("./test_agama_cedar_schema.json");
        let schema =
            Schema::from_json_str(schema_json).expect("Should parse Cedar schema from JSON");
        let json = serde_json::from_str::<CedarSchemaJson>(schema_json)
            .expect("Should parse cedar schema JSON");
        let cedar_schema = CedarSchema { schema, json };

        // No need to test parsing trusted_issuers here since we
        // already have tests in trusted_issuer_metadata.rs
        let trusted_issuers = HashMap::new();

        let expected = AgamaPolicyStore {
            name: "jans::store".to_string(),
            description: None,
            cedar_version: Some(Version::new(4, 0, 0)),
            policies,
            cedar_schema,
            trusted_issuers,
        };

        // We split the asserts into multiple steps since it's
        // difficult to read the error message for a struct this big
        // whenever the assertion fails.

        assert_eq!(
            parsed.name, expected.name,
            "Expected name to be parsed correctly: {:?}",
            parsed.name
        );

        assert_eq!(
            parsed.description, expected.description,
            "Expected description to be parsed correctly: {:?}",
            parsed.description
        );

        assert_eq!(
            parsed.cedar_version, expected.cedar_version,
            "Expected Cedar Version to be parsed correctly: {:?}",
            parsed.cedar_version
        );

        for (policy_store_id, policy_store) in &expected.policies {
            let parsed_policy_store = parsed
                .policies
                .get(&policy_store_id.clone())
                .expect("Expected to find a policy store with the same id.");

            assert_eq!(parsed_policy_store.description, policy_store.description);
            assert_eq!(
                parsed_policy_store.creation_date,
                policy_store.creation_date
            );
            assert_eq!(
                parsed_policy_store.policy_content,
                policy_store.policy_content
            );
        }

        for (trusted_issuer_id, trusted_issuer) in &expected.trusted_issuers {
            let parsed_trusted_issuer = parsed
                .trusted_issuers
                .get(&trusted_issuer_id.clone())
                .expect("Expected to find a trusted issuer with the same id.");

            assert_eq!(parsed_trusted_issuer, trusted_issuer);
        }
    }
}
