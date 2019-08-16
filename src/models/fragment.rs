use crate::models::version::FragmentVersion;
use crate::traits::db_item::DbItem;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragmentGet {
    pub values: Vec<FragmentGetValues>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragmentGetValues {
    pub display_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragmentClient {
    pub name: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragmentDb {
    pub name: String,
    pub version: String,
    pub app: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {
    pub name: String,
    pub versions: Vec<FragmentVersion>,
}

impl<'a> DbItem<'a> for FragmentDb {
    const PATH: &'a str = "./src/data/fragments.json";
}
