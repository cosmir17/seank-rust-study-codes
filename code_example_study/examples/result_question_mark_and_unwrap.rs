
fn main() -> Result<(), &'static str> {
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