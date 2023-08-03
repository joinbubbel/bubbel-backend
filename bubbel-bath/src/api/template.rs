use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct Foo {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct FooOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum FooError {
    Internal { ierror: String },
}

pub fn foo(
    //  db: &mut DataState,
    //  auth: &AuthState,
    req: Foo,
) -> Result<FooOut, FooError> {
    Ok(FooOut {})
}
