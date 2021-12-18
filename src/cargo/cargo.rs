/*
    cargo is the official Rust package management tool. 
    It has lots of really useful features to improve code quality and 
    developer velocity! These include

    Dependency management and integration with crates.io 
    (the official Rust package registry)
    
    Awareness of unit tests
    
    Awareness of benchmarks

    This chapter will go through some quick basics, 
    but you can find the comprehensive docs in The Cargo Book. 
 
    The Rust ecosystem comes standard with cargo! cargo can manage dependencies 
    for a project.

    # A binary
    cargo new foo

    # OR A library
    cargo new --lib foo

    The main.rs is the root source file for your new project -- 
    nothing new there. The Cargo.toml is the config file for 
    cargo for this project (foo). If you look inside it, 
    you should see something like this:

    [package]
    name = "foo"
    version = "0.1.0"
    authors = ["mark"]

    [dependencies]


    The name field under [package] determines the name of the project. 
    This is used by crates.io if you publish the crate (more later). 
    It is also the name of the output binary when you compile.

    The version field is a crate version number using Semantic Versioning.

    The authors field is a list of authors used when publishing the crate.

    The [dependencies] section lets you add dependencies for your project.

    cargo also supports other types of dependencies. Here is just a small sampling:
    [package]
    name = "foo"
    version = "0.1.0"
    authors = ["mark"]

    [dependencies]
    clap = "2.27.1" # from crates.io
    rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
    bar = { path = "../bar" } # from a path in the local filesystem


    cargo is more than a dependency manager. 
    All of the available configuration options are listed in the format 
    specification of Cargo.toml.

    To build our project we can execute cargo build anywhere in the project 
    directory (including subdirectories!). We can also do cargo run to 
    build and run. Notice that these commands will resolve all 
    dependencies, download crates if needed, and build everything, 
    including your crate. 
    (Note that it only rebuilds what it has not already built, similar to make).

*/