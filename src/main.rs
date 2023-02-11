#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

const HOSTNAME: &'static str = "localhost:8000";
const PROTOCOL: &'static str = "http://";

#[get("/")]
fn index() -> Template {
   Template::render("index", context!{
        hostname: HOSTNAME,
        protocol: PROTOCOL
   }) 
}

#[get("/services")]
fn services() -> Template {
    Template::render("services", context!{
        hostname: HOSTNAME,
        protocol: PROTOCOL
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, services])
        .attach(Template::fairing())
}
