use self::super::data::LoginRequest;
use rocket_contrib::json::Json;

#[post("/login", data = "<data>")]
fn login_post(data: Json<LoginRequest>) {

}
// owo