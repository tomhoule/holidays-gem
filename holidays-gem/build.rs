use failure::ResultExt;
use std::collections::HashMap;

fn main() -> Result<(), failure::Error> {
    let definitions_dir: std::path::PathBuf = "./definitions".into();
    let index_file = std::fs::File::open(definitions_dir.join("index.yaml"))?;
    let index: holidays_gem_definitions::Index = serde_yaml::from_reader(index_file)?;
    let paths: Vec<std::path::PathBuf> = index
        .defs
        .values()
        .flat_map(|v| v.to_owned())
        .map(|v| std::path::Path::new("./definitions").join(v))
        .collect();

    let mut definitions = HashMap::new();

    for path in paths {
        let contents: holidays_gem_definitions::CountryFile = serde_yaml::from_reader(
            std::fs::File::open(&path).context("Error opening country file.")?,
        )
        .context(format!("Deserialization error reading {:?}.", &path))
        .context("Error reading the definition files")?;
        let country: String = path.file_stem().unwrap().to_string_lossy().into_owned();
        definitions.insert(country, contents);
    }

    let out_file_path = std::path::Path::new(&std::env::var("OUT_DIR")?).join("definitions.yml");
    let out_file = std::fs::File::create(out_file_path)?;
    Ok(serde_yaml::to_writer(out_file, &definitions)?)
}
