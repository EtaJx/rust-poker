// Create an `enum` to classify a web event.
// Note how bothname ands type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`
// Each is different and independent
enum WebEvent {
    // An `enum` may either be `unit-like`
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

// Type aliases
// use a type alias, you can refer to each enum variant via its alias.
// this might be useful if the enum's name is to long or to generic, and you want to rename it.
enum VeryVerboseEnumOfTheingsToDoWithNumbers {
    Add,
    Subtract,
}

// `impl` blocks using the `Self` alias
impl VeryVerboseEnumOfTheingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfTheingsToDoWithNumbers;

// A function which takes a `WebEvent` enum as an argument and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the enum
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click {x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

// fn insepct_type_alias(event: Operations) {
//     match event {
//         Operations::Add => println!("Operation is Add"),
//         Operations::Subtract => println!("Operation is Subtract"),
//     }
// }

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("My Text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    // Type aliases
    // we can refer to each variant via its alias, not its long and inconvernient name.
    let a = Operations::Add.run(1, 2);
    let b = Operations::Subtract.run(2, 1);
    println!("Operations results {}, {}", a, b);

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
