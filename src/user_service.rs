use crate::db;
use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;

pub fn create_user(mut new_user: NewUser) -> User {
    let connection = db::create_connection();
    connection
        .transaction::<User, Error, _>(|| {
            new_user.created = Some(SystemTime::now());
            new_user.score = Some(0);
            let user: User = diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(&connection)
                .unwrap();

            Ok(user)
        })
        .unwrap()
}

pub fn get_users() -> Vec<User> {
    let connection = db::create_connection();
    users::table.load::<User>(&connection).unwrap()
}

pub fn get_user(id: i32) -> User {
    let connection = db::create_connection();
    users::table.find(id).first(&connection).unwrap()
}

pub fn delete_user(id: i32) {
    let connection = db::create_connection();
    diesel::delete(users::table.find(id))
        .execute(&connection)
        .unwrap();
}

pub fn update_user(user: User) -> User {
    let connection = db::create_connection();
    diesel::update(users::table.find(user.id))
        .set(users::columns::username.eq(user.username))
        .get_result(&connection)
        .unwrap()
}
