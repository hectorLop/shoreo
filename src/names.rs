use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Names {
    names_artifacts: HashMap<String, NameArtifact>
}

#[derive(Deserialize, Serialize, Debug)]
struct NameArtifact {
    name: String,
    versions: HashMap<String, Version>
}

#[derive(Deserialize, Serialize, Debug)]
struct Version {
    version: String,
    filepath: String,
    tags: Vec<String>,
}

