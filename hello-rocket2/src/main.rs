#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "hello, world@"
}

fn main() {
    rocket::ignite().mount("/hello", routes![world]).launch();
}
