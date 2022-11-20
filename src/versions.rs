use crate::tags::Tags;
use crate::names::Names;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct VersionData {
    tags_data: Tags,
    names_data: Names,
}

