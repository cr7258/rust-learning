#[macro_use]
mod mymacro;
fn main() {
    echo!();
    echo!("test");
    echo!(3==4);
    echo!("a", "b", "c");

    func!(golang);
    golang();
}
