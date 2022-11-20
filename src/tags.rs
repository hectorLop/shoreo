use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tags {
    tags: HashMap<String, TagItem>
}

#[derive(Deserialize, Serialize, Debug)]
struct TagItem {
    name: String,
    artifact: TagArtifact,
}

#[derive(Deserialize, Serialize, Debug)]
struct TagArtifact {
    artifact_name: String,
    version: String,
    filepath: String,
}
