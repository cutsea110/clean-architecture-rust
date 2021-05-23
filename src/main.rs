mod group;
mod user;

use group::*;
use user::*;

#[derive(Copy, Clone)]
struct PgConnection;

#[derive(Copy, Clone)]
struct UserPgDao(PgConnection);

#[derive(Copy, Clone)]
struct GroupPgDao(PgConnection);

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

impl GroupDao for GroupPgDao {
    type FindRequest = u32;
    type FindResponse = Option<Group>;
    fn find_group(&self, key: u32) -> Option<Group> {
        match key {
            1..=10 => None,
            _ => Some(Group {
                id: key,
                name: String::from(format!("group name {}", key)),
            }),
        }
    }
}
impl HaveGroupDao for GroupPgDao {
    type GroupDao = Self;
    fn group_dao(&self) -> Self::GroupDao {
        let con = PgConnection;
        GroupPgDao(con)
    }
}

#[derive(Debug)]
struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
struct Group {
    pub id: u32,
    pub name: String,
}
#[derive(Copy, Clone)]
struct Server {
    pub user_dao: UserPgDao,
    pub group_dao: GroupPgDao,
}
impl HaveUserDao for Server {
    type UserDao = UserPgDao;
    fn user_dao(&self) -> UserPgDao {
        self.user_dao
    }
}
impl HaveUserService for Server {
    type UserService = Self;
    fn user_service(&self) -> Server {
        *self
    }
}
impl HaveGroupDao for Server {
    type GroupDao = GroupPgDao;
    fn group_dao(&self) -> GroupPgDao {
        self.group_dao
    }
}
impl HaveGroupService for Server {
    type GroupService = Self;
    fn group_service(&self) -> Server {
        *self
    }
}

fn main() {
    let server = Server {
        user_dao: UserPgDao(PgConnection),
        group_dao: GroupPgDao(PgConnection),
    };

    let user = server.user_dao.find_user(101);
    println!("{:?}", user);
    let group = server.group_dao.find_group(21);
    println!("{:?}", group);
}
