pub use shoreo::versions::read_versions_data as read;


#[cfg(test)]
mod tests {
    pub use shoreo::versions;

    #[test]
    fn test_read_versions_data() {
        let folder = "tests/".to_string();
        let data = versions::read_versions_data(folder);

        assert_eq!(data.tags_data.tags.get("training").unwrap().artifact_name, "artifact_1");
        assert_eq!(data.names_data.names.get("artifact_1").unwrap().versions.get("1").unwrap().version, "1");
    }
}
