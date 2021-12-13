/*
    The "loop" keyword indicates an infinite loop

    The "break" statement can be used to exit the loop at 
    anytime, whereas the "continue" statementc an be used to 
    skip the rest of the iteration and start a new one

*/

fn main() {
    let mut count = 0u32;

    println!("Lets count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        
        if count == 3 {
            println!("three!");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("Thats enough!");
            break;
        }
    }
}