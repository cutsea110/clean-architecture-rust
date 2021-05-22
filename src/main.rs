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
