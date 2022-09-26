#[derive(Debug)]
struct User {
    name: String,
    age: u8
}

impl User {
    fn version(&self){
        println!("1.0.0");
    }
    fn to_string(&self) -> String {
        return String::from(format!("my name is {}, my age is {}", &self.name, &self.age));
    }
}

fn main(){
    let me = User{name:String::from("chengzw"), age: 19};
    println!("name is {}, age is {}", me.name, me.age);
    me.version();
    println!("{}", me.to_string());
    println!("{:#?}", me); // ? will excute debug method, # means pretty print.
}