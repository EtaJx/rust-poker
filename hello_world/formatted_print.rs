fn main() {
    // `31 days`
    println!("{} days", 31);

    // `Alice this is Bob. Bob, this is Alice, `
    println!("{0} this is {1}. {1}, this is {0}, ", "Alice", "Bob");

    // `the quick brown fox jumps over the lazy dog`
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    // `1 of 10 people know binary, the other half don't`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // `     1` - 5 white spaces & `1`
    println!("{number:>width$}", number=1, width=6);

    // `000001`
    println!("{number:>0width$}", number=1, width=6);

    // Rust will checks to make sure the correct number of arguments are used
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Different formatting can invoked by specified format character after a `:`
    println!("Base 10 repr: {}", 69420);
    println!("Base 2 (binary) repr: {:b}", 69420);
    println!("Base 8 (octal) repr: {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    let pi = 3.141592;
    println!("Formatted Pi: {0:>.3}", pi);
    let formatted_pi = format!("{:.3}", pi);
    println!("Formatted string Pi: {}", formatted_pi);


    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
}
