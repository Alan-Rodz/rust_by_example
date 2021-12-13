/*
    Rust's match is like C's switch

    The first matching arm is evaluated and all possible values
    must be covered  
*/

fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),

        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),

        13..=19 => println!("A teen!"),

        _ => println!("We don't care!"),
    }

    let boolean = true;
    
    // Match is an expression too
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}