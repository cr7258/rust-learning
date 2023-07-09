fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    
    println!("\nfib_loop:");
    
    loop {
        next_fib(&mut a,&mut b);
        i += 1;
        
        println!("next val is {}", b);
        
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    
    println!("\nfib_while:");
    
    while i < n {
        next_fib(&mut a,&mut b);
        i += 1;
        
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    
    println!("\nfib_for:");
    
    for _i in 2..n {
        next_fib(&mut a,&mut b);
        println!("next val is {}", b);
    }
}

fn next_fib(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}