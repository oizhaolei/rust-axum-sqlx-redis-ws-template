use crate::models::user::{User, UserList};

#[allow(dead_code)]
pub fn user_fixture(id: i32) -> User {
    User {
        id,
        username: format!("ferrari {}", id),
        password_hash: String::from("black"),
    }
}

#[allow(dead_code)]
pub fn users_fixture(num: i32) -> UserList {
    let mut users = vec![];
    for i in 1..num + 1 {
        users.push(user_fixture(i));
    }
    UserList {
        data: users,
        total: (num * 9) as i64,
    }
}
