fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 对于第二个条件 Some(y)，它可以匹配任何 Some，并将 Some 中的值绑定到一个新的变量 y
        Some(y) => println!("Matched, y = {y}"),
        // 如果你想在第二个匹配分支中匹配某个变量可以这样写
        //  Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);


    // @ 运算符允许为一个字段绑定另外一个变量。
    // 下面例子中，我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，
    // 同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // 如果不用 @ 运算符将一个字段绑定另外一个变量，可以这样写
        // Message::Hello {
        //     id: id_variable,
        // } if (3..=7).contains(&id_variable) => {
        //     println!("Found an id in range: {}", id_variable);
        // }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
