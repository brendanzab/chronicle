#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chronicle_domain;
extern crate chronicle_memory;
extern crate futures;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod api;
pub mod domain;
