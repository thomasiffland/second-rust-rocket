#![feature(proc_macro_hygiene, decl_macro)]
extern crate rand; // 0.6.5


#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rand::Rng;

const SORT_URL: &str = "http://localhost:9091/sort";
const SORT_REVERSE_URL: &str = "http://localhost:9091/sort/reverse";


#[get("/generate/<num>")]
fn generate(num: u128) -> Json<Vec<u128>> {
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<u128>::new();

    for _ in 0..num {
        numbers.push(rng.gen_range(1, 100001));
    }

    return Json(numbers)
}
#[post("/sort",data = "<integers>")]
fn sort(integers: Json<Vec<u128>>) -> Json<Vec<u128>> {
    let mut numbers = integers.into_inner();
    numbers.sort();
    return Json(numbers);
}

#[get("/generate/<num>/sorted")]
fn generate_sorted(num: u128) -> Json<Vec<u128>> {
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<u128>::new();

    for _ in 0..num {
        numbers.push(rng.gen_range(1, 100001));
    }


    let result = send_sort_request(&mut numbers,SORT_URL);
    return Json(result.unwrap());
}

fn send_sort_request(numbers: &mut Vec<u128>,url: &str) -> Result<(Vec<u128>), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&numbers)
        .send()?.json();
    return res;
}


#[get("/generate/<num>/sorted/reverse")]
fn generate_sorted_reverse(num: u128) -> Json<Vec<u128>> {
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<u128>::new();

    for _ in 0..num {
        numbers.push(rng.gen_range(1, 100001));
    }

    let result = send_sort_request(&mut numbers,SORT_REVERSE_URL);
    return Json(result.unwrap());
}


fn main() {
    rocket::ignite().mount("/", routes![generate,generate_sorted,generate_sorted_reverse]).launch();
}