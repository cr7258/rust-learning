#[derive(Debug)]
// one struct multiple trait(prods,stock)
pub struct Book {
    pub id: i32,
    pub price: f32,
}

pub fn new_book(id:i32,price:f32) -> Book {
    Book { id, price } // shorthand
}

// move to api/prods.rs
// impl Book {
//     pub fn get_price(&self)->&f32 {
//         &self.price
//     }
// }