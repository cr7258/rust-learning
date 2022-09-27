mod models;
use models::user_model::UserInfo;

fn set_user(u:&mut UserInfo) {
    u.user_id = 7;
    u.user_name = String::from("chengzw");
    u.user_age = 19;
    u.user_tags = ["java", "golang", "rust", "php", "python"];
}

fn main() {
    let mut user = models::user_model::new_user_info();
    set_user(&mut user);
    println!("{:?}", user);
}
