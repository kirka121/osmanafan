#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod routes;
mod controllers;

fn main() {
    routes::run();
}