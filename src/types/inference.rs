/*
    The type inference engine looks at how the variable is
    used afterwards to infer its type. Below is an example
    of type inference
*/

fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}",vec);
}