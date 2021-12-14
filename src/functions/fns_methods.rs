/*
 * Associated functions that are functions that are 
 * defined on a type generally
 * 
 * Methods are associated functions that are called on a 
 * particular instance type  
 * 
 */

 struct Point {
     x: f64, 
     y: f64
 }

 // Implementation block
 // All "Point" associated functions and methods go here
 impl Point {

    // Associated function cause its associated to the Point type
    // Associated fns dont need to be called with an instance
    // These fns are generally used like constructurs
    fn origin() -> Point {
        Point {x:0.0, y:0.0}
    }

    // Associated function that takes 2 arguments
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
    
 }

 struct Rectangle {
     p1: Point, 
     p2: Point,
 }

 impl Rectangle {
     // &self is sugar for self:&Self where Self is the type of the caller object
     // In this case, Self = Rectangle
     fn area(&self) -> f64 {
         // self gives access to the struct fields through dot operator
         let Point {x: x1, y: y1} = self.p1;
         let Point {x: x2, y: y2} = self.p2;

         // abs is an f64 method that returns the absolute value of the caller
         ((x1-x2)*(y1-y2)).abs()
     }

     fn perimeter(&self) -> f64 {
         let Point {x: x1, y: y1} = self.p1;
         let Point {x: x2, y: y2} = self.p2;

         2.0 * ((x1-x2).abs() + (y1-y2).abs())
     }

     // This method requires the caller object to be mutable
     // &mut self desugars into "self: &mut Self"
     fn translate(&mut self, x:f64, y:f64) {
         self.p1.x += x;
         self.p2.x += x;

         self.p1.y += y;
         self.p2.y += y;
     }
 }

 // "Pair" owns two heap allocated integers
 struct Pair(Box<i32>, Box<i32>);

 impl Pair {
     // This method consumes the resources of the caller object
     // "self" desugars into "self: Self"
     fn destroy(self) {
         // Destructure "self"
         let Pair(first, second) = self; 
         println!("Destroying Pair({}, {})", first, second);

         // "first" and "second" go out of scope and get freed
     }
 }

 fn main() {
     let rectangle = Rectangle {
         // Associated functions are called through double colons
         p1: Point::origin(),
         p2: Point::new(3.0, 4.0),
     };

     // Methods are called using the dot operator
     // Note that the first argument, "&self" is implicitly passed
     // rectangle.perimeter() === Rectangle::perimeter(&rectangle)
     println!("Rectangle perimeter: {}", rectangle.perimeter());
     println!("Rectangle area: {}", rectangle.area());

     let mut square = Rectangle {
         p1: Point::origin(),
         p2: Point::new(1.0, 1.0),
     };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    //rectangle.translate(1.0, 0.0);

    // Mutable objects can call mutable methods
    square.translate(1.0, 1.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();
 }