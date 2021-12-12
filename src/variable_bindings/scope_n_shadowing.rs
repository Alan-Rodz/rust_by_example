/*
    Variable bindings have a scope and they're constrained to
    live in a block. A block is a collection of statements
    enclosed by braces "{}"
*/

fn main() {
    let long_lived_binding = 1;

    // This block has a smaller scope than the main function
    {
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);
    }

        // Error! `short_lived_binding` doesn't exist in this scope
        // println!("outer short: {}", short_lived_binding);
        // FIXME ^ Comment out this line
    
        println!("outer long: {}", long_lived_binding);

    // Variable shadowing is allowed
    let shadowed_binding = 1;
    {
        println!("Before being shadowed: {}", shadowed_binding);

        // This binding shadows the outer one
        let shadowed_binding = "abc";
        println!("Shadowing in inner block: {}", shadowed_binding);
    }

    println!("Outside inner block: {}", shadowed_binding);

    // This binding shadows the previous one
    let shadowed_binding = 2;
    println!("Shadowed in outer block: {}", shadowed_binding);
}