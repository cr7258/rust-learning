fn main() {
    let mut tags = vec!("php", "java");
    tags.push("go");
    println!("{:?}", tags);

    for i in &tags { // value borrowed here after move
        println!("{:?}", i);
    }
    println!("{:?}",tags.len());

    // another initialize method
    let mut tags1 = Vec::new();
    tags1.push("js");
    tags1.push("php");
    for i in 0..tags1.len() {
        println!("{:?}",tags1[i]);
    }

    let mut tags2 = Vec::new();
    tags2.push(1);
    tags2.push(2);

    for i in &mut tags2 {
        *i += 10;
    }
    println!("{:?}",tags2);
}
