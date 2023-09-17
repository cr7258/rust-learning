use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let username = read_username_from_file().expect("Failed to read username from file");
    println!("username: {:?}", username);
}

fn read_username_from_file() -> Result<String, OurError> {

    // 方式一
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // 方式二：使用 ? 运算符简化表达式
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // 方式三：可以在 ? 之后直接使用链式方法调用来进一步缩短代码
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

#[derive(Debug)]
enum OurError {
    Io(io::Error),
}

// ? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
// 我们可以定义 impl From<io::Error> for OurError 来从 io::Error 构造一个 OurError 实例
impl From<io::Error> for OurError {
    fn from(err: io::Error) -> OurError {
        println!("构建自定义的错误类型");
        OurError::Io(err)
    }
}
