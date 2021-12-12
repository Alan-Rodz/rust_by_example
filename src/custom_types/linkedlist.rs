// A common use case for enums is to create a linked list 

use crate::List::*;

enum List {
    // Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>), 

    // Nil: A node that signifies the end of the linked list
    Nil, 
}

// Methods can be attached to an enum
impl List {
    // Create an empty List
    fn new() -> List {
        // "Nil" has type "List"
        Nil 
    }

    // Consume a list, and re turn the same list with a new element at its front 
    fn prepend (self, elem: u32) -> List {
        // "Cons" also has type "List"
        Cons(elem, Box::new(self))
    }

    // Return the length of the list 
    fn len(&self) -> u32 {
        match *self {
            // Can't take ownership of the tail since "self" is borrowed
            // Instead we take a reference to the tail 
            Cons(_, ref tail) => 1 + tail.len(),

            // Base Case: Empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a heap allocated string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // "format!" is like "print!" but returns a heap allocated string 
                // instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },

            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("Linked List has length: {}", list.len());
    println!("{}", list.stringify());
}