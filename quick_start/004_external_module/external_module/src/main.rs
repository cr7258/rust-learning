mod lib;
mod mymod;
fn main() {
    lib::helper::show_me();
    mymod::show_me();
    mymod::version::version();
}
