#![feature(decl_macro, proc_macro_hygiene, rustc_attrs)]
#![allow(unused_unsafe)]

mod api;
pub mod data;

#[macro_use]
extern crate rocket;

use self::api::routes;
use parking_lot::Mutex;
use postgres::{Client, NoTls};
use rocket::ignite;

struct ServerState {
	database: Mutex<Client>
}

fn main() {
	let state = ServerState {
		database: Mutex::new(
			Client::configure()
				.host("localhost")
				.user("root")
				.dbname("catboys")
				.connect(NoTls)
				.unwrap()
		)
	};

	ignite()
		.manage(state)
		.mount("/api", routes())
		.launch();
}
