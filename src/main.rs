mod group;
mod group_impl;
mod user;
mod user_impl;

use group::*;
use group_impl::*;
use user::*;
use user_impl::*;

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
        user_dao: UserPgDao::new(),
        group_dao: GroupPgDao::new(),
    };

    let user = server.user_dao.find_user(101);
    println!("{:?}", user);
    let group = server.group_dao.find_group(21);
    println!("{:?}", group);
}
