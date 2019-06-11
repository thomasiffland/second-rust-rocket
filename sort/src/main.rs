#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
#[post("/sort",data = "<integers>")]
fn sort(integers: Json<Vec<u128>>) -> Json<Vec<u128>> {
    let mut numbers = integers.into_inner();
    numbers.sort();
    return Json(numbers);
}

#[post("/sort/reverse",data = "<integers>")]
fn sort_reverse(integers: Json<Vec<u128>>) -> Json<Vec<u128>> {
    let mut numbers = integers.into_inner();
    numbers.sort();
    numbers.reverse();
    return Json(numbers);
}
fn main() {
    rocket::ignite().mount("/", routes![sort,sort_reverse]).launch();
}