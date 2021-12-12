/*
    It is possible to declare a variable first and initialize them later
    However this is seldom used since it may lead to the use
    of uninitialized variables
*/  

fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x*x;
    }
    println!("A binding: {}", a_binding);
    let another_binding;

    // Error: Use of uninitialized binding
    // println!("Another binding: {}", another_binding);
    another_binding = 1;
    println!("Another binding: {}", another_binding);
}