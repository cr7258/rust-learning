#[derive(Debug)]
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
    pub user_age: i8,
    pub user_tags: [&'static str;5]
}

pub fn new_user_info() -> UserInfo {
    UserInfo {
        user_id: 0,
        user_name: String::new(),
        user_age: 18,
        user_tags: ["";5]
    }
}