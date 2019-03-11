#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] extern crate rocket;
extern crate multipart;
extern crate rocket_contrib;
extern crate serde_json;

#[macro_use] extern crate serde_derive;

use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::thread;

use rocket::http::RawStr;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::*;
use rocket::http::ContentType;
use std::io::{Cursor, Read};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use multipart::server::Multipart;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[post("/create", format = "application/json", data = "<data>")]
fn create(data: String) -> String {    
    format!("{:?}", data)
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
        .mount("/", routes![index, files, create])
        .register(catchers![not_found])
        .launch();
}
