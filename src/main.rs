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
extern crate config;
extern crate errors;
extern crate vips_sys;

use base64_stream::{FromBase64Reader, ToBase64Reader};
use config::*;
use errors::*;
use regex::Regex;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{NamedFile, Redirect, Response};
use rocket::Data;
use rocket_include_static_resources::EtagIfNoneMatch;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
    MultipartFormDataOptions, RawField, SingleRawField,
};
use rocket_raw_response::RawResponse;
use serde_json::{
    from_reader, from_slice, from_str, from_value, json, to_string, to_string_pretty, to_value,
    to_vec, to_writer, Deserializer, Number, Value,
};
use std::ffi::CString;
use std::fs;
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

fn upload_from_json(response_files: &mut Vec<String>, data: String) -> Result<String, SimpleError> {
    let v: Value = serde_json::from_str(&data)?;
    let files = v["files"].as_array().unwrap();

    let config = MainConfig::load();
    let client = match config.proxy {
        Some(proxy_param) => reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_param.proxy)?.basic_auth(
                &proxy_param.login.unwrap_or("".to_string()),
                &proxy_param.password.unwrap_or("".to_string()),
            ))
            .use_default_tls()
            .build()?,
        None => reqwest::Client::builder().use_default_tls().build()?,
    };

    for (filenum, file) in files.iter().enumerate() {
        let file = file.as_str().unwrap();
        if file.starts_with("data:image/") {
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
            let file_ext = &matches[1];
            let file_name = Path::new("upload")
                .join(format!("byjson_{}", filenum))
                .with_extension(&matches[1]);
            let body = file.trim_start_matches(head);
            let mut reader = FromBase64Reader::new(Cursor::new(body));
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            let mut file = File::create(&file_name)?;
            file.write_all(data.as_slice())?;

            // Кропаем
            let crop_path = Path::new("upload").join("crop").with_extension(file_ext);
            let crop_path = crop_path.to_str().unwrap();
            vips_sys::crop_image(file_name.to_str().unwrap(), crop_path, 100, 100)?;

            // Добавляем в массив
            let b64file = to_imgbase64(crop_path, file_ext)?;
            response_files.push(b64file);

            // Удаляем файл
            fs::remove_file(file_name)?;
            fs::remove_file(crop_path)?;
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
    let mut response_files = Vec::new();
    let _ = upload_from_json(&mut response_files, data).unwrap();
    let json: Value = response_files.into();
    json.to_string()
}

fn to_imgbase64(crop_path: &str, ext: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let file = File::open(crop_path)?;
    let mut reader = ToBase64Reader::new(file);
    let _ = reader.read_to_string(&mut buffer);
    Ok(format!("data:image/{};base64,{}", ext, buffer))
}

fn raw_save(files: &mut Vec<String>, raw: rocket_multipart_form_data::SingleRawField) {
    // Сохранение файла
    let file_path = Path::new("upload");
    let photo_name = raw.file_name.unwrap_or("images[]".to_string());
    let file_name = file_path.join(&photo_name);
    let file_ext = file_name.extension().unwrap();
    let data = raw.raw;
    let mut file = File::create(file_name.to_str().unwrap()).unwrap();
    file.write_all(&data).unwrap();

    // Кропаем
    let crop_path = Path::new("upload").join("crop").with_extension(&file_ext);
    let crop_path = crop_path.to_str().unwrap();
    vips_sys::crop_image(file_name.to_str().unwrap(), crop_path, 100, 100).unwrap();

    // Добавляем в массив
    let b64file = to_imgbase64(crop_path, file_ext.to_str().unwrap()).unwrap();
    files.push(b64file);

    // Удаляем файл
    fs::remove_file(file_name).unwrap();
    fs::remove_file(crop_path).unwrap();
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
                let mut files = Vec::new();
                raw_save(&mut files, raw);
                let json: Value = files.into();
                RawResponse::from_vec(to_vec(&json).unwrap(), "", Some(mime::TEXT_PLAIN_UTF_8))
            }
            RawField::Multiple(raws) => {
                let mut files = Vec::new();
                for raw in raws {
                    raw_save(&mut files, raw);
                }
                let json: Value = files.into();
                RawResponse::from_vec(to_vec(&json).unwrap(), "", Some(mime::TEXT_PLAIN_UTF_8))
            }
        },
        None => RawResponse::from_vec(
            "Вы не отправили ни одного файла, загружать нечего."
                .bytes()
                .collect(),
            "",
            Some(mime::TEXT_PLAIN_UTF_8),
        ),
    }
}

fn raw_to_imgbase64(raw: &std::vec::Vec<u8>) -> String {
    "".to_string()
    //data:image/{};base64,{}
}

fn raw_save(
    raw: rocket_multipart_form_data::SingleRawField,
    json_resp: &str,
) -> Result<String, SimpleError> {
    //
    let file_name = raw.file_name.unwrap_or("images[]".to_string());
    let file_name = Path::new("upload").join(&file_name);
    let data = raw.raw;
    dbg!(file_name.extension());
    let mut file = File::create(file_name.to_str().unwrap())?;    
    file.write_all(&data)?;
    raw_to_imgbase64(&data);
    Ok("".to_string())
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
