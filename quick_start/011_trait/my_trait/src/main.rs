mod models;
mod api;
use models::Book;
use models::Phone;
use api::Prods;
use api::Stock;

fn show_prod<T:Prods>(p:T){
    println!("the price of product is {}",p.get_price());
}

fn sum<T:Prods,U:Prods>(p1:T,p2:U) {
    println!("the total price of product is {}",p1.get_price() + p2.get_price());
}

fn sum1(p1:Book, p2:Book) {
    println!("the total price of product is {}", p1+p2);
}

// fn show_detail<T:Prods+Stock>(p:T){
//     println!("the stock of product is {}", p.get_stock());
// }

// same as above
fn show_detail<T>(p:T) 
where T:Prods+Stock {
    println!("the stock of product is {}", p.get_stock());
}

fn main() {
    // let book = new_book(101,25.0);
    // you mush declare type
    let book:Book = Prods::new(101,25.0);
    println!("{:?}",book.get_price());
    show_prod(book);

    let book1:Book = Prods::new(102,20.6);
    let phone1:Phone = Prods::new(103,1300.6);
    sum(book1,phone1);

    let book2:Book = Prods::new(101,20.6);
    show_detail(book2);

    let book3:Book = Prods::new(104,50.1);
    let book4:Book = Prods::new(105,100.2);
    sum1(book3,book4);

}
