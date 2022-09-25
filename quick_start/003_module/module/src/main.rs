mod lib{
    pub fn show_me(){
        let my_name = "chengzw";
        let my_age = 19;
        println!("my name is {}, my age is {}", my_name, my_age);
    }
}

fn main(){
    lib::show_me();
}
