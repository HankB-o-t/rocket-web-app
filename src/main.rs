#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
   Template::render("index", context!{
        hostname: "localhost:8000",
        protocol: "http://"
   }) 
}

#[get("/services")]
fn services() -> Template {
    Template::render("services", context!{
        hostname: "localhost:8000",
        protocol: "http://"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, services])
        .attach(Template::fairing())
}
