#![feature(const_vec_new)]
#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate reqwest;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lazy_static_include;
#[macro_use]
extern crate rocket_include_static_resources;
extern crate rocket_raw_response;
#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate errors;
extern crate opencv;

use errors::*;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{NamedFile, Redirect, Response};
use rocket::Data;
use rocket_include_static_resources::EtagIfNoneMatch;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
    RawField,
};
use rocket_raw_response::RawResponse;
use serde_json::{
    from_reader, from_slice, from_str, from_value, to_string, to_string_pretty, to_value, to_vec,
    to_writer, Deserializer, Number, Value,
};
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn upload_from_json(data: String) -> Result<String, SimpleError> {
    let v: Value = serde_json::from_str(&data)?;
    let files = v["files"].as_array().unwrap();
    dbg!(files);
    let client = reqwest::Client::builder()
        .proxy(
            reqwest::Proxy::http("http://proxy.bolid.ru:3128")?
                .basic_auth("sinicin", "130492130492"),
        )
        .proxy(
            reqwest::Proxy::https("http://proxy.bolid.ru:3128")?
                .basic_auth("sinicin", "130492130492"),
        )
        .use_default_tls()
        .build()?;
    for file in files {
        let file = file.as_str().unwrap();
        dbg!(file);
        if file.starts_with("data:image/") {

        } else if file.starts_with("http") {
            let mut response = client.get(file).send()?;
            let mut dest = {
                let fname = Path::new("upload").join(Path::new(file).file_name().unwrap());
                File::create(fname)?
            };
            std::io::copy(&mut response, &mut dest)?;
        } else {
            SimpleError::IoError(Error::new(ErrorKind::Other, "Не известный формат изображения."));
        }
    }
    Ok(data)
}

#[post("/upload_rest", format = "application/json", data = "<data>")]
fn upload_rest(data: String) -> String {
    let _ = upload_from_json(data);
    "".to_string()
}

#[post("/upload_multipart", data = "<data>")]
fn upload_multipart(content_type: &ContentType, data: Data) -> RawResponse {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    );

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => match err {
            MultipartFormDataError::DataTooLargeError(_) => {
                return RawResponse::from_vec(
                    "Размер файла слишком большой."
                        .bytes()
                        .collect(),
                    "",
                    Some(mime::TEXT_PLAIN_UTF_8),
                )
            }
            MultipartFormDataError::DataTypeError(_) => {
                return RawResponse::from_vec(
                    "Вы отправили не изображение."
                        .bytes()
                        .collect(),
                    "",
                    Some(mime::TEXT_PLAIN_UTF_8),
                )
            }
            _ => {
                return RawResponse::from_vec(
                    "Произошла ошибка при загрузке файла."
                        .bytes()
                        .collect(),
                    "",
                    Some(mime::TEXT_PLAIN_UTF_8),
                )
            }
        },
    };

    let image = multipart_form_data.raw.remove(&"image".to_string());

    match image {
        Some(image) => match image {
            RawField::Single(raw) => {
                let content_type = raw.content_type;
                let file_name = raw.file_name.unwrap_or("Image".to_string());
                let data = raw.raw;

                let mut file =
                    File::create(Path::new("upload").join(&file_name).to_str().unwrap()).unwrap();
                file.write_all(&data).unwrap();

                RawResponse::from_vec(data, file_name, content_type)
            }
            RawField::Multiple(_) => unreachable!(),
        },
        None => RawResponse::from_vec(
            "Please input a file.".bytes().collect(),
            "",
            Some(mime::TEXT_PLAIN_UTF_8),
        ),
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
        .mount("/", routes![index, files, upload_multipart, upload_rest])
        .register(catchers![not_found])
        .launch();
}
