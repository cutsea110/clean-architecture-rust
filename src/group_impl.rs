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
