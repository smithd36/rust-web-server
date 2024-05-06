# Tips
Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It aims to bring modern language design and an advanced type system to systems programming. Rust does not use a garbage collector, using advanced static analysis to provide deterministic drops instead. It accomplishes this via the concept of ownership.

Some commands to get started with Rust:
Powershell

- rustc - compiles rust code to the binary of current OS
    - `rustc --version`
        * check the version of rust

    - cargo - rust package manager
    `cargo --version`
        * check the version of `cargo`
    - Create new rust project with cargo:
        * `cargo new <project_name>`

Rust requires semicolons `;` at the end of every line of code, unless there is only one line in a block.

There is a shortcut in Rust called `rustfmt` that formats the Rust code:
    - e.g. Removes unecessary spaces...

Rust is a statically and strongly typed programming language. What that means is when we define a variable, it is going to be given <i>some sort of</i> type. We can <b>explicitly</b> define that type by definiing the type manually, or we can let the compiler implicitly decide the type at runtime.
    - For example: `let myVar = 6;` is an <i>implicitly</i> typed assignment because there is not an explicit specification of `myVar`'s type. At run-time, it is going to see that `myVar` is an `int`, and will implicitly define `myVar` as type `int`.
