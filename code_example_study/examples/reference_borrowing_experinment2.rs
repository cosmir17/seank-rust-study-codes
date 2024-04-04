use std::ops::Add;

fn multiply(num: &u32) -> u32 {
    num * 3
}

fn divide(num: u32) -> u32 {
    num / 2
}


fn main() {
    let my_number = 4;
    let multiplied = multiply(&my_number);
    let second_multiply = divide(my_number);
    println!("{}", my_number);
    println!("{}", multiplied);
    println!("{}", second_multiply);

    let mut abc_string = String::from("abc");
    let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
    println!("{}", mutable_ref_push_str_done);

    let mutable_variable_added = mutable_variable_add(abc_string);
    // println!("{}", abc_string); //this fails because move happens in mutable_variable_add variable

    println!("{}", mutable_variable_added);
}

fn mutable_ref_push_str(string_input_ref: &mut String) -> &String {
    string_input_ref.push_str("pppp");
    string_input_ref
}

fn mutable_variable_add(string_input: String) -> String {
    string_input.add("nnnn")
}

//A.I. learning

//S
// I have the following Rust program :

// why can my_number be displayed? I thought the ownership of my_number got moved to divide function.
//
// In the given Rust program, the ownership of `my_number` is not moved to the `divide` function. Let's go through the code step by step to understand why `my_number` can still be displayed:
//
// 1. `let my_number = 4;`: Here, `my_number` is declared as a variable with an initial value of 4. It is owned by the `main` function.
//
// 2. `let multiplied = multiply(&my_number);`: In this line, `&my_number` creates a reference to `my_number` and passes it to the `multiply` function. The `multiply` function accepts a reference to a `u32` value. Since it only borrows a reference to `my_number`, the ownership of `my_number` remains with the `main` function.
//
// 3. `let second_multiply = divide(my_number);`: Here, `my_number` is passed directly to the `divide` function. However, `my_number` is of type `u32`, which implements the `Copy` trait. Types that implement the `Copy` trait allow their values to be duplicated when assigned or passed as arguments. In this case, a copy of `my_number` is passed to the `divide` function, and the original `my_number` remains owned by the `main` function.
//
// 4. `println!("{}", my_number);`: Since the ownership of `my_number` was not moved in the previous function calls, it can still be accessed and displayed here.
//
// 5. `println!("{}", multiplied);` and `println!("{}", second_multiply);`: These lines simply display the results of the `multiply` and `divide` function calls.
//
// In summary, `my_number` can be displayed because:
// - In the `multiply` function call, only a reference to `my_number` is passed, so the ownership remains with the `main` function.
// - In the `divide` function call, `my_number` is of type `u32`, which implements the `Copy` trait. This means a copy of `my_number` is passed to the function, and the original `my_number` remains owned by the `main` function.
//
// Therefore, `my_number` can still be accessed and displayed after the function calls.
//

// I have the following code :
//
//
// Copy code
// fn main() {
//     let abc_string = String::from("abc");
//     let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
//     println!("{}", mutable_ref_push_str_done);
//
//     let mutable_variable_added = mutable_variable_add(abc_string);
//     // println!("{}", abc_string);
//
//     println!("{}", mutable_variable_added);
// }
//
// fn mutable_ref_push_str(string_input_ref: &mut String) -> &String {
//     string_input_ref.push_str("pppp");
//     string_input_ref
// }
//
// fn mutable_variable_add(string_input: String) -> String {
//     string_input.add("nnnn")
// }
// I have an error message :
//
// """
//
// error[E0596]: cannot borrow abc_string as mutable, as it is not declared as mutable
//
// --> code_example_study/examples/reference_borrowing_experinment2.rs:21:58
//
// |
//
// 21 | let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
//
// | ^^^^^^^^^^^^^^^ cannot borrow as mutable
//
// |
//
// help: consider changing this to be mutable
//
// |
//
// 20 | let mut abc_string = String::from("abc");
//
// | +++
//
// """
//
// The error message is correct. The issue here is that you declared abc_string as an immutable variable, but you're trying to pass a mutable reference &mut abc_string to the mutable_ref_push_str function.
//
// In Rust, to borrow a variable as mutable, the variable itself needs to be declared as mut. Here's the corrected code:
//
// rust
//
//
// Copy code
// fn main() {
//     let mut abc_string = String::from("abc"); // Declare abc_string as mutable
//     let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
//     println!("{}", mutable_ref_push_str_done);
//
//     let mutable_variable_added = mutable_variable_add(abc_string);
//     // println!("{}", abc_string); // Error: abc_string has been moved
//
//     println!("{}", mutable_variable_added);
// }
//
// fn mutable_ref_push_str(string_input_ref: &mut String) -> &String {
//     string_input_ref.push_str("pppp");
//     string_input_ref
// }
//
// fn mutable_variable_add(string_input: String) -> String {
//     string_input.add("nnnn")
// }
// In the corrected code:
//
// abc_string is declared as let mut abc_string = String::from("abc");, making it a mutable String variable.
// You can now pass &mut abc_string as a mutable reference to the mutable_ref_push_str function.
// However, there's another issue with the line let mutable_variable_added = mutable_variable_add(abc_string);. The mutable_variable_add function takes ownership of the String value, so after this line, abc_string is no longer valid. Uncommenting println!("{}", abc_string); will result in an error because abc_string has been moved.
//
// To fix this, you can either:
//
// Clone abc_string before passing it to mutable_variable_add:
// rust
//
//
// Copy code
// let mutable_variable_added = mutable_variable_add(abc_string.clone());
// println!("{}", abc_string); // abc_string is still valid
// Or, change the mutable_variable_add function to take a reference instead of consuming the String value:
// rust
//
//
// Copy code
// fn mutable_variable_add(string_input: &String) -> String {
//     let mut new_string = string_input.clone();
//     new_string.push_str("nnnn");
//     new_string
// }
// In this case, you would call mutable_variable_add like this:
//
// rust
//
//
// Copy code
// let mutable_variable_added = mutable_variable_add(&abc_string);
// The key takeaway is that for a variable to be borrowed as mutable, it needs to be declared as mut. Additionally, be mindful of ownership and borrowing rules to avoid moving or invalidating variables unintentionally.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
//
// Does it mean the variable is mutable or the reference is mutable?
//
// In the line:
//
// rust
//
//
// Copy code
// let mutable_ref_push_str_done = mutable_ref_push_str(&mut abc_string);
// The &mut abc_string part means that we are taking a mutable reference to the abc_string variable. In other words, the reference itself is mutable, not the mutable_ref_push_str_done variable binding.
//
// To break it down:
//
// To break it down:
//
// abc_string is a String variable declared as let mut abc_string = String::from("abc");. Since it's declared with mut, it's a mutable variable.
// &mut abc_string creates a mutable reference to the abc_string variable.
// This mutable reference &mut abc_string is then passed as an argument to the mutable_ref_push_str function.
// Inside the mutable_ref_push_str function, the argument string_input_ref is of type &mut String, which means it's a mutable reference to a String.
// The mutable_ref_push_str_done variable is an immutable binding to the return value of the mutable_ref_push_str function, which is &String (an immutable reference to a String).
// So, in summary:
//
// abc_string is a mutable String variable.
// &mut abc_string is a mutable reference to the abc_string variable.
// mutable_ref_push_str_done is an immutable variable binding to an immutable reference (&String).
// The mutability in this line is about the reference &mut abc_string, not the mutable_ref_push_str_done variable binding itself. The mutable reference allows you to modify the value that abc_string points to within the mutable_ref_push_str function.