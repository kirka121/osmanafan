use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use crate::controllers;

// todo do not return a template, rather return a controller or a result or something, this file doesnt need to know about Templates
#[get("/")]
fn index() -> Template {
    controllers::home::index()
}

#[get("/about")]
fn about() -> Template {
    controllers::about::index()
}

pub fn run() {
    rocket::ignite()
        .mount("/", routes![index, about])
        .mount("/assets", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")))
        .attach(Template::fairing())
        .launch();
}