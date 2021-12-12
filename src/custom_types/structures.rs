/*
    struct allows us to define structures
    enums allow us to define enumerations
*/

/*
    There are 3 types of structures that can be created using
    the "struct" keyword:
        - Tuple structs, which are named tuples
        - The classic C structs
        - Unit structus, which are field less and useful for generics
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// Tuple struct
struct Pair(i32, f32);

// Struct with 2 fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are in space
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(input_rectangle: Rectangle) -> f32 {
    // top left, bottom right, top right, bottom left
    let (x1, y1): (f32, f32) = (input_rectangle.top_left.x, input_rectangle.top_left.y);
    let (x2, y2): (f32, f32) = (
        input_rectangle.bottom_right.x,
        input_rectangle.bottom_right.y,
    );
    let (x3, y3): (f32, f32) = (x2, y2 + (y1 - y2));
    let (x4, y4): (f32, f32) = (x1, y1 - (y1 - y2));

    let base: f32 = ((x3 - x1).powf(2.0) + (y3 - y1).powf(2.0)).sqrt();
    let height: f32 = ((x4 - x1).powf(2.0) + (y4 - y1).powf(2.0)).sqrt();

    let rectangle_area: f32 = base * height;
    rectangle_area
}

/*
    Add a function square which takes a Point and a f32 as arguments,
    and returns a Rectangle with its lower left corner on the point,
     and a width and height corresponding to the f32.
*/

fn square(input_point: Point, input_float: f32) -> Rectangle {
    // bottom left, top left, bottom right
    let (x1, y1): (f32, f32) = (input_point.x, input_point.y);
    let (x2, y2): (f32, f32) = (x1, y1 + input_float);
    let (x3, y3): (f32, f32) = (x1 + input_float, y1);


    let new_rectangle = Rectangle {
        top_left: Point { x: x2, y: y2 },
        bottom_right: Point { x: x3, y: y3 },
    };

    new_rectangle
}

fn main() {
    // Create a struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a "Point"
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("Point Coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax
    let bottom_right = Point { x: 5.2, ..point };
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity
    let rect = Rectangle {
        top_left: Point { x: 2.0, y: 4.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };
    println!("The area of the rectangle is: {}", rect_area(rect));

    let squar = square(Point { x: 1.0, y: 1.0 }, 5.0);
    println!("The area of the square is: {}", rect_area(squar));
}
