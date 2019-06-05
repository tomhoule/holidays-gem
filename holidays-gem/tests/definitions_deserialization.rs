#[test]
fn definitions_deserialization() {
    let definitions_dir: std::path::PathBuf = "./definitions".into();
    let index_file = std::fs::File::open(definitions_dir.join("index.yaml")).unwrap();
    let index: holidays_gem::Index = serde_yaml::from_reader(index_file).unwrap();
    let paths: Vec<std::path::PathBuf> = index.defs.values().flat_map(|v| v.to_owned()).collect();

    // Sanity check: all the files are here.
    assert_eq!(paths.len(), 114);

    for path in paths {
        println!("file: {}", path.to_string_lossy());
        let file = std::fs::File::open(definitions_dir.join(path)).unwrap();

        let _: holidays_gem::CountryFile = serde_yaml::from_reader(file).unwrap();
    }
}
