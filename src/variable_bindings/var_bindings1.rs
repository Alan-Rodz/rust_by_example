fn main() {
    let an_integer = 1u32;
    let a_boolean = true; 
    let unit = ();

    let copied_integer = an_integer;
    
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?},", a_boolean);
    println!("The unit value: {:?}", unit);

    // Unused variables can be prefixed with "_" so that the compiler does not complain
    let _unused_variable =3u32;

    let noisy_unused_variable=2u32;
}