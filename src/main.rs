#![feature(proc_macro_hygiene, decl_macro)]
#![deny(clippy::pedantic)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
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

#[catch(404)]
#[get("/404")]
fn e_404() -> Template {
    Template::render("404", &json!({}))
}

#[derive(FromForm)]
struct Submit {
    chapter_num_1: String,
}
#[post("/guest", data = "<task>")]
fn guest_editor_post(task: Form<Submit>) -> Template {
    Template::render("editor", &json!({}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, guest_editor, guest_editor_post, e_404])
        .mount("/public/style", StaticFiles::from("style"))
        .register(catchers![e_404])
        .attach(Template::fairing())
        .launch();
}
