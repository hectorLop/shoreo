use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tags {
    pub tags: HashMap<String, TagItem>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagItem {
    pub artifact_name: String,
    pub version: String,
    pub filepath: String,
}

//#[derive(Deserialize, Serialize, Debug)]
//struct TagArtifact {
//    artifact_name: String,
//    version: String,
//    filepath: String,
//}
