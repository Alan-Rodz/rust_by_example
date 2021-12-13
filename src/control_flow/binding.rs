/*
 * Indirectly accessing a variable makes it impossible to
 * branch and use that variable without rebinding. Match
 * provides the @ sigil for binding variables to names
 */

// You can also use binding to destructure enum variants

fn age() -> u32 {
    15
}


/*
    The option<T> enum has 2 variants:
    None, to indicate failure or lack of value
    Some(value), a tuple struct that wraps a value with type T
*/
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("You've not celebrated your first birthday yet"),

        // We match the sequence of 1..= 12
        n @ 1..=12 => println!("You're a child of age {:?}", n),

        // Then the sequence of 13..=19
        n @ 13..=19 => println!("You're a teen of age {:?}", n),

        n => println!("Youre a person of age {:?}", n),
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}
