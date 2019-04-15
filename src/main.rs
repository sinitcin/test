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
extern crate libtest;
extern crate vips_sys;

use base64_stream::{FromBase64Reader, ToBase64Reader};
use config::*;
use errors::*;
use libtest::*;
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

#[post("/upload_rest", format = "application/json", data = "<data>")]
fn upload_rest(data: String) -> String {
    let mut response_files = Vec::new();
    let _ = upload_from_json(&mut response_files, data).unwrap();
    let json: Value = response_files.into();
    json.to_string()
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
