#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
