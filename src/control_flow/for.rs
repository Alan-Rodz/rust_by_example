/*
  
    The "for in" construct can be used to iterate through an
    Iterator. One of the easiest ways to create an iterator is
    to use the range notation, a..b. This yields the values from
    a (inclusive) to b (exclusive) in steps of one

    a..=b can be used for a range that is inclusive in both ends

    The "for in" construct is able to interact with an Iterator
    in several ways. By default the for loop will apply the 
    into_iter function to the collection, but this is not the 
    only means of converting collections into iterators

    into_iter, iter and iter_mut all handle the conversion of a
    collection into an iterator in different ways, by providing
    different views on the data within

    iter borrows each element of the collection through each 
    iteration, thus leaving the collection untouched and available
    for reuse after the loop

    into_iter consumes the collection so that on each iteration
    the data is provided. Once the collection has been consumed
    it is no longer available for reuse, as it has been
    "moved" within the loop

    iter_mut mutably borrows each element of the collection, 
    allowing for the collection to be modified in place

*/

fn main() {
    // a..b
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // a..=b
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // iter
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    
    // into_iter
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // iter_mut
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

}