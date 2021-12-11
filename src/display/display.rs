// We can import stuff through the "use" keyword
use std::fmt;

// Define a structure for which "fmt::Display" will be imlpemented 
struct Structure(i32);

// To use "{}", "fmt::Display" will must be implemented manually 
impl fmt::Display for Structure {

    // This trait requires "fmt" with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Write strictly the first element into the supplied output
        // stream: "f". Returns "fmt::Result", which indicates whether the 
        // operation succeeded or failed. Notice that "write" uses syntax
        // which is very similar to "println!"
        write!(f, "{}", self.0)

    }

}

fn main() {

   let seven = Structure(7);
    println!("{}", seven);
}
