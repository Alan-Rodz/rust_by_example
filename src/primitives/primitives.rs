/* 
 * Signed integers: i8, i16, i32, i64, i128, isize (pointer size)
 *
 * Unsigned integers: u8, u16, u32, u64, u128, usize (pointer size)
 *
 * Floating point: f32, f64
 *
 * char Unicode scalar values (4 bytes each)
 *
 * bool, either true or false 
 *
 * The unit type "()", whose only possible value is an empty tuple: (). Even though it 
 * is a tuple, it is not considered a compound type since it does not contain multiple 
 * values
 *
 * Compound types are arrays like [1,2,3] and tuples like (1, true)
 *
 * Variables can always be "type annotated". Numbers may additionally be annotated via a 
 * suffix or by default. Integers default to i32 and floats to i64.
 * Rust can infer types from context 
 *
 */

fn main () {

    // Variables can be type annotated
    let logical: bool = true; 
    let a_float: f64 = 1.0;              // Regular Annotation
    let an_integer   = 5i32;            // Suffix annotation 
    println!("{}", logical);
    println!("{}", a_float);
    println!("{}", an_integer);

    // Or a default will be used
    let default_float = 3.0;            // f64
    let default_integer = 7;            // i32
    println!("{}", default_float);
    println!("{}", default_integer);

    // A type can also be inferred from context
    let mut inferred_type = 12;          // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    // A mutable variable's value can be changed
    let mut mutable = 12;               // Mutable i32 
    mutable = 21;
    println!("{}", mutable);

    // The type of the value can't be changed
    // mutable = true;
    
    // Variables can be overwritten with shadowing
    let mutable = true; 
    println!("{}", mutable);
}

