use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Names {
    pub names: HashMap<String, NameArtifact>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameArtifact {
    pub name: String,
    pub versions: HashMap<String, Version>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Version {
    pub version: String,
    pub filepath: String,
    pub tags: Vec<String>,
}

