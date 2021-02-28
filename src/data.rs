use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct LoginRequest {
	password: String,
	username: String
}

impl LoginRequest {
	fn validate(&self) {
		
	}
}
