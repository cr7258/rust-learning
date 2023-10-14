pub trait Draw {
    fn draw(&self);
}

// trait 对象
// 泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
// 这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。如果
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 为什么在 Vec<Box<dyn Draw>> 中需要使用 Box，原因在于 trait 对象的大小不是在编译时确定的，
// 因此不能直接存储在 Vec 中。Rust 中的 Vec 需要元素具有相同的大小，以便进行有效的内存布局和管理。
// 而 Box 允许你在堆上存储具体类型的对象，并且在 Vec 中存储指向堆上的 Box 的指针，这使得 Vec 中的元素都具有相同的大小。

// 具体来说，Vec<Box<dyn Draw>> 允许你在 Vec 中存储指向实现 Draw trait 的不同类型对象的堆分配内存的指针（Box），
// 而这些 Box 具有相同的大小，因此可以有效地存储在 Vec 中。

// 如果你尝试直接使用 Vec<dyn Draw>，编译器会报错，因为 dyn Draw 的大小是在运行时确定的，
// 而 Vec 需要在编译时知道元素的大小。所以，为了在 Vec 中存储 trait 对象，你需要使用 Box 或 Rc 这样的堆分配类型来固定大小，以便 Vec 能够正确地管理内存。


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 泛型类型参数
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
//     where
//         T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw for button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw for selectbox");
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Hi"))  // the trait `Draw` is not implemented for `String`
        ],
    };

    screen.run();
}