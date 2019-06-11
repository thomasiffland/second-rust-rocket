#![feature(proc_macro_hygiene, decl_macro)]
extern crate rand; // 0.6.5


#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;

const GENERATE_URL: &str = "http://localhost:9090/generate";


#[get("/sum/<num>")]
fn sum_with_num(num: u128) -> Json<u128> {

    let numbers = send_generate_request(num).unwrap();
    let sum = numbers.iter().fold(0,|a, &b| a + b);

    return Json(sum)
}
#[post("/sum",data = "<integers>")]
fn sum(integers: Json<Vec<u128>>) -> Json<u128> {
    let numbers = integers.into_inner();
    let sum = numbers.iter().fold(0,|a, &b| a + b);
    return Json(sum);
}


fn send_generate_request(num: u128) -> Result<(Vec<u128>), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", GENERATE_URL,num).to_string();
    let res = client.get(&url)
        .send()?.json();
    return res;
}




fn main() {
    rocket::ignite().mount("/", routes![sum,sum_with_num]).launch();
}