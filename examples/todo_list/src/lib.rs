#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chronicle;
extern crate chronicle_domain;
extern crate chronicle_memory;
extern crate futures;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod api;
pub mod domain;
