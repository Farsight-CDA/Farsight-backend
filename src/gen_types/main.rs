use std::{fs::File, io::BufWriter, path::Path};

use ethers::prelude::Abigen;

const CONCRACTS_PATH: &str = "./contracts/abi/";
const OUTPUT_PATH: &str = "./types_output";
const OUTPUT_FILE: &str = "abi_types.rs";

pub fn main() {
    std::env::set_var("LOGLEVEL", "trace");
    pretty_env_logger::init_custom_env("LOGLEVEL");

    if !Path::new(OUTPUT_PATH).exists() {
        std::fs::create_dir_all(OUTPUT_PATH).expect("failed to create file");
    }

    let output_filepath = Path::new(OUTPUT_PATH).join(OUTPUT_FILE);

    let mut output_writer = BufWriter::new(File::create(output_filepath).unwrap());

    let files = std::fs::read_dir(CONCRACTS_PATH)
        .unwrap()
        .filter(|i| i.as_ref().unwrap().metadata().unwrap().is_file());

    for file in files {
        let file = file.unwrap();

        let file_name = file.file_name().to_str().unwrap().to_string();
        let file_path = file.path().to_str().unwrap().to_string();

        let contract_name = file_name.strip_suffix(".abi.json").unwrap().to_string();

        log::info!("Compiling file {:?}; Name: {contract_name:?}", file_path);

        Abigen::new(&contract_name, file_path)
            .unwrap()
            .generate()
            .unwrap()
            .write(&mut output_writer)
            .unwrap();
    }
}
