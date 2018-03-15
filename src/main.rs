#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mysql;

use rocket_contrib::{Json, JsonValue};
use rocket::State;
use std::collections::HashMap;
// Add database

mod store;

fn main() {
    println!("Hello, world!");
}

