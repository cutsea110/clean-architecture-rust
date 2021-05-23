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
