use std::sync::Mutex;



fn main() {
    let counter = Mutex::new(0);

    *counter.lock().unwrap() += 1; // Incrementing the counter requires exclusive access

    println!("Counter {:?}", *counter.lock().unwrap()); // It prints out 'Counter 1'

}


//I provided the following example :
//
// """
//
// use std::sync::Mutex; let counter = Mutex::new(0); // Incrementing the counter requires exclusive access
// *counter.lock().unwrap() += 1;
//
// """
//
// Why *?
//
//
// In the example you provided, the `*` is the dereference operator in Rust. It is used to access the value behind a reference or a smart pointer, such as `Mutex`.
//
// Let's break down the line `*counter.lock().unwrap() += 1;`:
//
// 1. `counter` is a `Mutex<i32>`, which is a smart pointer that provides thread-safe access to the wrapped value.
//
// 2. `counter.lock()` acquires the lock on the mutex, allowing exclusive access to the wrapped value. It returns a `Result<MutexGuard<i32>, PoisonError<MutexGuard<i32>>>`.
//
// 3. `unwrap()` is called on the `Result` returned by `lock()`. It unwraps the `Result`, yielding the inner `MutexGuard<i32>` if the lock was successfully acquired, or panicking if the lock was poisoned due to a thread panic.
//
// 4. The `MutexGuard<i32>` is a smart pointer that provides mutable access to the wrapped value inside the mutex. However, to actually access or modify the value, we need to dereference the `MutexGuard`.
//
// 5. The `*` operator is used to dereference the `MutexGuard`, giving us direct access to the wrapped `i32` value.
//
// 6. Finally, `+= 1` increments the dereferenced value by 1.
//
// So, the `*` is necessary to access the actual value inside the `MutexGuard` and perform the increment operation on it.
//
// Without the `*`, the line would be `counter.lock().unwrap() += 1;`, which would attempt to increment the `MutexGuard` itself, not the wrapped value. This would result in a compilation error because `MutexGuard` does not have the `+=` operator implemented.
//
// In summary, the `*` dereference operator is used to access the value behind the `MutexGuard` smart pointer, allowing us to modify the wrapped value inside the mutex.





//Is '*counter.lock().unwrap() += 1;' same as '*(counter.lock().unwrap()) += 1;' or '*(counter.lock()).unwrap() += 1;'?
//
//
// No, the expressions `*counter.lock().unwrap() += 1;`, `*(counter.lock().unwrap()) += 1;`, and `*(counter.lock()).unwrap() += 1;` are not the same and have different meanings.
//
// Let's break it down:
//
// 1. `*counter.lock().unwrap() += 1;`:
//    - This is the correct expression to increment the value inside the `Mutex`.
//    - `counter.lock()` acquires the lock on the mutex and returns a `Result<MutexGuard<i32>, PoisonError<MutexGuard<i32>>>`.
//    - `unwrap()` is called on the `Result`, yielding the `MutexGuard<i32>` if the lock was successfully acquired, or panicking if the lock was poisoned.
//    - The `*` operator dereferences the `MutexGuard<i32>`, giving access to the wrapped `i32` value.
//    - `+= 1` increments the dereferenced value by 1.
//
// 2. `*(counter.lock().unwrap()) += 1;`:
//    - This expression is equivalent to `*counter.lock().unwrap() += 1;`.
//    - The parentheses `()` around `counter.lock().unwrap()` are redundant and don't change the meaning.
//    - It acquires the lock, unwraps the `Result`, dereferences the `MutexGuard`, and increments the value.
//
// 3. `*(counter.lock()).unwrap() += 1;`:
//    - This expression is incorrect and would result in a compilation error.
//    - `counter.lock()` returns a `Result<MutexGuard<i32>, PoisonError<MutexGuard<i32>>>`.
//    - Attempting to dereference the `Result` with `*` before calling `unwrap()` is not allowed because `Result` does not implement the `Deref` trait.
//    - The correct order is to call `unwrap()` on the `Result` first, and then dereference the resulting `MutexGuard`.
//
// So, the correct and idiomatic way to increment the value inside a `Mutex` is:
//
// ```rust
// *counter.lock().unwrap() += 1;
// ```
//
// This acquires the lock, unwraps the `Result`, dereferences the `MutexGuard`, and increments the value in a single, concise expression.
//
// The expression `*(counter.lock().unwrap()) += 1;` is equivalent but has redundant parentheses.
//
// The expression `*(counter.lock()).unwrap() += 1;` is incorrect and would not compile.