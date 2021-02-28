#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::ignite;

mod api;
mod data;

fn main() {
	ignite()
		.mount("/api", routes![])
		.launch();
}
