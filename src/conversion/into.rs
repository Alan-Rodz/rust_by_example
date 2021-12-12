/*
    The "Into" trait is simply the reciprocal of the "From" trait.
    That is, if you've implemented the From trait for your 
    type, Into will call it when necessary

    Using the Into trait will typically require specification of
    the type to convert into as the compiler is unable to determine 
    this most of the time. However this is a small trade-off considering
    we get the functionality for free
*/

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

fn main() {
    let int = 5;

    let num: Number = int.into();
    println!("My number is: {:?}", num);
}