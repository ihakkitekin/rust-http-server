use actix_web::client::Client;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use serde::Serialize;
use serde_urlencoded::to_string;
use std::time::Duration;

use crate::mappers::fragments_mapper::map_fragments_list;
use crate::models::fragment::FragmentGet;
use std::fs::File;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Params {
    filter_text: String,
}

pub fn get_fragments() -> HttpResponse {
    let json_file = File::open("./src/data/branches.json").expect("file not found");
    let fragments_list: FragmentGet =
        serde_json::from_reader(json_file).unwrap_or_else(|_| FragmentGet {
            values: Vec::default(),
        });

    HttpResponse::Ok().json(map_fragments_list(fragments_list))
}
