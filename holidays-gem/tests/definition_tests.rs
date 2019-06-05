#[test]
fn definition_tests() {
    let definitions_dir: std::path::PathBuf = "./definitions".into();
    let index_file = std::fs::File::open(definitions_dir.join("index.yaml")).unwrap();
    let index: holidays_gem::Index = serde_yaml::from_reader(index_file).unwrap();
    let paths: Vec<std::path::PathBuf> = index.defs.values().flat_map(|v| v.to_owned()).collect();

    unimplemented!();
}
