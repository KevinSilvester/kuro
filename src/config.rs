use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FilePath {
    pub windows: Option<String>,
    pub unix: Option<String>,
    pub mac: Option<String>,
    pub linux: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Files {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigEntry {
    pub name: String,
    pub files: Option<Files>,
}
