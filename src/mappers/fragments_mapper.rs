use crate::db::db;
use crate::models::application::Application;
use crate::models::fragment::{Fragment, FragmentClient, FragmentDb, FragmentGet};
use crate::models::version::{FragmentVersion, Version};
use itertools::Itertools;

#[derive(Debug)]
struct TempFragment<'a> {
    pub app: &'a str,
    pub fragment: &'a str,
}

pub fn map_fragments_list(response: FragmentGet) -> Vec<FragmentClient> {
    let applications = map_applications(response);
    let db_fragments = db::get::<FragmentDb>();

    let client_fragments: Vec<FragmentClient> = applications
        .iter()
        .map(|app| FragmentClient {
            name: app.name.to_owned(),
            fragments: app
                .fragments
                .iter()
                .map(|fragment_name| Fragment {
                    name: fragment_name.to_owned(),
                    versions: db_fragments
                        .iter()
                        .filter(|frag| {
                            frag.name == fragment_name.to_owned() && frag.app == app.name
                        })
                        .map(|frag| FragmentVersion {
                            url: frag.url.to_owned(),
                            version: frag.version.to_owned(),
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect();

    client_fragments
}

fn map_applications(response: FragmentGet) -> Vec<Application> {
    let groups = response
        .values
        .iter()
        .filter(|value| value.display_id != "fragments/default")
        .map(|value| {
            let parts: Vec<&str> = value.display_id.split("/").collect();

            TempFragment {
                app: parts.get(1).unwrap_or_else(|| &""),
                fragment: parts.get(2).unwrap_or_else(|| &""),
            }
        })
        .group_by(|val| val.app);

    groups
        .into_iter()
        .map(|(key, values)| Application {
            name: key.to_owned(),
            fragments: values
                .into_iter()
                .map(|value| value.fragment.to_owned())
                .collect(),
        })
        .collect()
}
