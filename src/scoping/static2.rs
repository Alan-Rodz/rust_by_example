/*

    As a trait bound, it means the type does not contain any non-static 
    references. Eg. the receiver can hold on to the type for as 
    long as they want and it will never become invalid until they drop it.

    It's important to understand this means that any owned data 
    always passes a 'static lifetime bound, but a reference to that
     owned data generally does not:

*/

use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
