fn filter(html:&str){
    for c in html.chars() {
        println!("{}", c);
    }

    match html.len() {
        // [4,10]
        4..=10 => println!("{}", "ok"),
        // [0,3]
        0..=3 => println!("{}", "not ok"),
        // else
        _ => println!("{}", "good"),
    }

    // [1,10)
    for i in 1..10 {
        println!("{}", i);
    }
}
fn main() {
    let html = "abcd";
    filter(html);
}
