/*
 * format! writes formatted text to String
 * print! is the same as format! but the text is printed to the console (io::stdout)
 * println! is the same as print! but a newline is appended
 * eprint! is the same as format! but the text is printed to the standard error (io::stderr)
 * eprintln! is the same as eprint! but a new line is appended
 * Rust checks formatting correctness at compile time
 * 
 * std::fmt contains many traits which govern the display of text. The base form of 2 
 * important ones are listed below:
 * fmt::Debug uses the {:?} marker. Format text for debugging purposes.
 * fmt::Display uses the {} marker. Format text in a more elegant, user friendly fashion.
 * Implementing the fmt::Display trait automatically implements the ToString trait, which 
 * allows us to convert the type to String
 */

fn main () 
{
    // {}s will be replaced with any arguments, which will be stringified
    println!("{} days", 31);

    // This works with various other optional patterns. Positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments can also be used 
    println!("{subject} {verb} {object}", 
             object = "the lazy dog", 
             subject= "the quick brown fox", 
             verb = "jumps over");
    
    // Special formatting can be specified after a `;`
    // In this example, 2 appears as binary (:b)
    println!("{} of {:b} people know binary. The other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output 5 white spaces and a 1
    // Here we're using ":>" and "$" to specify the width
    println!("{number:>width$}", number=1, width=6);

    // Creates a structure named `Structure` which contains an i32
    #[allow(dead_code)]
    struct Structure(i32);

}


