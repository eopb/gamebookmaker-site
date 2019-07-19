#![feature(proc_macro_hygiene, decl_macro)]
#![deny(clippy::pedantic)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde_json::json;

#[get("/")]
fn index() -> Template {
    Template::render("index", &json!({}))
}

#[get("/guest")]
fn guest_editor() -> Template {
    Template::render("editor", &json!({}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, guest_editor])
        .mount("/public/style", StaticFiles::from("style"))
        .attach(Template::fairing())
        .launch();
}
