// if let can be used to match any enum value 
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable "a" matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable "b" does not match Foo::Bar so this prints nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value siilar to Some()
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with "if let"
    if let Foo::Qux(value @ 100) = c {
        println!("c is 100!");
    }
}
