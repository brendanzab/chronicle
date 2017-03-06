#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate uuid;


pub mod models;
pub mod schema;


embed_migrations!("migrations");
