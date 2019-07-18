#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde_json::json;

#[get("/")]
fn index() -> Template {
    Template::render("index", &json!({}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/public/style", StaticFiles::from("style"))
        .attach(Template::fairing())
        .launch();
}
