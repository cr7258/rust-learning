fn main() {
    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = ||list.push(7);
    // 当 borrows_mutably 定义时，它捕获了 list 的可变引用
    // 因为当可变借用存在时不允许有其它的借用，所以在闭包定义和调用之间不能有不可变引用来进行打印
    // println!("Before calling closure: {:?}", list);

    borrow_mutably();
    print!("After calling closure: {:?}", list);
}
