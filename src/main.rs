mod user {
    pub trait UserDao {
        type FindRequest;
        type FindResponse;
        fn find_user(&self, req: Self::FindRequest) -> Self::FindResponse;
    }
    pub trait HaveUserDao {
        type UserDao: UserDao;
        fn user_dao(&self) -> Self::UserDao;
    }

    pub trait UserService: HaveUserDao {
        fn get_user_by_id(
            &self,
            req: <<Self as HaveUserDao>::UserDao as UserDao>::FindRequest,
        ) -> <<Self as HaveUserDao>::UserDao as UserDao>::FindResponse {
            self.user_dao().find_user(req)
        }
    }

    impl<T: HaveUserDao> UserService for T {}

    pub trait HaveUserService {
        type UserService: UserService;
        fn user_service(&self) -> Self::UserService;
    }
}

mod group {
    pub trait GroupDao {
        type FindRequest;
        type FindResponse;
        fn find_group(&self, req: Self::FindRequest) -> Self::FindResponse;
    }
    pub trait HaveGroupDao {
        type GroupDao: GroupDao;
        fn group_dao(&self) -> Self::GroupDao;
    }

    pub trait GroupService: HaveGroupDao {
        fn get_group_by_id(
            &self,
            req: <<Self as HaveGroupDao>::GroupDao as GroupDao>::FindRequest,
        ) -> <<Self as HaveGroupDao>::GroupDao as GroupDao>::FindResponse {
            self.group_dao().find_group(req)
        }
    }

    impl<T: HaveGroupDao> GroupService for T {}

    pub trait HaveGroupService {
        type GroupService: GroupService;
        fn group_service(&self) -> Self::GroupService;
    }
}

mod user_impl {
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
}

mod group_impl {
    use crate::group::*;

    #[derive(Copy, Clone)]
    struct PgConnection;

    #[derive(Copy, Clone)]
    pub struct GroupPgDao(PgConnection);

    impl GroupPgDao {
        pub fn new() -> Self {
            GroupPgDao(PgConnection)
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
    pub struct Group {
        pub id: u32,
        pub name: String,
    }
}

mod server {
    use super::group::*;
    use super::group_impl::*;
    use super::user::*;
    use super::user_impl::*;

    #[derive(Copy, Clone)]
    pub struct Server {
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
}

use group::*;
use group_impl::*;
use server::*;
use user::*;
use user_impl::*;

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
