#![feature(plugin, decl_macro, proc_macro_hygiene)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_multipart_form_data;
extern crate graceful;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

use std::io;
use std::path::{Path, PathBuf};
use rocket::http::RawStr;
#[allow(unused_imports)]
use rocket::request::{Form, FromFormValue, Request};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;

use graceful::SignalGuard;

static STOP: AtomicBool = AtomicBool::new(false);

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
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
    let signal_guard = SignalGuard::new();

    let handle = thread::spawn(|| {
        println!("Worker thread started. Type Ctrl+C to stop.");
        while !STOP.load(Ordering::Acquire) {
            rocket::ignite()
                .mount("/", routes![index, files])
                .register(catchers![not_found])
                .launch();
        }
        println!("Bye.");
    });

    signal_guard.at_exit(move |sig| {
        println!("Signal {} received.", sig);
        STOP.store(true, Ordering::Release);
        handle.join().unwrap();
    });
}
