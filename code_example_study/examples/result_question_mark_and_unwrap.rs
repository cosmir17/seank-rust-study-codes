
fn main() -> Result<(), &'static str> {

    let a: &str = "normal string literals";
    let b: &'static str = "static string";

    let unwrap_result: Result<i32, &str> = Ok(42);
    let unwrapped_value = unwrap_result.unwrap(); // Unwraps the value, panics if `result` is an `Err`
    println!("Unwrap Value: {}", unwrapped_value);

    let question_result: Result<i32, &str> = Err("Something went wrong");
    let question_value = question_result?; // Propagates the error, returns from `main` early
    println!("Question Mark Value: {}", question_value); // This line is not reached !!
    Err("to satisfy the compiler")
}

// You're right, my previous explanation was incorrect. Let me clarify why the code works with the changes.
//
// The `main` function's return type is `Result<(), &'static str>`, which means it returns either `Ok(())` (the unit value wrapped in `Ok` variant) or an `Err` with a `&'static str` error message.
//
// When you use the `?` operator with `let question_value = question_result?;`, it propagates the error from `question_result` (which is `Err("Something went wrong")`). Since the `main` function returns a `Result<(), &'static str>`, the `?` operator early returns the error value `Err("Something went wrong")` from the `main` function.
//
// The line `println!("Question Mark Value: {}", question_value);` is not reached because the `?` operator has already returned from the `main` function.
//
// So, by changing the last line to `Err("Something went wrong")`, you're explicitly returning the same error value that was propagated by the `?` operator.
//
// However, it's a good practice to have an explicit return statement at the end of a function that returns a `Result` or `Option`. This way, if the control flow reaches the end of the function without encountering any errors or `return` statements, it will return the default value (in this case, `Ok(())`) instead of leaving the return value uninitialized.
//
// Here's the code with an explicit `Ok(())` return at the end:
//
// ```rust
// fn main() -> Result<(), &'static str> {
//     let unwrap_result: Result<i32, &str> = Ok(42);
//     let unwrapped_value = unwrap_result.unwrap(); // Unwraps the value, panics if `result` is an `Err`
//     println!("Unwrap Value: {}", unwrapped_value);
//
//     let question_result: Result<i32, &str> = Err("Something went wrong");
//     let question_value = question_result?; // Propagates the error, returns from `main` early
//     println!("Question Mark Value: {}", question_value); // This line is not reached
//
//     Ok(()) // Explicitly return `Ok(())` if no errors occurred
// }
// ```
//
// This way, if the `question_result` is `Ok`, the `?` operator will not propagate an error, and the `main` function will return `Ok(())` at the end.



// In the line `let question_result: Result<i32, &str> = Err("Something went wrong");`, the `&str` inside `Err` has a `'static` lifetime because the string literal `"Something went wrong"` has a `'static` lifetime.
//
// In Rust, string literals have a `'static` lifetime, which means they are stored in the read-only data section of the binary and have a lifetime that spans the entire program execution. This is a compiler optimization to avoid unnecessarily allocating and freeing memory for string literals every time they are used.
//
// When you use a string literal like `"Something went wrong"` in a context that requires a reference (`&str`), the compiler automatically infers the lifetime of the reference to be `'static` because the string literal itself has a `'static` lifetime.
//
// The `'static` lifetime is a special lifetime that represents the entire duration of the program's execution. It is tied to the binary itself and ensures that the data it references will always be valid and accessible throughout the program's lifetime.
//
// In contrast, if you were to create a `String` value from a non-literal string, the resulting `&str` reference would not have a `'static` lifetime because the `String` is allocated on the heap and has a lifetime tied to the scope in which it's created.
//
// For example:
//
// ```rust
// let message = String::from("Something went wrong"); // message has a non-'static lifetime
// let question_result: Result<i32, &str> = Err(&message[..]); // &message[..] has the same non-'static lifetime as message
// ```
//
// In this case, the `&str` reference inside `Err` has the same lifetime as the `message` variable because it's a reference to the data owned by `message`.
//
// So, in your example, the `&str` inside `Err` has a `'static` lifetime because it's a reference to a string literal, which has a `'static` lifetime by default. This is a common pattern in Rust when you need to return or store string literals or other static data in a context that requires a reference.