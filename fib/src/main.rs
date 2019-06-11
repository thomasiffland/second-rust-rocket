#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;


#[get("/fib/<num>")]
fn fib(num: u128) -> Json<u128> {
    return Json(fibonacci(num));
}

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    rocket::ignite().mount("/", routes![fib]).launch();
}