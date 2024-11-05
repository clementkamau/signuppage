use diesel::{Insertable, Queryable};
use crate::schema::users; 

#[derive(Queryable)]
pub struct User { 
    id: i32,
    name: String,
    email: String,
    password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)] 
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
