/*
    Rust has 2 different types of constants which can be
    declared in any scope including global. Both require
    explicit type annotation:
    - Const: An unchangeable value (the common case)
    - Static: A possibly mut variable with "static" lfetime. 
    The static lifetime is inferred and does not have to be
    specified. Accessing or modifying a mutable static 
    variable is unsafe
*/

// Globals are declared outside all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access the constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"smal"});

    // Error! Cannot modify "const"
    // THRESHOLD = 5;
}