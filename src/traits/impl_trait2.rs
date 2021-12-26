/*

    More importantly, some Rust types can't 
    be written out. For example, every 
    closure has its own unnamed concrete type. 
    Before impl Trait syntax, you had to 
    allocate on the heap in order to return a 
    closure. But now you can do it all 
    statically, like this:

*/

// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
