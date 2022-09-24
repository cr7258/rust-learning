```bash
> cargo run  
   Compiling penguins v0.1.0 (/Users/I576375/Code/rust-learning/rust_in_action/ch1/penguins)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/penguins`
debug: "    Little penguin,33" -> ["Little penguin", "33"]
Little penguin, 33cm
debug: "    Yellow-eyed penguin,65" -> ["Yellow-eyed penguin", "65"]
Yellow-eyed penguin, 65cm
debug: "    Fiordland penguin,60" -> ["Fiordland penguin", "60"]
Fiordland penguin, 60cm
debug: "    Invalid,data" -> ["Invalid", "data"]

// compile a release build that can eliminate the debug message                                                                                                                     
> cargo run --release
   Compiling penguins v0.1.0 (/Users/I576375/Code/rust-learning/rust_in_action/ch1/penguins)
    Finished release [optimized] target(s) in 0.25s
     Running `target/release/penguins`
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm

// reduce the output                                                                                                                        
> cargo run --release -q
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm
```