/*
    Closures are fns that can capture the enclosing environment 

    Calling a closure is like calling a function, but both input and return
    types can be inferred and input variable names must be specified

    They also allow us to:
    - Use || instead of () around input variables
    - Optional body delimination ({}) for a single expression (mandatory otherwise)
    - the ability to capture the outer environment variables

*/

fn main() {
    // Increment via closures and functions
    fn function(i:i32) -> i32 {i+1}

    // Closures are anonymous
    let closure_annotated = |i:i32| -> i32 {i+1};
    let closure_inferred = |i| i+1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}