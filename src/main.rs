#![feature(const_vec_new)]
#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate rocket_include_static_resources;
extern crate rocket_raw_response;
#[macro_use] extern crate rocket;
extern crate rocket_multipart_form_data;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::thread;
use rocket::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, MultipartFormDataError, RawField};
use rocket_include_static_resources::EtagIfNoneMatch;
use rocket_raw_response::RawResponse;
use rocket::response::{Response, NamedFile, Redirect};
use rocket::request::Request;
use serde_json::{
    from_reader, from_slice, from_str, from_value, to_string, to_string_pretty, to_value, to_vec,
    to_writer, Deserializer, Number, Value,
};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[post("/upload_rest", format = "application/json", data = "<data>")]
fn upload_json(data: String) -> String {
    let v: Value = serde_json::from_str(&data).unwrap();
    let files = v["files"].as_array();
    for file in &files {
        println!("{:?}", &file);
    }
    format!("{:?}", data)
}

#[post("/upload_multipart", data = "<data>")]
fn upload(content_type: &ContentType, data: Data) -> RawResponse {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::raw("image").size_limit(32 * 1024 * 1024).content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => match err {
            MultipartFormDataError::DataTooLargeError(_) => return RawResponse::from_vec("Размер файла слишком большой.".bytes().collect(), "", Some(mime::TEXT_PLAIN_UTF_8)),
            MultipartFormDataError::DataTypeError(_) => return RawResponse::from_vec("Вы отправили не изображение.".bytes().collect(), "", Some(mime::TEXT_PLAIN_UTF_8)),
            _ =>  return RawResponse::from_vec("Произошла ошибка при загрузке файла.".bytes().collect(), "", Some(mime::TEXT_PLAIN_UTF_8))
        }
    };

    let image = multipart_form_data.raw.remove(&"image".to_string());

    match image {
        Some(image) => match image {
            RawField::Single(raw) => {
                let content_type = raw.content_type;
                let file_name = raw.file_name.unwrap_or("Image".to_string());
                let data = raw.raw;
                
                let mut file = File::create(Path::new("upload").join(&file_name).to_str().unwrap()).unwrap();
                file.write_all(&data).unwrap();

                RawResponse::from_vec(data, file_name, content_type)
            }
            RawField::Multiple(_) => unreachable!()
        },
        None => RawResponse::from_vec("Please input a file.".bytes().collect(), "", Some(mime::TEXT_PLAIN_UTF_8))
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(_req: &Request) -> io::Result<NamedFile> {
    NamedFile::open("static/404.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, upload, upload_json])
        .register(catchers![not_found])
        .launch();
}
