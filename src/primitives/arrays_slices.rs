/*
    An array is a collection of objects of the same type T
    stored in contiguous memory. They're creted using 
    brackets []. Their length must be known at compile time, 
    and it is part of their type signature: [T; length]

    Slices are similar to arrays but their length is not
    known at compile time. Instead, a slice is a two word
    object. The first word is a pointer to the data and the
    second word is the length of the slice

    The word size is the same as usize, determined by the 
    processor architecture. E.g. 64 bits on an x64_64

    Slices can be used to borrow a section of an array, 
    and they have the type signature &[T]
*/

use std::mem; 

// Function that borrows a slice
fn analyze_slice(slice: &[i32]) {
    
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());

}

fn main() {
    
    // Fixed size array
    let xs: [i32; 5] = [1,2,3,4,5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0 
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // "len" returns the count of elements in the array 
    println!("Number of elements in the array: {}", xs.len());

    // Arrays are stack allocated
    println!("The array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can automatically be borrowed as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index .. ending index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice 
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bounds indexing causes compile error
    // println!("{}", xs[5]);
}