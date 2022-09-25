fn get_user(uid:i32) ->&'static str{
    if uid == 101 {
        return "chengzw";
    }else if uid == 102 {
        return "zhangshan";
    }else {
        return "unknown";
    }
}


fn get_user2(uid:i32) ->&'static str {
    let ret = if uid == 101 {
        "chengzw"
    }else if uid == 102 {
        "zhangsan"
    }else {
        "unknown"
    };
    ret
}

fn main() {
    let uid:i32 = 101;
    println!("{}", get_user(uid));
    println!("{}", get_user2(uid));
}
