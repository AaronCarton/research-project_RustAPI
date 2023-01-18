use crate::db::establish_connection;
use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
use std::borrow::BorrowMut;
use std::time::SystemTime;

pub fn create_user(new_user: NewUser) -> User {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    users.order(id.desc()).first(connection).unwrap()
}

pub fn get_users() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users.load::<User>(connection).unwrap();

    results
}

pub fn get_user(id: i32) -> User {
    let connection = &mut establish_connection();
    users::table.find(id).first(connection).unwrap()
}

pub fn get_user_by_uid(uid: String) -> User {
    let connection = &mut establish_connection();
    users::table
        .filter(users::columns::uid.eq(uid))
        .first(connection)
        .unwrap()
}

pub fn delete_user(id: i32) {
    let connection = &mut establish_connection();
    diesel::delete(users::table.find(id))
        .execute(connection)
        .unwrap();
}

pub fn update_user(user: User) -> User {
    // let connection = &mut establish_connection();
    // diesel::update(users::table.find(user.id))
    //     .set(&user)
    //     .get_result(connection)
    //     .unwrap()
    user
}

pub fn increase_score(id: i32) -> User {
    let connection = &mut establish_connection();
    let user = get_user(id);
    let new_score = user.score + 1;
    diesel::update(users::table.find(id))
        .set(users::score.eq(new_score))
        .execute(connection)
        .unwrap();

    get_user(id)
}
