/*
// ref.) https://qiita.com/kawadumax/items/0875eda8a89f5d24d3c3
trait Repository {
    type CreateRequest;
    type CreateResponse;

    fn create(req: Self::CreateRequest) -> Self::CreateResponse;
}

struct PgDatabase;
impl Repository for PgDatabase {
    type CreateRequest = u8;
    type CreateResponse = User;

    fn create(age: u8) -> User {
        User { age }
    }
}

struct Service<U: Repository>(U);
impl<U: Repository> Service<U> {
    pub fn new(&self, req: U::CreateRequest) -> U::CreateResponse {
        U::create(req)
    }
}

#[derive(Debug)]
struct User {
    age: u8,
}

fn main() {
    let db: PgDatabase = PgDatabase;
    let service = Service::<PgDatabase>(db);
    let user = service.new(20);
    dbg!(user);
}
 */
/*
// ref.) https://keens.github.io/blog/2017/12/01/rustnodi/
// naive implementation
trait UserDao {
    type FindRequest;
    type FindResponse;
    fn find_user(&self, req: Self::FindRequest) -> Self::FindResponse;
}

struct PgConnection;
struct UserPgDao(PgConnection);
impl UserDao for UserPgDao {
    type FindRequest = u32;
    type FindResponse = Option<User>;
    fn find_user(&self, key: u32) -> Option<User> {
        match key {
            1..=100 => None,
            _ => Some(User {
                id: key,
                name: String::from(format!("name {}", key)),
            }),
        }
    }
}

struct UserService<U: UserDao>(U);
impl<U: UserDao> UserService<U> {
    pub fn get_user_by_id(&self, key: U::FindRequest) -> U::FindResponse {
        self.0.find_user(key)
    }
}

#[derive(Debug)]
struct User {
    pub id: u32,
    pub name: String,
}

fn main() {
    let con: PgConnection = PgConnection;
    let dao = UserPgDao(con);
    let service = UserService::<UserPgDao>(dao);
    let user = service.get_user_by_id(101);
    dbg!(user);
}
 */

// ref.) https://keens.github.io/blog/2017/12/01/rustnodi/
// cake pattern

// User
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

// Group

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

// Impl

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
