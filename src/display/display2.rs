use std::fmt;

// Structure holding two numbers
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement "Display" for "MinMax"
impl fmt::Display for MinMax {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        // Use "self.number" to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    
    }
}

// Define a structure where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Implement "Display" for "Point2D"
impl fmt::Display for Point2D {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        
        // Customize so that only "x" and "y" are denoted
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Complex number structure
#[derive(Debug)]
struct ComplexNumber {
    real_part: f64,
    complex_part: f64,
}

// Implement "Display" for "ComplexNumber"
impl fmt::Display for ComplexNumber {


    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{} + {}i", self.real_part, self.complex_part)

    }

}

fn main () {

    let minmax = MinMax(0,14);
    
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big} and the small range is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D {x:3.3, y:7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex_number = ComplexNumber {real_part: 3.5, complex_part: 3.8};
    println!("Compare Complex Numbers:");
    println!("Display: {}", complex_number);
    println!("Debug: {:?}", complex_number);

}

