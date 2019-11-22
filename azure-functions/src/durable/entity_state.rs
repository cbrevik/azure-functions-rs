use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityId {
    name: String,
    key: String,
}

#[doc(hidden)]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperationResult {
    is_error: bool,
    duration: f64,
    result: Option<String>,
}

#[doc(hidden)]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Signal {
    target: EntityId,
    name: String,
    input: String,
}

#[doc(hidden)]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityState {
    #[serde(rename = "entityExists")]
    exists: bool,
    #[serde(rename = "entityState")]
    state: Option<String>,
    results: Vec<OperationResult>,
    signals: Vec<Signal>,
}

impl EntityState {
    pub(crate) fn new(exists: bool, state: Option<String>) -> Self {
        Self {
            exists,
            state,
            ..Default::default()
        }
    }
}
