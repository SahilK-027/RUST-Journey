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

fn main() {
    // type_number();
    type_boolean();
}
