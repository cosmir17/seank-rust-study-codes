fn main() {
    // let mut x = 5;
    //
    // let y = &x;     // Immutable borrow
    // let z = &mut x; // Mutable borrow
    //
    // println!("y: {}, z: {}", y, z); //can't use them together


    //This works
    //
    // let mut x = 5;
    // {
    //     let y = &x;     // Immutable borrow
    //     println!("y: {}", y);
    //
    // }
    // let z = &mut x; // Mutable borrow
    // println!("z: {}", z);


    //can use Immutable and mutable references separately according to the burrow rule
    let mut x = 5;

    let z = &mut x; // Mutable borrow
    println!("z: {}", z);

    let y = &x;     // Immutable borrow
    println!("y: {}", y);

    // println!("z: {}", z); this can't be done here as it breaches the Rust's burrow rule

}


//S
// This works because 'y' is implicitly dropped after the 'println!("y: {}", y);' line?
//
//
// Copy code
// let mut x = 5;
//
// let y = &x;     // Immutable borrow
// println!("y: {}", y);
//
// let z = &mut x; // Mutable borrow
// println!("z: {}", z);

// Yes, that's correct! In the code snippet you provided:
//
// ```rust
// let mut x = 5;
//
// let y = &x;     // Immutable borrow
// println!("y: {}", y);
//
// let z = &mut x; // Mutable borrow
// println!("z: {}", z);
// ```
//
// The immutable borrow `y` is implicitly dropped after the line `println!("y: {}", y);`. This is because `y` is not used anymore after that point, and Rust automatically inserts a drop point at the end of the variable's scope.
//
// In Rust, a variable's scope extends until the last time it is used. In this case, `y` is last used in the `println!` statement, so its scope ends immediately after that line. When a variable goes out of scope, Rust automatically drops the variable and releases any borrowed references associated with it.
//
// After `y` is dropped, the immutable borrow is released, allowing the mutable borrow `z` to be created on the next line without violating Rust's borrowing rules. The mutable borrow `z` then lasts until it is last used in the `println!` statement.
//
// So, the code works because:
// 1. `y` is an immutable borrow of `x`.
// 2. `y` is used in the `println!` statement and then implicitly dropped, releasing the immutable borrow.
// 3. `z` is created as a mutable borrow of `x`, which is allowed because the immutable borrow `y` has been released.
// 4. `z` is used in the `println!` statement and then implicitly dropped at the end of its scope.
//
// Rust's ownership system and automatic drop semantics ensure that borrows are released at the appropriate times, preventing issues like dangling references and allowing for safe and efficient memory management.