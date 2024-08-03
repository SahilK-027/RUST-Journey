// Basic variable declaration
/**
 * You can define variables using the `let` keyword
 * You can assign the type of variable, or it can be `inferred` as well.
 */

fn type_number() {
    let num: i32 = 10; // We have assigned type to as signed integer of 32 bit
    let x = 42; // The type of `x` is inferred to be `i32` (an integer)
    let y: u32 = 20;
    let z: f32 = 20.1;

    // let a: i8 = 1024; // Overflow

    // In Rust, the print! macro uses formatting specifiers to determine how to print the provided arguments.
    // The correct way to use print! is to include a formatting specifier within the string, such as {} for a placeholder.
    print!("x: {}, y: {}, z: {}", x, y, z);
}

fn type_boolean() {
    let is_raining = true;
    let mut is_sunny = false;

    // is_raining = false; // You cannot do this all variables in rust are immutable by default
    is_sunny = true; // you can do this because is_sunny is mutable because of `mut` keyword

    // Use booleans in an if statement
    if is_raining {
        println!("Don't forget your umbrella!");
    } else if is_sunny {
        println!("It's a beautiful day!");
    } else {
        println!("Weather is unpredictable today.");
    }
}

fn type_string() {
    // Mutability
    // Immutable &str: String slices (&str) are immutable, meaning you cannot change the content they point to.
    // This immutability requires careful handling when you need to modify strings.
    //
    // Mutable String: The String type is mutable, but operations that modify the string (like appending or inserting)
    // can be complex due to the need to manage dynamic memory allocation and potential reallocation.

    // Theory
    // Strings are considered more complex in Rust (and in general) for several reasons:
    // Rust's ownership system is designed to ensure memory safety without a garbage collector.
    // Strings, especially String type, are dynamically allocated on the heap.
    // Managing the ownership and lifetime of these strings can be challenging, particularly when passing them around in a program.

    // Immutable &str
    let name_immutable = "MyNameImmu";
    // Mutable String
    let name_mutable: String = String::from("MyNameMu");

    // Printing the immutable and mutable strings
    println!("Immutable name: {}", name_immutable);
    println!("Mutable name: {}", name_mutable);

    let str = String::from("String");
    let char_0 = str.chars().nth(0);
    // Notice `nth` returns an Option<char> because the element at the specified index might not exist.
    // If the index is out of bounds, None is returned; otherwise, it returns Some(char).
    let char_100 = str.chars().nth(100);

    print!("{}", char_0.unwrap()); // You cannot simply print the char you need to unwrap it
    print!("{}", char_100.unwrap()); // This will print called `Option::unwrap()` on a `None` value, we will see better ways of doing this
}

fn main() {
    // type_number();
    // type_boolean();
    type_string();
}
