use crate::tags::Tags;
use crate::names::Names;
use serde_derive::{Deserialize, Serialize};

pub trait Data {}


#[derive(Deserialize, Serialize, Debug)]
pub struct VersionData {
    pub tags_data: Tags,
    pub names_data: Names,
}

#[derive(Deserialize, Serialize, Debug)]
enum FileType {
    TagFile(Tags),
    NameFile(Names),
}

pub fn read_versions_data(folder: String) -> VersionData {
    let tags_filepath = folder.clone() + "tags.json";
    let names_filepath = folder + "names.json";

    let tags_data = {
        let str_data = std::fs::read_to_string(&tags_filepath).unwrap();

        serde_json::from_str::<Tags>(&str_data).unwrap()
    };
    
    let names_data = {
        let str_data = std::fs::read_to_string(&names_filepath).unwrap();

        serde_json::from_str::<Names>(&str_data).unwrap()
    };

    VersionData {
        tags_data: tags_data,
        names_data: names_data,
    }
}
