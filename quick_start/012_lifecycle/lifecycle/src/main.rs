fn max<'x>(a:&'x i32, b:&'x i32) -> &'x i32 {
    if a > b {
        a
    }else {
        b
    }
}

#[derive(Debug)]
struct User<'a> {
    id: &'a i32
}

fn show_name(name:&'static str) -> &'static str {
    name
}

fn main() {
    let a=12;
    let b=21;
    println!("max number is {}", max(&a, &b));

    let mut id=109;
    let u=User{id:&id};
    // id=108; // can't use id, because next line use u, id has been borrowed to u
    println!("{:?}", u);

    let name = "chengzw";
    show_name(name);
}
