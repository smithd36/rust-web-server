use std::io; // the input/output module for console

fn main() {
    println!("Hello, world!");

    println!("VARIABLES:");

    // Example 1: Normal
    let x = 4;
    println!("X is: {}", x);

    let y = 6;
    println!("Y is {}", y);
    println!("Sum is {}", x + y);

    // Example 2: Name Shadowing (Make our own Scope)
    {
        let x = 2;
        println!("x is (example scope 1): {}", x)
    }

    // What is the output betweeb Ex. 1 & 2 if ran at same time? ^^^
    // This proves that {} are used to 'scope' an interior scope to the code

    {
        // Vars ffrom exterior scope are accessible from interior scope
        let x = x - 1;
        println!("X is now: (in 2nd scope where x -= 1): {}", x);
    }

    // Are vars from interior scope visible from exterior after execution?
    // Let's see
    // This sets type as 32 bit integer explicitly
    // unsigned (u32) cannot have a sign in front such as a negative sign (-)
    let x = x - 1;
    println!("X on exterior after interior modification execution?: {}", x);

    // last example for variables the types are Changeable
    let x = "i am x! now a string";
    println!("X is now a string! Types can be changed if explicitly specified: {}", x);

    //END OF VARIABLES
    
    
    println!("\nCONSTANTS:");
    // constants are not variables, once the value and type of a const is defined, it cannto
    // be changed throughout the program's entirety
    const SECONDS_IN_MINUTE: i32 = 60;
    println!("This is a constant of type i32: {}", SECONDS_IN_MINUTE);

    const FLOAT32: f32 = 24.019;
    println!("This is a constant of type FLOAT32: {}", FLOAT32);

    // Console input
    // Devlare a mutable input str
    // mut is mutable or changeable
    let mut input = String::new();

    println!("\n--Type favorite food then [ENTER]: ");
    io::stdin().read_line(&mut input).expect("failed to read line"); // a mutable reference, which allows direct modification to the var from input

    println!("----Your input: \n------    {}", input);

    println!("\nCONDITIONS:\n");

    let cond = (2 as f32) <= 2.2;
    println!("Conditional Boolean return to 2 as f32 <= 2.2: {}", cond);

    // If both true, return true, else false
    let cond2 = true && cond;
    println!("Cond for true && above result: {}", cond2);

    println!("if one or more is true, return true, else false with ||");
    let cond3 = !(true || !cond);
    println!("If one of more is true in: let cond3 = !(true || !cond): {}", cond3);

    println!("If statement is: if food == cookies");
    if input.trim() == "cookies" {
        println!("I like {}too!", input);
    } else {
        println!("Oh food is not cookies! It is: {}.\nyou suck!", input);
    }

    println!("\n\nFunctions!:");
    // does not matter if ran above or below function definition block!!
    first_function();
    fn first_function() {
        println!("just ran the first_function!");
    }
    first_function();

    fn add_numbers(x: i32, y:i32) {
        println!("Adding {} and {}", x, y);
        println!("The sum is: {}: ", x + y);
    }

    add_numbers(99, 37);

    println!("\nNOTE: Statements vs. expressions.\n Rust can returns statements, but NOT expressions.");

    println!("\nNote: An expression is anything that returns or has a value.\n");

    // This is an expression that evals to x = 4.
    let number = {
        let x = 3;
        x + 1
    };

    println!("Number expression at line 104 is {}", number);

}