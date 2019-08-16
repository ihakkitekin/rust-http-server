use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub fragments: Vec<String>,
}
