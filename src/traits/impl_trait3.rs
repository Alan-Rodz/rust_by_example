/*

    You can also use impl Trait to return an iterator
    that uses map or filter closures! 
    
    This makes using map and filter easier. 
    Because closure types don't have names, you 
    can't write out an explicit return type if 
    your function returns iterators with 
    closures. But with impl Trait you can do this easily:

*/

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
