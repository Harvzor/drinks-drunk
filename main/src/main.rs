#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};
use std::thread;

use core;
use cli;
use api;

fn main() {
    let drinks = Arc::new(Mutex::new(core::drinks::Drinks::new()));

    let drinks_api = drinks.clone();

    thread::spawn(move || {
        api::Api::new(drinks_api)
            .run();
    });

    loop {
        cli::cli(drinks.clone());
    }
}