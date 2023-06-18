fn main() {
    // Integer Types
    // println!("{}", (-4).abs()); //  can't call method `abs` on ambiguous numeric type `{integer}`
    // 以下 3 种方式都可以，显式指定了 -4 的类型
    let num: i32 = -4;
    println!("{}", num.abs());

    println!("{}", (-4_i32).abs());

    println!("{}", i32::abs(-4));

    // Floating-Point Types
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    // The bool Type
    assert_eq!(false as i32, 0); 
    assert_eq!(true  as i32, 1);

    // Characters
    assert_eq!('*' as i32, 42);
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('8'.is_alphanumeric(),true);

    // Tuples
    let text = "I see the eigenvalue in thine eye"; 
    let (head, tail) = text.split_at(21); 
    assert_eq!(head, "I see the eigenvalue "); 
    assert_eq!(tail, "in thine eye");

    // Pointer Types
    let a = 32;
    let b = &a;
    let c = *b + 1;
    println!("{}", c);

    // Boxes
    // The simplest way to allocate a value in the heap is to use Box::new
    let t = (12, "eggs");
    let u = Box::new(t); // allocate a tuple in the heap
    println!("{}", u.1);

    // Arrays, Vectors, and Slices
    // Arrays
    let lazy_caterer: [u32; 6] = [1,2,4,7,11,16];
    assert_eq!(lazy_caterer[3],7);
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]); 
    assert!(!sieve[9876]);

    let mut chaos = [3,5,4,1,2];
    // the sort method is actually defined on slices, but since it takes its operand by reference, 
    // Rust implicitly produces a &mut [i32] slice referring to the entire array and passes that to sort to operate on.
    chaos.sort(); 
    assert_eq!(chaos,[1,2,3,4,5]);

    // Vector
    let mut primes = vec![2,3,5,7];
    primes.push(11);
    primes.push(13);
    println!("{:?}", primes);

    let p = vec![0; 5];
    println!("{:?}", p);

    // The vec! macro is equivalent to calling Vec::new to create a new, empty vector 
    // and then pushing the elements onto it, which is another idiom
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("on");
    pal.push("pets");
    println!("{:?}", pal);

    // Another possibility is to build a vector from the values produced by an iterator:
    let q:Vec<i32> = (0..5).collect();
    println!("{:?}", q);


    // A Vec<T> consists of three values: 
    // a pointer to the heapallocated buffer for the elements, which is created and owned by the Vec<T>
    // capacity: the number of elements that buffer has the capacity to store
    // len: the number it actually contains now

    // create a vector with a buffer
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(),0);
    assert_eq!(v.capacity(),2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(),2);
    assert_eq!(v.capacity(),2);
    v.push(3);
    assert_eq!(v.len(),3);
    println!("capacity is now {}", v.capacity());

    // insert element into vector
    let mut x = vec![10,20,30,40,50];
    // Make the element at index 3 be 35.
    x.insert(3, 35); 
    assert_eq!(x, [10, 20, 30, 35, 40, 50]);  
    // Remove the element at index 1.
    x.remove(1); 
    assert_eq!(x, [10, 30, 35, 40, 50]);

    // You can use the pop method to remove the last element and return it.
    let mut c = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(c.pop(), Some("Glass Gem"));
    assert_eq!(c.pop(),Some("Snow Puff"));
    assert_eq!(c.pop(),None);

    // use a for loop to iterate over a vector
    let languages = vec!["Chinese", "Japanese", "English"];
    for l in languages {
        println!("{}", l);
    }

    // Slices
    let h: Vec<f64> = vec![0.0,0.707,1.0,0.707];
    let a:[f64;4] = [0.0,-0.707,-1.0,-0.707];
    // You can get a reference to a slice of an array or vector, 
    // or a slice of an existing slice, by indexing it with a range:
    print(&h[0..2]);
    print(&a[2..]);

    // String Types
    // String literals are enclosed in double quotes. They use the same backslash escape sequences as char literals:
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}",speech);
    println!("In the room the women come and go,
         Singing of Mount Abora");

    // If one line of a string ends with a backslash, then the newline character and the leading whitespace on the next line are dropped
    println!("It was a bright, cold day in April, and \
         there were four of us—\
              more or less.");

    // Byte Strings
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    println!("{}", oodles);

    // There are several ways to create Strings
    // The .to_string() or .to_owned() method converts a &str to a String. This copies the string
    let error_message = "too many pets".to_string();
    println!("{}", error_message);

    // The format!() macro
    assert_eq!(format!("{}",24), "24".to_string());

    // Arrays, slices, and vectors of strings have two methods, .concat() and .join(sep), that form a new String from many strings:
    let bits = vec!["veni","vidi","vici"];
    assert_eq!(bits.concat(),"venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
}

fn print(n: &[f64]){
    for elt in n {
        println!("{}", elt);
    }
}