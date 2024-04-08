
use std::sync::Arc;

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let (truck_a, truck_b, truck_c) = (Arc::new(Truck { capacity: 1 }),
                                       Arc::new(Truck { capacity: 2 }),
                                       Arc::new(Truck { capacity: 3 }),
    );

    // let thread = std::thread::spawn(|| {
    //     let facility_one = vec![truck_a, truck_b.clone()];
    //     let facility_two = vec![truck_b.clone(), truck_c];
    //     (facility_one, facility_two)
    // });

    //error[E0277]: `Rc<Truck>` cannot be sent between threads safely
    //    --> code_example_study/examples/main_arc.rs:31:18
    //     |
    // 31  |       let thread = std::thread::spawn(|| {
    //     |  __________________^
    // 32  | |         let facility_one = vec![truck_a, truck_b.clone()];
    // 33  | |         let facility_two = vec![truck_b.clone(), truck_c];
    // 34  | |         (facility_one, facility_two)
    // 35  | |     });
    //     | |______^ `Rc<Truck>` cannot be sent between threads safely
    //     |
    //     = help: the trait `Send` is not implemented for `Rc<Truck>`, which is required by `(Vec<Rc<Truck>>, Vec<Rc<Truck>>): Send`
    //     = note: required for `Unique<Rc<Truck>>` to implement `Send`
    // note: required because it appears within the type `alloc::raw_vec::RawVec<Rc<Truck>>`
    //    --> /Users/sk/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:61:19
    //     |
    // 61  | pub(crate) struct RawVec<T, A: Allocator = Global> {
    //     |                   ^^^^^^
    // note: required because it appears within the type `Vec<Rc<Truck>>`
    //    --> /Users/sk/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:398:12
    //     |
    // 398 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    //     |            ^^^
    //     = note: required because it appears within the type `(Vec<Rc<Truck>>, Vec<Rc<Truck>>)`
    // note: required by a bound in `spawn`
    //    --> /Users/sk/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:682:8
    //     |
    // 678 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     |        ----- required by a bound in this function
    // ...
    // 682 |     T: Send + 'static,
    //     |        ^^^^ required by this bound in `spawn`

    let thread = std::thread::spawn(move || {
        let facility_one = vec![truck_a, truck_b.clone()];
        let facility_two = vec![truck_b, truck_c];
        (facility_one, facility_two)
    });

    // Move allows move operations (method, variables) inside spawn method
    // even if borrow is used within the closure

    //atomic data types which the compiler knows how to order operations on
    //it doesn't use locks
    //non-atomic safety allows the compiler to do more optimisation

    let (facility_one, facility_two) = thread.join().unwrap();

    println!("Facility one {:?}", facility_one);
    println!("Facility two {:?}", facility_two);

    let truck_b= Arc::clone(&facility_one[1]);
    //In the line let truck_b = Arc::clone(&facility_one[1]);, the & applies to the second element of the facility_one vector, not the vector itself. Here's how it works:
    // facility_one[1] retrieves the second element (index 1) from the facility_one vector.
    // The & is then applied to facility_one[1], taking a reference to that element.
    // Arc::clone(&facility_one[1]) creates a new Arc pointer that shares ownership of the Truck instance referenced by facility_one[1]. This increments the reference count of the Arc.
    // The cloned Arc pointer is then assigned to truck_b.
    // So, truck_b becomes a new Arc pointer that shares ownership with the second element of facility_one.

    println!("truck_b strong count : {:?}", Arc::strong_count(&truck_b));
    std::mem::drop(facility_two);
    println!("truck_b strong count after facility_two drop : {:?}", Arc::strong_count(&truck_b));
    println!("Facility one after facility_two drop : {:?}", facility_one);
    std::mem::drop(facility_one);

    let rrr = String::from("sdfsdf");
    println!("rrr string is {:}", rrr);
}


//A.I.

//The error message is indicating that the type `Rc<Truck>` does not implement the `Send` trait, which is required for types that can be safely sent between threads.
//
// In Rust, the `Send` trait is an auto trait that indicates that ownership of a type implementing `Send` can be transferred between threads. However, `Rc<T>` (Reference Counting) is a non-thread-safe reference-counted pointer type, meaning it cannot be safely sent between threads.
//
// The error occurs because you are attempting to move `Rc<Truck>` values (`truck_a`, `truck_b`, `truck_c`) into a new thread spawned by `std::thread::spawn`. The closure passed to `spawn` captures these `Rc<Truck>` values, but since `Rc<Truck>` does not implement `Send`, it cannot be safely sent to the new thread.
//
// To fix this issue, you have a couple of options:
//
// 1. Use `Arc<T>` (Atomically Reference Counted) instead of `Rc<T>`. `Arc<T>` is a thread-safe reference-counted pointer type that implements the `Send` trait. By using `Arc<Truck>` instead of `Rc<Truck>`, the code will compile successfully.
//
// 2. If you don't actually need to send the `Truck` values between threads, you can move the code that uses them inside the main thread instead of spawning a new thread.
//
// Here's an example of using `Arc<T>` to fix the code:
//
// ```rust
// use std::sync::Arc;
//
// #[derive(Debug)]
// struct Truck {
//     capacity: i32,
// }
//
// fn main() {
//     let (truck_a, truck_b, truck_c) = (Arc::new(Truck { capacity: 1 }),
//                                        Arc::new(Truck { capacity: 2 }),
//                                        Arc::new(Truck { capacity: 3 }),
//     );
//
//     let thread = std::thread::spawn(move || {
//         let facility_one = vec![truck_a, truck_b.clone()];
//         let facility_two = vec![truck_b.clone(), truck_c];
//         (facility_one, facility_two)
//     });
// }
// ```
//
// By using `Arc<Truck>` and adding the `move` keyword before the closure, the code will compile successfully, allowing you to send the `Truck` values between threads safely.

//...

//S
// still you are not giving me the right answer. I was asking how the mechanism is automatic

//Apologies for the confusion. Let me explain how the `Send` trait requirement is automatically enforced by the Rust compiler.
//
// In Rust, the `Send` trait is an auto trait, which means that it is automatically implemented for types that satisfy certain conditions. The `Send` trait is defined in the Rust standard library as follows:
//
// ```rust
// pub unsafe auto trait Send {
//     // ...
// }
// ```
//
// The `auto` keyword indicates that the trait is an auto trait. The Rust compiler automatically determines whether a type implements the `Send` trait based on the following rules:
//
// 1. All primitive types (e.g., `i32`, `bool`, `char`) automatically implement `Send`.
// 2. Structs and enums automatically implement `Send` if all their fields implement `Send`.
// 3. References (`&T`) and mutable references (`&mut T`) automatically implement `Send` if `T` implements `Send` and the reference is not a reference to a thread-local variable.
// 4. Raw pointers (`*const T` and `*mut T`) automatically implement `Send`.
// 5. Tuples, arrays, and slices automatically implement `Send` if all their elements implement `Send`.
// 6. Closures automatically implement `Send` if all the captured variables implement `Send`.
//
// The Rust compiler automatically checks these rules and determines whether a type implements the `Send` trait. If a type meets all the necessary conditions, the `Send` trait is automatically implemented for that type.
//
// In the case of `Rc<T>`, it does not automatically implement `Send` because it uses non-atomic reference counting, which is not thread-safe. The Rust compiler recognizes this and does not automatically implement `Send` for `Rc<T>`.
//
// When you attempt to move an `Rc<T>` value into a new thread using `std::thread::spawn`, the Rust compiler automatically checks if the closure and all the captured variables implement the `Send` trait. If any of the captured variables do not implement `Send`, the compiler generates an error indicating that the type cannot be sent between threads safely.
//
// This automatic enforcement of the `Send` trait by the Rust compiler ensures thread safety at compile time. It prevents you from accidentally sending non-thread-safe types across thread boundaries, helping to eliminate data races and undefined behavior.
//
// The automatic nature of the `Send` trait is part of Rust's ownership system and thread safety guarantees. By using auto traits like `Send`, Rust can provide thread safety checks at compile time without requiring explicit annotations or implementations from the programmer in most cases.
//
// I hope this clarifies how the `Send` trait requirement is automatically enforced by the Rust compiler. Let me know if you have any further questions!

//S
// you are getting there. now, explain to me what part in the original code is triggering that auto process

//In the original code, the part that triggers the automatic `Send` trait enforcement is the use of `std::thread::spawn` to create a new thread:
//
// ```rust
// let thread = std::thread::spawn(|| {
//     let facility_one = vec![truck_a, truck_b.clone()];
//     let facility_two = vec![truck_b.clone(), truck_c];
//     (facility_one, facility_two)
// });
// ```
//
// The `std::thread::spawn` function has a requirement that the closure passed to it and all the captured variables must implement the `Send` trait. This requirement is part of the function signature of `spawn`:
//
// ```rust
// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
// where
//     F: FnOnce() -> T,
//     F: Send + 'static,
//     T: Send + 'static,
// ```
//
// The `where` clause specifies that the closure `F` and the return type `T` must implement the `Send` trait and have a `'static` lifetime.
//
// When you call `std::thread::spawn` with a closure, the Rust compiler automatically checks if the closure and all the captured variables satisfy the `Send` trait requirement. In this case, the closure captures the `Rc<Truck>` values `truck_a`, `truck_b`, and `truck_c`.
//
// The Rust compiler then tries to automatically determine if `Rc<Truck>` implements the `Send` trait based on the auto trait rules mentioned earlier. However, `Rc<T>` does not automatically implement `Send` because it uses non-atomic reference counting, which is not thread-safe.
//
// As a result, the Rust compiler generates an error message indicating that `Rc<Truck>` cannot be sent between threads safely. The error message specifically points to the line where `std::thread::spawn` is called, highlighting that the captured variables do not satisfy the `Send` trait requirement.
//
// This automatic enforcement of the `Send` trait is triggered by the use of `std::thread::spawn` and the requirement specified in its function signature. The Rust compiler automatically checks the closure and captured variables against the `Send` trait requirement and generates an error if the requirement is not met.
//
// In summary, the use of `std::thread::spawn` in the original code triggers the automatic enforcement of the `Send` trait. The Rust compiler checks if the closure and captured variables (`Rc<Truck>` in this case) implement `Send`, and since `Rc<T>` does not automatically implement `Send`, the compiler generates an error message indicating that `Rc<Truck>` cannot be sent between threads safely.

//S
// I understand the point 2 but I don't understand point 1. Elaborate point 1

//Certainly! Let's dive deeper into the `move` keyword and its purpose in the context of `std::thread::spawn`.
//
// In Rust, closures capture variables from their surrounding scope by default. The way closures capture variables depends on how the variables are used within the closure. There are three types of captures:
//
// 1. Capture by reference (&T): The closure borrows the variable and refers to it by reference.
// 2. Capture by mutable reference (&mut T): The closure borrows the variable mutably and can modify it.
// 3. Capture by value (T): The closure takes ownership of the variable and moves it into the closure.
//
// When you spawn a new thread using `std::thread::spawn`, the closure passed to it must be `'static`, meaning it cannot contain references to variables that are owned by the spawning thread. This is because the spawned thread may outlive the spawning thread, and using references to variables owned by the spawning thread could lead to dangling references and undefined behavior.
//
// To solve this issue, you can use the `move` keyword before the closure. The `move` keyword forces the closure to take ownership of the captured variables, moving them into the closure. This way, the spawned thread owns the captured variables and can safely use them, even if the spawning thread terminates.
//
// In the code snippet you provided:
//
// ```rust
// let thread = std::thread::spawn(move || {
//     let facility_one = vec![truck_a, truck_b.clone()];
//     let facility_two = vec![truck_b.clone(), truck_c];
//     (facility_one, facility_two)
// });
// ```
//
// By using `move`, the ownership of `truck_a`, `truck_b`, and `truck_c` is transferred into the closure. The spawned thread takes ownership of these variables, allowing it to safely access and use them independently of the spawning thread.
//
// Without the `move` keyword, the closure would capture the variables by reference, which would not be allowed because the spawned thread requires ownership of the captured variables to ensure safety.
//
// So, in summary, the `move` keyword is used to transfer ownership of captured variables into the closure when spawning a new thread, guaranteeing that the spawned thread can safely access and use those variables without the risk of dangling references.

//S
// Elaborate : This is because the spawned thread may outlive the spawning thread, and using references to variables owned by the spawning thread could lead to dangling references and undefined behavior.

//Certainly! Let's explore the concept of dangling references and undefined behavior in the context of spawning threads.
//
// In Rust, each thread has its own stack, which is a memory region where local variables and function call information are stored. When a thread terminates, its stack is deallocated, and the memory is no longer accessible.
//
// Consider a scenario where you spawn a new thread and pass a closure that captures variables by reference from the spawning thread's scope. If the spawning thread terminates before the spawned thread finishes executing, the references captured by the closure in the spawned thread will become invalid. These references would be pointing to memory that has been deallocated, leading to dangling references.
//
// Dangling references are references that refer to memory that has been freed or is no longer valid. Accessing or dereferencing dangling references can lead to undefined behavior, which means the program's behavior is unpredictable and can cause crashes, corruptions, or other unexpected results.
//
// Here's a simple example to illustrate the problem:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(|| {
//         println!("Spawned thread: {:?}", data);
//     });
//     // Spawning thread terminates here
//     thread.join().unwrap();
// }
// ```
//
// In this example, the `data` vector is owned by the main thread. The spawned thread captures `data` by reference in its closure. If the main thread terminates before the spawned thread finishes executing, the reference to `data` in the spawned thread becomes a dangling reference because the memory owned by the main thread (including `data`) has been deallocated.
//
// To prevent such issues, Rust's ownership system and borrowing rules ensure that references are always valid. When spawning a thread, the closure must either capture variables by value (moving ownership) or ensure that the references captured have a lifetime that outlives the spawned thread.
//
// By using the `move` keyword before the closure, you force the closure to take ownership of the captured variables, moving them into the closure. This way, the spawned thread owns the captured variables, and their lifetimes are no longer tied to the spawning thread. The spawned thread can safely access and use the variables without the risk of dangling references.
//
// In the code snippet you provided earlier:
//
// ```rust
// let thread = std::thread::spawn(move || {
//     let facility_one = vec![truck_a, truck_b.clone()];
//     let facility_two = vec![truck_b.clone(), truck_c];
//     (facility_one, facility_two)
// });
// ```
//
// By using `move`, the ownership of `truck_a`, `truck_b`, and `truck_c` is transferred to the spawned thread, ensuring that the spawned thread can safely access and use these variables even if the spawning thread terminates.
//
// This is why the `move` keyword is important when spawning threads: it helps prevent dangling references and undefined behavior by transferring ownership of captured variables to the spawned thread.

//S
// Does it mean whatever variables in the spawn method will be referenced even if method that requires ownership is in the spawn method?

//No, that's not quite right. The `move` keyword in the `spawn` method ensures that the closure takes ownership of the captured variables, but it doesn't automatically make everything referenced.
//
// When you use `move` before the closure in `spawn`, the closure will take ownership of any variables it captures. If the closure uses a method or operation that requires ownership of a variable, it will move that variable into the closure.
//
// However, if the closure only needs to borrow a variable (either immutably or mutably), it will capture it by reference instead of moving ownership. The `move` keyword only affects the capture behavior of the closure, not the specific usage of the variables within the closure.
//
// Let's look at an example:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(move || {
//         // Ownership of `data` is moved into the closure
//         println!("Spawned thread: {:?}", data);
//         // `data` can be used here because the closure owns it
//     });
//     // Cannot use `data` here because ownership has been moved into the closure
//     thread.join().unwrap();
// }
// ```
//
// In this case, the `move` keyword causes the closure to take ownership of `data`. The closure can then use `data` freely because it owns it.
//
// On the other hand, if the closure only needs to borrow `data` immutably, it can capture it by reference:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(move || {
//         // `data` is borrowed immutably by the closure
//         println!("Spawned thread: {:?}", &data);
//         // `data` is not moved and can still be used by the spawning thread
//     });
//     // `data` can still be used here because it was only borrowed by the closure
//     println!("Main thread: {:?}", data);
//     thread.join().unwrap();
// }
// ```
//
// In this example, the closure captures `data` by immutable reference using `&data`, so ownership is not moved. The spawning thread can still use `data` after spawning the thread.
//
// So, while `move` ensures that the closure takes ownership of captured variables, it doesn't automatically make everything referenced. The specific usage of the variables within the closure determines whether they are moved, borrowed immutably, or borrowed mutably.

//S
// What I meant was : without move keyword in spawn method, does it mean whatever variables in the spawn method will be referenced even if method that requires ownership is in the spawn method?

//Apologies for the confusion. Let me clarify.
//
// Without the `move` keyword in the `spawn` method, the closure will capture variables by reference by default, even if a method that requires ownership is used within the closure.
//
// However, this can lead to compilation errors if the closure tries to use a method or operation that requires ownership of a captured variable. The Rust compiler will enforce the borrowing rules and ownership semantics to ensure safety.
//
// Let's consider an example:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(|| {
//         // Trying to move `data` into the closure without `move` keyword
//         println!("Spawned thread: {:?}", data);
//     });
//     // Compilation error: `data` is borrowed by the closure and cannot be used here
//     println!("Main thread: {:?}", data);
//     thread.join().unwrap();
// }
// ```
//
// In this case, the closure captures `data` by reference because the `move` keyword is not used. However, the `println!` macro inside the closure tries to move `data` by using it without the `&` reference operator.
//
// This will result in a compilation error because the closure only has a borrowed reference to `data`, but the `println!` macro requires ownership of `data`. The Rust compiler will prevent this code from compiling to ensure safety.
//
// To fix this, you have two options:
//
// 1. Use the `move` keyword to transfer ownership of `data` into the closure:
//
// ```rust
// let thread = std::thread::spawn(move || {
//     println!("Spawned thread: {:?}", data);
// });
// ```
//
// 2. Borrow `data` immutably in the closure:
//
// ```rust
// let thread = std::thread::spawn(|| {
//     println!("Spawned thread: {:?}", &data);
// });
// ```
//
// In summary, without the `move` keyword, the closure will capture variables by reference, even if a method that requires ownership is used within the closure. However, this will lead to compilation errors if the closure tries to use a captured variable in a way that requires ownership. The Rust compiler will enforce the borrowing rules and ownership semantics to prevent unsafe code.

//S
// how about with move keyword in spawn method?

//When you use the `move` keyword in the `spawn` method, the closure takes ownership of the captured variables, regardless of how they are used within the closure.
//
// Here's an example:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(move || {
//         // Ownership of `data` is moved into the closure
//         println!("Spawned thread: {:?}", data);
//         // `data` can be used here because the closure owns it
//     });
//     // Cannot use `data` here because ownership has been moved into the closure
//     // println!("Main thread: {:?}", data); // This would cause a compilation error
//     thread.join().unwrap();
// }
// ```
//
// In this case, the `move` keyword causes the closure to take ownership of `data`. The closure can then use `data` in any way it needs, including methods that require ownership, because it owns the variable. The spawning thread can no longer use `data` after the closure takes ownership of it.
//
// If the closure only needs to borrow `data`, it can still do so with the `move` keyword:
//
// ```rust
// fn main() {
//     let data = vec![1, 2, 3];
//     let thread = std::thread::spawn(move || {
//         // `data` is borrowed immutably by the closure
//         println!("Spawned thread: {:?}", &data);
//         // `data` is still owned by the closure and cannot be used by the spawning thread
//     });
//     // Cannot use `data` here because ownership has been moved into the closure
//     // println!("Main thread: {:?}", data); // This would cause a compilation error
//     thread.join().unwrap();
// }
// ```
//
// In this example, even though the closure only borrows `data` immutably using `&data`, the `move` keyword still causes the closure to take ownership of `data`. The spawning thread cannot use `data` after spawning the thread.
//
// The `move` keyword is useful when you want to ensure that the spawned thread has exclusive ownership of the captured variables and can use them independently of the spawning thread. It helps prevent issues related to shared mutable state and lifetime conflicts between the spawning thread and the spawned thread.
//
// So, with the `move` keyword in the `spawn` method, the closure takes ownership of the captured variables, regardless of whether the methods used within the closure require ownership or not. This allows the closure to use the captured variables freely, but it also means that the spawning thread loses access to those variables after the closure takes ownership.