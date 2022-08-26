#### A process of learning rust

code with comment

- [Hello World!](./hello_world/README.md)
  - [Formatted print](./hello_world/formatted_print.rs)
    - `format!`: write formatted text to `String`
    - `print!`: same as `format!` but the text is printed to the console(io::stdout)
    - `println!`: same as `print!` but a newline is appended
    - `eprint!`: same as `print!` but the text is printed to the [standard error(io:stderr)](https://zh.m.wikipedia.org/zh-my/%E6%A8%99%E6%BA%96%E4%B8%B2%E6%B5%81)
    - `eprintln!`: same as `eprint!` but a newline is appended
  - [Debug](./hello_world/debug.rs)
  - [Display](./hello_world/display.rs)
- primitives
  - [literals_operators](./primitives/literals_operators/)
  - [tuples](./primitives/tuples/)
  - [arrays_slices](./primitives/arrays_slices/)
- custom types
  - [structures](./custom_types/strctures/)
  - [enums](./custom_types/enums/)
    - [use](./custom_types/use_declaration/)
    - [c-like](./custom_types/c_like/)
    - [linked-list](./custom_types/linked_list/)
