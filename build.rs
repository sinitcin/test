extern crate bindgen;

use std::env;
use std::fs::{self, DirEntry};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn get_exedir(env_path: &str, level: u8) -> String {
    let mut curr_level = 0;
    let mut use_dir = true;
    let mut out_dir: String = env_path
        .chars()
        .map(|x| {
            if x == '\\' {
                curr_level += 1;
                if curr_level == level {
                    use_dir = false;
                }
            };
            if use_dir {
                x
            } else {
                '\0'
            }
        })
        .collect();
    out_dir.retain(|c| c != '\0');
    out_dir
}

fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn get_relevant_path(base_path: &str, other_path: &str) -> String {
    other_path.trim_start_matches(base_path).to_owned()
}

fn recurse_copy(out_dir: &str, proj_dir: &str, other_dir: &str) -> Result<(), std::io::Error> {
    let path = Path::new(&other_dir);
    visit_dirs(path, &mut |dir_entry: &DirEntry| {
        // Получили путь к файлу
        let dir = &dir_entry.clone();
        let template_path = dir.path();

        // Получим его путь относительно основной директории
        let relevant_path = &template_path.to_string_lossy();
        let relevant_path = get_relevant_path(&proj_dir, relevant_path);
        println!("{:?}", relevant_path);

        let dir_path = Path::new(&relevant_path).parent().unwrap();
        let _ =
            fs::create_dir_all(format!("{}{}", out_dir, dir_path.to_string_lossy())).unwrap_or(());
        fs::copy(&template_path, format!("{}{}", out_dir, relevant_path)).unwrap();
    })?;
    Ok(())
}

fn generate_bindings() {
    println!("cargo:rustc-link-lib=ImageMagick");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() -> Result<(), std::io::Error> {
    // Мы не в CI?
    if let Ok(_expr) = env::var("TRAVIS_RUST_VERSION") {
        return Ok(());
    }

    // Создание биндингов
    //generate_bindings();

    // Получаем переменные
    let env_out_dir = env::var("OUT_DIR").unwrap();
    // let env_out_dir = "C:\\sci_questionnaire\\target\\debug\\build";
    let out_dir = get_exedir(&env_out_dir, 4);
    let proj_dir = get_exedir(&env_out_dir, 2);
    let templates_dir = format!("{}\\templates", proj_dir);
    let static_dir = format!("{}\\static", proj_dir);
    // Сканируем директории шаблонов
    recurse_copy(&out_dir, &proj_dir, &templates_dir)?;
    recurse_copy(&out_dir, &proj_dir, &static_dir)?;
    Ok(())
}
