#![feature(proc_macro_hygiene, decl_macro)]
#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde_json::json;

#[get("/")]
fn index() -> Template {
    Template::render("index", &json!({}))
}

#[get("/users/<user>")]
fn user_page(user: String) -> Template {
    Template::render("user_page", &json!({ "user": user }))
}

#[get("/projects/<user>/<project_name>")]
fn project_editor(user: String, project_name: String) -> Template {
    Template::render(
        "project_editor",
        &json!({ "user": user, "project_name": project_name  }),
    )
}

#[get("/projects/<user>/<project_name>/chapters/<chapter_num>")]
fn chapter_editor(user: String, project_name: String, chapter_num: u32) -> Template {
    Template::render(
        "chapter_editor",
        &json!({
            "user": user,
            "project_name": project_name,
            "chapter_num": chapter_num
        }),
    )
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, user_page, project_editor, chapter_editor],
        )
        .mount("/public/style", StaticFiles::from("style"))
        .attach(Template::fairing())
        .launch();
}
