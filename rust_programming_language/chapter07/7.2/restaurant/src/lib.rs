mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}


mod customer {

    // 方式一：将 use 移动到 customer 模块内
    // use 只能创建 use 所在的特定作用域内的短路径
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {

        hosting::add_to_waitlist();

        // 方式二：通过 super::hosting 引用父模块中的这个短路径
        super::front_of_house::hosting::add_to_waitlist();
    }
}