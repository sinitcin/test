#![feature(const_vec_new)]
#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate regex;
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
extern crate base64_stream;
extern crate errors;
extern crate opencv;

use base64_stream::FromBase64Reader;
use errors::*;
use regex::Regex;
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
use std::io::prelude::*;
use std::io::{Cursor, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const MAX_IMG_COUNT: u32 = 25;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn upload_from_json(data: String) -> Result<String, SimpleError> {
    let v: Value = serde_json::from_str(&data)?;
    let files = v["files"].as_array().unwrap();
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
        if file.starts_with("data:image/") {
            // dbg!(file);
            let regex = match Regex::new(r"(?m)data:image.([a-z]*);base64,") {
                Ok(r) => r,
                Err(e) => {
                    return Err(SimpleError::IoError(Error::new(
                        ErrorKind::Other,
                        format!(
                            "Не смог скомпилировать регулярное выражение: {}",
                            e
                        ),
                    )));
                }
            };
            let matches = regex.captures_iter(file).nth(0).unwrap();
            let head = &matches[0];
            let fname = Path::new("upload")
                .join("byjson")
                .with_extension(&matches[1]);
            let body = file.trim_start_matches(head);
            let mut reader = FromBase64Reader::new(Cursor::new(body));
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            let mut file = File::create(fname)?;
            file.write_all(data.as_slice())?;
        } else if file.starts_with("http") {
            let mut response = client.get(file).send()?;
            let mut dest = {
                let fname = Path::new("upload").join(Path::new(file).file_name().unwrap());
                File::create(fname)?
            };
            std::io::copy(&mut response, &mut dest)?;
        } else {
            SimpleError::IoError(Error::new(
                ErrorKind::InvalidData,
                "Не известный формат изображения.",
            ));
        }
    }
    Ok(data)
}

#[post("/upload_rest", format = "application/json", data = "<data>")]
fn upload_rest(data: String) -> String {
    let result = upload_from_json(data);
    match result {
        Ok(_) => { /* Передать изменённые файлы обратно */ }
        Err(_) => { /* Сообщить об ошибке */ }
    }
    "".to_string()
}

#[post("/upload_multipart?<img_count>", data = "<data>")]
fn upload_multipart(content_type: &ContentType, img_count: u32, data: Data) -> RawResponse {
    if img_count > MAX_IMG_COUNT {
        return RawResponse::from_vec(
            format!(
                "Нельзя загружать больше {} изображений за один раз.",
                img_count
            )
            .bytes()
            .collect(),
            "",
            Some(mime::TEXT_PLAIN_UTF_8),
        );
    }
    let mut options = MultipartFormDataOptions::new();
    for _ in 0..img_count {
        options.allowed_fields.push(
            MultipartFormDataField::raw("images[]")
                .size_limit(32 * 1024 * 1024)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
        );
    }

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

    let image = multipart_form_data.raw.remove(&"images[]".to_string());

    match image {
        Some(image) => match image {
            RawField::Single(raw) => {
                println!("RawField::Single");
                let content_type = raw.content_type;
                let file_name = raw.file_name.unwrap_or("images[]".to_string());
                let data = raw.raw;

                let mut file =
                    File::create(Path::new("upload").join(&file_name).to_str().unwrap()).unwrap();
                file.write_all(&data).unwrap();

                RawResponse::from_vec(data, file_name, content_type)
            }
            RawField::Multiple(raws) => {
                println!("RawField::Multiple");
                for raw in raws {
                    //let content_type = raw.content_type;
                    let file_name = raw.file_name.unwrap_or("images[]".to_string());
                    let data = raw.raw;

                    let mut file =
                        File::create(Path::new("upload").join(&file_name).to_str().unwrap())
                            .unwrap();
                    file.write_all(&data).unwrap();

                    //RawResponse::from_vec(data, file_name, content_type)
                }
                unreachable!()
            }
        },
        None => RawResponse::from_vec(
            "Вы не отправили ни одного файла, загружать нечего.".bytes().collect(),
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
