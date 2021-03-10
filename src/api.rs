use self::super::{
	data::{
		primitives::{Discriminator, MaybeDiscriminator, MaybeUserName},
		structures::UserObject
	},
	ServerState
};
use rocket::{http::Status, Route, State, routes};
use rocket_contrib::json::Json as JSON;
use std::time::Duration;

#[get("/users/names/<user_name>")]
fn get_users_names_user_name<'p>(state: State<ServerState>,
		user_name: MaybeUserName<'p>) -> Result<JSON<UserObject>, Status> {
	// SAFETY: 0 is a valid discriminator.
	let discriminator = unsafe {Ok(Discriminator::new_unchecked(0))};
	get_users_names_user_name_discriminator(state, user_name, discriminator)
}

#[get("/users/names/<user_name>/<discriminator>")]
fn get_users_names_user_name_discriminator<'p>(state: State<ServerState>,
		user_name: MaybeUserName<'p>, discriminator: MaybeDiscriminator<'p>) ->
			Result<JSON<UserObject>, Status> {
	match state.database.try_lock_for(Duration::from_secs(5)) {
		Some(mut database) => match (user_name, discriminator) {
			(Ok(user_name), Ok(discriminator)) => {
				let data = database.query(
					"SELECT id FROM users WHERE user_name = $1 AND discriminator = $2",
					&[&user_name, &discriminator]
				).unwrap();
				assert!(2 > data.len());

				match data.first() {
					Some(row) => Ok(JSON(UserObject {
						id: row.get::<_, i64>("id") as usize,
						user_name,
						discriminator
					})),
					None => Err(Status::NotFound)
				}
			},
			_ => Err(Status::BadRequest)
		},
		// This may actually happen without any bugs, it just means the server is
		// slow.
		None => Err(Status::InternalServerError)
	}
}

pub fn routes() -> Vec<Route> {
	routes![
		get_users_names_user_name,
		get_users_names_user_name_discriminator
	]
}
