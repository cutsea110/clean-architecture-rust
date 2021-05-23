use crate::user::*;

#[derive(Copy, Clone)]
struct PgConnection;

#[derive(Copy, Clone)]
pub struct UserPgDao(PgConnection);

impl UserPgDao {
    pub fn new() -> Self {
        UserPgDao(PgConnection)
    }
}

impl UserDao for UserPgDao {
    type FindRequest = u32;
    type FindResponse = Option<User>;
    fn find_user(&self, key: u32) -> Option<User> {
        match key {
            1..=100 => None,
            _ => Some(User {
                id: key,
                name: String::from(format!("user name {}", key)),
            }),
        }
    }
}
impl HaveUserDao for UserPgDao {
    type UserDao = Self;
    fn user_dao(&self) -> Self::UserDao {
        let con = PgConnection;
        UserPgDao(con)
    }
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
}
