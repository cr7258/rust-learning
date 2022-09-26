fn main() {
    let mut name = String::new();
    name.push_str("chegnzw");
    println!("{}", name);

    let first_name = String::from("cheng");
    let last_name = String::from("zw"); 
    // let full_name = first_name + &last_name + "_good"; // concatenate two string, the second one must be &str type.  &String => &str
    // println!("{}", full_name);

    // same as above
    let format_name = format!("{}{}", first_name, last_name);
    println!("{}", format_name);

    let mystr = String::from("abc");
    println!("length is {}, capacity is {}, ptr is {:p}", mystr.len(), mystr.capacity(), name.as_ptr()); // {:p} print pointer

    let me = String::from("chengzw");
    let you = me.clone(); // let you = me will cause error: value borrowed here after move
    //let you = &me; // create quote point to value me, the ownership doesn't change, it's ok as well
    println!("ptr is {:p}", me.as_ptr());
    println!("ptr is {:p}", you.as_ptr());

    let mut youstr = String::from("chengzw");
    change(&mut youstr);
    println!("{}", youstr);
}

fn change(s:&mut String){
    s.push_str("_19");
}
