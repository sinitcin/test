extern crate base64_stream;
extern crate config;
extern crate errors;
extern crate regex;
extern crate reqwest;
extern crate rocket_multipart_form_data;
extern crate vips_sys;

use base64_stream::{FromBase64Reader, ToBase64Reader};
use config::*;
use errors::*;
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::fs::*;
use std::io;
use std::io::*;
use std::path::Path;

// Преобразование файла в формат img base64
pub fn to_imgbase64(crop_path: &str, ext: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let file = File::open(crop_path)?;
    let mut reader = ToBase64Reader::new(file);
    let _ = reader.read_to_string(&mut buffer);
    Ok(format!("data:image/{};base64,{}", ext, buffer))
}

// Сохранение изображений из multipart form data с изменением размера до 100х100 пикселей
// в вектор response_files
pub fn raw_save(response_files: &mut Vec<String>, raw: rocket_multipart_form_data::SingleRawField) {
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
    response_files.push(b64file);

    // Удаляем файл
    fs::remove_file(file_name).unwrap();
    fs::remove_file(crop_path).unwrap();
}

// Сохранение изображений из REST с изменением размера до 100х100 пикселей
// в вектор response_files
pub fn upload_from_json(
    response_files: &mut Vec<String>,
    data: String,
) -> std::result::Result<String, SimpleError> {
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
            let file_name = Path::new("upload").join(Path::new(file).file_name().unwrap());
            let file_ext = &file_name.extension().unwrap().to_str().unwrap();
            let mut dest = File::create(&file_name)?;
            std::io::copy(&mut response, &mut dest)?;

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
        } else {
            SimpleError::IoError(Error::new(
                ErrorKind::InvalidData,
                "Не известный формат изображения.",
            ));
        }
    }
    Ok(data)
}