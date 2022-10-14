use std::{fs::File, io::BufWriter};

use ethers::prelude::Abigen;

const CONCRACTS_PATH: &str = "./contracts/abi/";
const OUTPUT_FILE: &str = "./types_output/abi_types.rs";

pub fn main() {
    std::env::set_var("LOGLEVEL", "trace");
    pretty_env_logger::init_custom_env("LOGLEVEL");

    let mut output_writer = BufWriter::new(File::create(OUTPUT_FILE).unwrap());

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
