macro_rules! echo {
    () => {
        println!("chengzw");
    };
    ($exp:expr) => {
        println!("{}",$exp);
    };
    // stringify! means don't parse expression
    // ($abc:expr) => {
    //     println!("{}",stringify!($abc));
    // }
    // variable parameter
    // + means at least one, * 0~infinity
    ($($exp:expr),+) => {
        $(
            println!("{}",stringify!($exp));
        )+
    }
}

macro_rules! func {
    ($fn_name:ident) => {
        fn $fn_name(){
            println!("my function name is {}", stringify!($fn_name));
        }
    };
}

