use crate::{
    get_config,
    types::api::{error, img_gen},
};
use actix_files::NamedFile;
use actix_web::{web::Json, Responder};
use log::debug;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub async fn handle(req: Json<img_gen::Request>) -> Result<impl Responder, error::Error> {
    let name = request_hash(&req.hash).await?;

    if name.is_empty() {
        return Err(error::Error::Internal);
    }

    let local_filename = base64::encode(&name);
    let exists = img_cached(&local_filename);

    if !exists {
        gen_image(&name)?;
    }

    Ok(NamedFile::open(img_fpath(&local_filename))?)
}

async fn request_hash(inp: &str) -> Result<String, error::Error> {
    // if resp empty: -> name does not exists -> 404 returnen
    Ok(String::from("testfilehashlolmock"))
}

fn img_path() -> PathBuf {
    Path::new(&crate::get_config().image.image_cache).to_path_buf()
}

fn img_fpath(name: &str) -> PathBuf {
    let filename = format!("{name}.png");
    img_path().join(&filename)
}

fn img_cached(name: &str) -> bool {
    img_fpath(name).exists()
}

fn gen_image(name: &str) -> Result<(), error::Error> {
    if !img_path().exists() {
        std::fs::create_dir_all(img_path())?;
    }

    let bg_img = &crate::get_config().image.bg_image;

    // TODO: find a way to include this into the rust binary at ct
    let py_path = abs_path("../../python/pic_mod/main.py")?;
    let py_path = py_path.as_str();

    let font_file = abs_path(&get_config().image.font)?;
    let font_file = font_file.as_str();

    let out_path = abs_path(img_path())?;
    let out_path = out_path.as_ref();

    let mut cmd = Command::new(&get_config().image.python_bin)
        .args([py_path, name, bg_img, font_file, out_path])
        .spawn()?;

    let res = cmd.wait()?;

    debug!("Python exectuion exited with {:?}", res.code());

    Ok(())
}

fn abs_path<P: AsRef<Path>>(inp: P) -> Result<String, error::Error> {
    Ok(std::fs::canonicalize(inp.as_ref())?
        .to_str()
        .unwrap()
        .to_string())
}
