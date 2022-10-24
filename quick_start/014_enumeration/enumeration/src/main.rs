#[derive(Debug)]
enum Sex {
    Male,
    Female
}

#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex,
}

fn check(u:User) {
    // match u.sex {
    //     Sex::Male => {
    //         println!("{}", "male");
    //     },
    //     Sex::Female => {
    //         println!("{}", "female");
    //     }
    // };
    
    // same as above
    if let Sex::Male = u.sex {
        println!("male");
    }else {
        println!("female");
    }
}



fn main() {
    let u = User {
        id: 101,
        sex: Sex::Male
    };
    check(u);
}
