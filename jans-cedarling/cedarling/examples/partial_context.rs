use cedar_policy::*;

const SCHEMA : &str = r#"
namespace Jans {
  type Url = {"host": String, "path": String, "protocol": String};

  entity Access_token = {"aud": String, "exp": Long, "iat": Long, "iss": TrustedIssuer, "jti": String};
  entity Issue = {"country": String, "org_id": String};
  entity Role;
  entity Tagged {value: String} tags Set<Tagged>;
  entity TrustedIssuer = {"issuer_entity_id": Url};
  entity User in [Role] = {"country": String, "email": String, "sub": String, "username": String} tags Set<String>;
  entity Workload = {"client_id": String, "iss": TrustedIssuer, "name": String, "org_id": String};
  entity id_token = {"acr": String, "amr": String, "aud": String, "exp": Long, "iat": Long, "iss": TrustedIssuer, "jti": String, "sub": String};
  entity Userinfo_token  = {"iss": String, "jti": String, "client_id": String};
  action "Update" appliesTo {
    principal: [Workload, User, Userinfo_token, Role],
    resource: [Issue],
    context: {"thingz": String, properties?: Set<String>}
  };
}
"#;

#[allow(dead_code)]
const TAGS: &str = r#"
// these are from cedar-policy-validator/typecheck/test/tags.rs
entity E tags String;
entity F { foo: String, opt?: String } tags Set<String>;

// from cedar-policy-validator/src/schema.rs
entity E;
type Blah = {
    foo: Long,
    bar: Set<E>,
};
entity Foo1 in E {
    bool: Bool,
} tags Bool;
entity Foo2 in E {
    bool: Bool,
} tags { bool: Bool };
entity Foo3 in E tags E;
entity Foo4 in E tags Set<E>;
entity Foo5 in E tags { a: String, b: Long };
entity Foo6 in E tags Blah;
entity Foo7 in E tags Set<Set<{a: Blah}>>;
entity Foo8 in E tags Foo7;
"#;

const POLICY : &str = r#"
@desc("Resource 1 Update Rule")
permit(
    principal is Jans::User,
    action == Jans::Action::"Update",
    resource == Jans::Issue::"Resource1"
) when {
    principal has country && principal.country == "Easter Island" &&
    // This is only possible where the Request is not validated against the schema.
    // context has permissions && context.permissions.Resource1.contains("User") &&
    context.thingz == "yes" &&
    context has properties && context.properties.contains("Guest") &&
    // principal.hasTag("pet") && principal.getTag("pet").contains("cat") &&
    true
};
"#;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let policy: PolicySet = POLICY.parse()?;
  let schema : Schema = SCHEMA.parse()?;

  // Validate the policies against the schema.
  // Not that Validator consumes schema, so clone is required.
  // let validator = Validator::new(schema.clone());
  // let validation_result = validator.validate(&policy, ValidationMode::Strict);
  // if !validation_result.validation_passed() {
  //   println!("validation_result: {:#?}", validation_result);
  // }

  let principal = {
    let principal_id : cedar_policy::EntityUid = r#"Jans::User::"Drofio""#.parse()?;
    let attrs = [
      ("country", "Easter Island"),
      ("email", "david@easter.is.land"),
      ("sub", "Issue Resource"),
      ("username", "drofio"),
    ];
    use std::collections::HashMap;
    let attrs : HashMap<String, RestrictedExpression> = attrs.iter()
      .map(|(k,v)| ((*k).into(), RestrictedExpression::new_string((*v).into())) )
      .collect();
    cedar_policy::Entity::new(principal_id, attrs, std::collections::HashSet::new())?
  };

  // So the only way to get tags into an Entity is to use json or json::Value
    // uid: EntityUidJson,
    // attrs: HashMap<SmolStr, JsonValueWithNoDuplicateKeys>,
    // parents: Vec<EntityUidJson>,
    // tags: HashMap<SmolStr, JsonValueWithNoDuplicateKeys>,
  let json_principal = Entity::from_json_value(serde_json::json!({
    "uid": { "type": "Jans::User", "id": "Drofio" },
    "attrs": {
      "country": "Easter Island",
      "email": "david@easter.is.land",
      "sub": "Issue Resource",
      "username": "drofio",
    },
    "parents": [],
    "tags": {
      "pet": ["djou van de be", "cat"],
      "wut": ["this is a world of value"],
      "tingz": [],
      // "not-a-set-of-string": "This is a plain String", this would fail because User tags is Set<String>
    },
  }), Some(&schema) )?;

  // required for context validation
  let update_action : cedar_policy::EntityUid = r#"Jans::Action::"Update""#.parse()?;

  // Context not validated at all.
  // This succeeds here, but will fail when a Request is created from it.
  let unvalidated_context = {
    Context::from_json_value(serde_json::json!({
      "permissions": {
        "Resource1": ["User"]
      },
      "thingz": "yes",
      "properties": ["Guest"]
    }), None)?
  };

  // partially validated with schema
  let partially_validated_context = {
    let context = Context::empty();

    // Creation of an unvalidated context part like this is ok at this point,
    // But, when a schema-validated Request is created, that will fail with an error:
    //   InvalidContextError(InvalidContextError { context: Value({"permissions": Value { value: Record({"Resource1
    let perm_context = Context::from_json_value(serde_json::json!({
      "permissions": {
        "Resource1": ["User"]
      }
    }), None)?;
    let context = context.merge(perm_context)?;

    let props_context = Context::from_json_value(serde_json::json!({
      "thingz": "yes",
      "properties": ["Guest"] // NOTE that properties is an optional value - properties?: Set<String>
    }), Some((&schema, &update_action)) )?;
    let context = context.merge(props_context)?;

    context
  };

  // fully validated with schema
  let validated_context = {
    let context = Context::empty();

    let props_context = Context::from_json_value(serde_json::json!({
      "thingz": "yes",
      "properties": ["Guest"] // NOTE that properties is an optional value - properties?: Set<String>
    }), Some((&schema, &update_action)) )?;
    let context = context.merge(props_context)?;

    context
  };

  let request = Request::new(
    r#"Jans::User::"Drofio""#.parse()?,
    update_action,
    r#"Jans::Issue::"Resource1""#.parse()?,
    // unvalidated_context,
    // partially_validated_context,
    validated_context,
    Some(&schema)
    // None
  );

  let request = match request {
    Ok(req) => req,
    Err(err) => {
      println!("{:#?}", err);
      return Err(err)?
    },
  };

  let entities = Entities::empty().add_entities(std::iter::once(json_principal), Some(&schema))?;

  let answer = Authorizer::new().is_authorized(&request, &policy, &entities);

  match answer.decision() {
    Decision::Allow => println!("--\nAllow\n--"),
    Decision::Deny => {
      // NOTE at this point, can validate the schema to see if that was the problem.
      // Because certain kinds of errors in the policy result in no diagnositcs for the Deny message.
      // eg misnaming an entity in the policy, eg calling it `Jans::NonIssue` instead of `Jans::Issue`
      println!("--\nDeny\n--\n{:#?}", answer.diagnostics())
    }
  };

  Ok(())
}
