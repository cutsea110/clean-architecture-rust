mod dao {
    pub mod user {

        pub trait UserDao {
            type FindRequest;
            type FindResponse;
            fn find_user(&self, req: Self::FindRequest) -> Self::FindResponse;
        }

        pub trait HaveUserDao {
            type UserDao: UserDao;
            fn user_dao(&self) -> Self::UserDao;
        }
    }
    pub mod group {

        pub trait GroupDao {
            type FindRequest;
            type FindResponse;
            fn find_group(&self, req: Self::FindRequest) -> Self::FindResponse;
        }

        pub trait HaveGroupDao {
            type GroupDao: GroupDao;
            fn group_dao(&self) -> Self::GroupDao;
        }
    }
}

mod service {
    pub mod user {
        use super::super::dao::user::{HaveUserDao, UserDao};

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

    pub mod group {
        use super::super::dao::group::{GroupDao, HaveGroupDao};

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
}

mod pgdao {
    pub mod models {
        #[derive(Debug)]
        pub struct User {
            pub id: u32,
            pub name: String,
        }
        #[derive(Debug)]
        pub struct Group {
            pub id: u32,
            pub name: String,
        }
    }

    pub mod user {
        use super::models::User;
        use crate::dao::user::UserDao;

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
    }

    pub mod group {
        use super::models::Group;
        use crate::dao::group::GroupDao;

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
    }
}

mod cli {
    pub use super::dao::group::{GroupDao, HaveGroupDao};
    pub use super::dao::user::{HaveUserDao, UserDao};

    pub use super::pgdao::{group::GroupPgDao, user::UserPgDao};

    #[derive(Copy, Clone)]
    pub struct Server {
        user_dao: UserPgDao,
        group_dao: GroupPgDao,
    }

    impl Server {
        pub fn new() -> Self {
            Server {
                user_dao: UserPgDao::new(),
                group_dao: GroupPgDao::new(),
            }
        }
    }

    impl HaveUserDao for Server {
        type UserDao = UserPgDao;
        fn user_dao(&self) -> UserPgDao {
            self.user_dao
        }
    }

    impl HaveGroupDao for Server {
        type GroupDao = GroupPgDao;
        fn group_dao(&self) -> GroupPgDao {
            self.group_dao
        }
    }
}

// main

use cli::Server;
use service::group::GroupService;
use service::user::UserService;

fn main() {
    let cli = Server::new();

    let user = cli.get_user_by_id(101);
    println!("{:?}", user);
    let group = cli.get_group_by_id(21);
    println!("{:?}", group);
}
