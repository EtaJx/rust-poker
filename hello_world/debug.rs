// All `std` library types are automatically printable with `{:?}` too

// `struct` cannot be printed either with `fmt::Display` or with `fmt::Debug`

// the `derive` attribute automatically created the implementation
// required to make this `struct` printable width `fmt::Debug`
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // `{:?}` is similar to with `{}`
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    // if I want this to just show a `7`, can use `{:#?}` to 'pretty printing`
    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {:#?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}
