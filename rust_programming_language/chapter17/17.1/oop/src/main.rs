#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: i32
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> i32 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total:i32 = self.list.iter().sum();
        self.average = total / self.list.len() as i32;
    }
}

fn main() {
    let mut num = AveragedCollection {
        list: vec![1,2,3],
        average: 2,
    };
    num.add(10);
    num.add(20);
    num.add(30);
    num.remove();

    println!("{:?}", num.average)
}