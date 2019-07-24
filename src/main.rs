#![feature(proc_macro_hygiene, decl_macro)]
#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate rocket;

use rocket::{request::Form, response::Redirect};
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

#[get("/projects/<user>/new", rank = 1)]
fn new_project(user: String) -> Template {
    Template::render(
        "new",
        &json!({
            "user": user,
            "text": "Name your project",
            "post_url": uri!(submitted_project_name: user = user.clone()).path() }),
    )
}

#[derive(FromForm)]
struct Submit {
    message: String,
}

#[post("/projects/<user>/new", data = "<task>")]
fn submitted_project_name(user: String, task: Form<Submit>) -> Redirect {
    Redirect::to(uri!(
        project_editor: user = user,
        project_name = task.into_inner().message
    ))
}

#[get("/projects/<user>/<project_name>", rank = 2)]
fn project_editor(user: String, project_name: String) -> Template {
    Template::render(
        "project_editor",
        &json!({ "user": user, "project_name": project_name  }),
    )
}

#[catch(404)]
fn e_404() -> Template {
    Template::render("404", &json!({}))
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
            routes![
                index,
                user_page,
                project_editor,
                chapter_editor,
                new_project,
                submitted_project_name
            ],
        )
        .mount("/public/style", StaticFiles::from("style"))
        .register(catchers![e_404])
        .attach(Template::fairing())
        .launch();
}
