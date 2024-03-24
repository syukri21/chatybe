use diesel::prelude::*;

use crate::schema::users;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Insertable)]
#[table_name = "users"]
pub struct User<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
