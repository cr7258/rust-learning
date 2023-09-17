fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，
// 所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
