use crate::traits::db_item::DbItem;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;

pub fn get<'a, T>() -> Vec<T>
where
    T: Serialize + DeserializeOwned + DbItem<'a>,
{
    let json_file = File::open(T::PATH).expect("file not found");
    let items: Vec<T> = serde_json::from_reader(json_file).expect("error while reading json");

    items
}
