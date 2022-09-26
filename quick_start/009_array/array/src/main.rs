fn main() {
    let tags = ["java", "golang", "rust"];
    // let tags:[&str;3] = ["java", "golang", "rust"]; // same as above, complete written
    println!("{:?}", tags);

    // [0,3)
    for i in 0..tags.len() {
        println!("{:?}", tags[i]);
    }

    for item in tags.iter() {
        println!("{}", item);
    }


    let mut nums:[u8;10] = [1;10]; // set the initial value to 1
    for i in 0..nums.len() {
        nums[i] = (i+10) as u8;  // i is unsize type, need to transform type
    }
    println!("{:#?}", nums);

    // tuple's each value could have different type
    let my:(&str,u8)=("chengzw", 18);
    println!("{:#?}", my);
    println!("{:#?}", my.0); // get first value

    for (i, item) in tags.iter().enumerate() {
        println!("index is {}, value is {}", i, item);
    }
}
