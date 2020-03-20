use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;

use crate::schema::user;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub avatar_id: Option<String>,
    pub phone_no: Option<String>,
    pub company_name: Option<String>,
    pub vat_id: Option<String>,
}

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<User>> {
    user::table.load::<User>(&*connection)
}
