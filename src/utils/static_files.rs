use actix_files::NamedFile;
use actix_web::{ get, HttpRequest, Result, Error };
use std::path::PathBuf;

#[get("/static/{filename:.*}")]
pub async fn get_statics(req: HttpRequest) -> Result<NamedFile, Error> {
    let pathf: PathBuf = req.match_info().query("filename").parse().unwrap();

    let mut path_string = pathf.into_os_string().into_string().unwrap();
    path_string = format!("./static/{}", path_string);

    let static_file = NamedFile::open(path_string)?;

    Ok(static_file.use_last_modified(true))
}