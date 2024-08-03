// Basic variable declaration
/**
 * You can define variables using the `let` keyword.
 * You can assign the type of variable explicitly, or it can be inferred by the compiler.
 */

/// Demonstrates working with numbers in Rust.
fn type_number() {
    // Explicit type annotation
    let num: i32 = 10; // Signed 32-bit integer
    // Inferred type
    let x = 42; // The type of `x` is inferred to be `i32` (an integer)
    // Unsigned 32-bit integer
    let y: u32 = 20;
    // 32-bit floating-point number
    let z: f32 = 20.1;

    // Uncommenting the following line will cause an overflow error
    // let a: i8 = 1024; // Overflow

    // Print variables using formatting specifiers
    println!("x: {}, y: {}, z: {}", x, y, z);
}

/// Demonstrates working with boolean values in Rust.
fn type_boolean() {
    // Immutable boolean variable
    let is_raining = true;
    // Mutable boolean variable
    let mut is_sunny = false;

    // Uncommenting the following line will cause a compile-time error
    // is_raining = false; // All variables in Rust are immutable by default

    // Changing the value of a mutable variable
    is_sunny = true;

    // Use booleans in an if statement
    if is_raining {
        println!("Don't forget your umbrella!");
    } else if is_sunny {
        println!("It's a beautiful day!");
    } else {
        println!("Weather is unpredictable today.");
    }
}

/// Demonstrates working with strings in Rust.
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
    // Strings, especially the String type, are dynamically allocated on the heap.
    // Managing the ownership and lifetime of these strings can be challenging, particularly when passing them around in a program.

    // Immutable &str
    let name_immutable = "MyNameImmu";
    // Mutable String
    let name_mutable: String = String::from("MyNameMu");

    // Printing the immutable and mutable strings
    println!("Immutable name: {}", name_immutable);
    println!("Mutable name: {}", name_mutable);

    let str = String::from("String");

    // Getting the first character
    let char_0 = str.chars().nth(0);
    // Notice `nth` returns an Option<char> because the element at the specified index might not exist.
    // If the index is out of bounds, None is returned; otherwise, it returns Some(char).
    let char_100 = str.chars().nth(100);

    print!("{}", char_0.unwrap()); // You cannot simply print the char you need to unwrap it
    print!("{}", char_100.unwrap()); // This will print called `Option::unwrap()` on a `None` value, we will see better ways of doing this
}

fn conditional() {
    let number = 101;
    // In Rust, you must ensure that the expression in conditionals evaluates to a boolean value (true or false).
    // Unlike C/C++, Rust does not allow using integers directly in conditionals.
    // For example, `if 1 {}` or `if 0 {}` will result in a compile-time error in Rust.

    if number & 1 != 0 {
        println!("The number is odd.");
    } else {
        println!("The number is even.");
    }
}

fn for_loop(){
    // This loop will run 0 to 9 10 excluded
    for i in 0..10{
        print!("{} ", i);
    }
}

fn main() {
    // Uncomment the function calls below to test each function
    // type_number();
    // type_boolean();
    // type_string();
    // conditional();
    for_loop();
}


// List of Common Operators in Rust
// Here’s a list of common operators you can use in Rust:

// Arithmetic Operators:
// + (Addition): a + b
// - (Subtraction): a - b
// * (Multiplication): a * b
// / (Division): a / b
// % (Modulo): a % b

// Comparison Operators:
// == (Equal to): a == b
// != (Not equal to): a != b
// > (Greater than): a > b
// < (Less than): a < b
// >= (Greater than or equal to): a >= b
// <= (Less than or equal to): a <= b

// Logical Operators:
// && (Logical AND): a && b
// || (Logical OR): a || b
// ! (Logical NOT): !a

// Bitwise Operators:
// & (Bitwise AND): a & b
// | (Bitwise OR): a | b
// ^ (Bitwise XOR): a ^ b
// << (Left shift): a << b
// >> (Right shift): a >> b

// Assignment Operators:
// = (Assignment): a = b
// += (Add and assign): a += b
// -= (Subtract and assign): a -= b
// *= (Multiply and assign): a *= b
// /= (Divide and assign): a /= b
// %= (Modulo and assign): a %= b

// Unary Operators:
// - (Negation): -a
// + (Unary plus): +a

// Compound Assignment Operators:
// Used for shorthand operations like +=, -=, etc.
