use std::io::SeekFrom::Start;
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。
// 因为在 MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
// 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
// Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = MyBox::new(x);
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 如果 Rust 没有实现 Deref 强制转换，为了使用 &MyBox<String> 类型的值调用 hello，则不得不编写以下代码来实现
    // (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名。
    // hello(&(*m)[..]);
}
