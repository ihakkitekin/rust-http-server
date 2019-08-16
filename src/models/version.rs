use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FragmentVersion {
    pub version: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub patch: String,
    pub minor: String,
    pub major: String,
}

impl Version {
    pub fn new() -> Self {
        Version {
            major: String::from("0"),
            minor: String::from("0"),
            patch: String::from("0"),
        }
    }

    pub fn from<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        let version = value.into();
        let parts: Vec<&str> = version.split(".").collect();

        Version {
            major: parts.get(0).unwrap_or_else(|| &"0").to_string(),
            minor: parts.get(1).unwrap_or_else(|| &"0").to_string(),
            patch: parts.get(2).unwrap_or_else(|| &"0").to_string(),
        }
    }

    pub fn major<T>(&mut self, value: T) -> &mut Version
    where
        T: Into<String>,
    {
        self.major = value.into();
        self
    }

    pub fn minor<T>(&mut self, value: T) -> &mut Version
    where
        T: Into<String>,
    {
        self.minor = value.into();
        self
    }

    pub fn patch<T>(&mut self, value: T) -> &mut Version
    where
        T: Into<String>,
    {
        self.patch = value.into();
        self
    }

    pub fn to_string(&self) -> String {
        let versions: [&str; 3] = [&self.major, &self.minor, &self.patch];

        versions.join(".")
    }
}
