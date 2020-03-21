use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{db::DbConn, model::user};
use user::User;

use std::io::Error as ioError;

// NOTE: test API without db conneciton;
fn users_mock() -> Result<Json<User>, ()> {
    let _users_json = r#"{"id":"89251ab3-1cdc-4629-9086-ce022cf3549e","first_name":"Marek","last_name":"Majdak","email":"marek.majdak@sufrago.com","name":"sufrago","create_at":"2019-12-17T17:58:07.533406","avatar_id":"1cb15088-afd4-4d00-a7fc-d95eae1abefb","phone_no":"+48505117069","company_name":"Sufrago sp z o.o.","vat_id":"PL 9512468001"}"#;

    let user: User = serde_json::from_str(_users_json).unwrap();

    Ok(Json(user))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<User>>, Status> {
    user::all(&connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

// #[get("/")]
// pub fn all() -> Result<Json<User>, ()> {
//     users_mock()
// }
