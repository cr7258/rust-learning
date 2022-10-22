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

    let mut user_score_a=models::user_model::new_user_score_a();
    user_score_a.user_id = 10;
    user_score_a.score = 101;
    println!("{:?},{:?},{:?}", user_score_a, user_score_a.get_user_type(),user_score_a.get_user_id()); 

    let mut user_score_b=models::user_model::new_user_score_b();
    user_score_b.user_id = "S1001";
    user_score_b.score = 10.0;
    println!("{:?},{:?},{:?}", user_score_b, user_score_b.get_user_type(),user_score_b.get_user_id()); 
}
