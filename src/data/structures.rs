use self::super::primitives::{Discriminator, UserName};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserObject {
	pub id: usize,
	pub user_name: UserName,
	pub discriminator: Discriminator
}
