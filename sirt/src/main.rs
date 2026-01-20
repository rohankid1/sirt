use libsirt::types::*;
use serde::{Deserialize, Serialize};

const FILE: &'static str = include_str!("../../examples/one.ro");

#[derive(Debug, Deserialize, Serialize)]
struct Thing {
    a: Vec<String>,
    b: Vec<Int>,
}

fn main() {
    let a: Thing = libsirt::from_str_named(FILE, "Thing").unwrap();
    println!("{:?}", a.a);
    let a = libsirt::to_pretty_string("Thing", &a).unwrap();
    println!("{a}");
    println!("{:?}", libsirt::from_str::<Thing>(&a).unwrap());
}
