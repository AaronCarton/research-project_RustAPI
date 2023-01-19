use crate::db::establish_connection;
use crate::models::user::{NewUser, User, UserHistory, UserModel};
use crate::schema::{questions, user_history, users};
use diesel::prelude::*;

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

pub fn get_user_joined(id: i32) -> UserModel {
    let connection = &mut establish_connection();

    // get user's history (History table joined with Question table)
    let history = user_history::table
        .inner_join(questions::table)
        .filter(user_history::columns::user_id.eq(id))
        .select((
            questions::columns::id,
            questions::columns::question,
            user_history::columns::answer,
        ))
        .load::<UserHistory>(connection)
        .unwrap();

    // get user's info
    let result = users::table
        .filter(users::columns::id.eq(id))
        .first::<User>(connection)
        .unwrap();

    // combine into custom model
    let user_model = UserModel {
        uid: result.uid,
        role: result.role,
        username: result.username,
        score: result.score,
        history: history,
    };

    user_model
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
