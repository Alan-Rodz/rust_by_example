/*
    Primitive types can be converted to each other
    through casting.

    Rust addresses conversion between custom types (i.e. struct
    and enum) by the use of traits. The generic conventions will
    use the From and Into traits. However, there are more specific ones
    for the most common cases, in particular when converting to
    and from Strings
*/

/*
    The "From" trait allows for a type to define how to 
    create itself from another type, hence providing a very
    simple mechanism for converting between several types
    
    There are numerous implementations of this trait within
    the standard library for conversion of primitive and 
    common types

*/

use std::convert::From;


#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item:i32)->Self {
        Number {value: item}
    }
}

fn main() {
    // We can easily convert str into string
    let my_str = "hello";
    let my_string = String::from(my_str);

    // We can do a similar thing to deifne our own type
    let num = Number::from(30);
    println!("My number is {:?}", num);
}