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
