#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);

    // 或者使用 dbp! 宏，还可以打印出代码所在的文件和行号
    dbg!(&rect1);
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}