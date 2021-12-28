/*

    In the unsuccessful case, parse() 
    leaves us with an error for unwrap() 
    to panic on. Additionally, 
    the panic exits our program and 
    provides an unpleasant error message.

    To improve the quality of our error 
    message, we should be more specific 
    about the return type and 
    consider explicitly handling the error.


    The Result type can also be the return 
    type of the main function if 
    specified explicitly. 
    Typically the main function 
    will be of the form:

    fn main() {
        println!("Hello World!");
    }

    However main is also able to have a return 
    type of Result. If an error occurs within 
    the main function it will return an error 
    code and print a debug representation of 
    the error (using the Debug trait). 
    
    The following example shows such a 
    scenario and touches on aspects covered 
    in the following section.

*/


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
