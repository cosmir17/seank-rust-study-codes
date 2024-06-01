fn selection_sort_no_return<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() - 1 {
        let mut smallest = i;
        for j in (i+1)..array.len() {
            if array[j] < array[smallest] {
                smallest = j
            }
        }
        array.swap(smallest, i);
    }
}

// fn selection_sort_to_vec(array: &mut Vec<i8>) -> Vec<i8> {
//     for i in 0..array.len() - 1 {
//         let mut smallest = i;
//         for j in (i+1)..array.len() {
//             if array[j] < array[smallest] {
//                 smallest = j
//             }
//         }
//         array.swap(smallest, i);
//     }
//     array.to_vec()
// }
//
// fn selection_sort_mut_vec(array: &mut Vec<i8>) -> &mut Vec<i8> {
//     for i in 0..array.len() - 1 {
//         let mut smallest = i;
//         for j in (i+1)..array.len() {
//             if array[j] < array[smallest] {
//                 smallest = j
//             }
//         }
//         array.swap(smallest, i);
//     }
//     array
// }

fn main () {
    let mut arr: Vec<i8> = vec![4, 3, 1, 2];

    println!("Before sorting: {:?}", arr);
    selection_sort_no_return(&mut arr);               //use this! they all modify the original arr
    // let a = selection_sort_mut_vec(&mut arr);      //they all modify the original arr
    // let a = selection_sort_to_vec(&mut arr);       //they all modify the original arr
    println!("After sorting: {:?}", arr);
}

// Q:
// 'array.to_vec()', Is there a performance drawback?

// Using `array.to_vec()` instead of returning a mutable reference has performance implications.
//
// When you use `array.to_vec()`, it creates a new `Vec<i8>` by cloning the elements from the original vector. This means that a new allocation is performed, and the elements are copied into the new vector. The time complexity of `to_vec()` is O(n), where n is the number of elements in the vector, as it needs to iterate over all the elements to copy them.
//
// On the other hand, returning a mutable reference to the original vector (`&mut Vec<i8>`) does not involve any additional allocations or copying of elements. It simply returns a reference to the existing vector, allowing the caller to access and modify the vector directly.
//
// In terms of performance, returning a mutable reference is generally faster because it avoids the overhead of creating a new vector and copying the elements. However, it's important to note that returning a mutable reference also means that the ownership of the vector remains with the caller, and the caller is responsible for managing the lifetime of the vector.
//
// If the API design prioritizes simplicity and ease of use, returning a new `Vec<i8>` using `array.to_vec()` can be a valid choice. It provides a clean and straightforward interface, and the performance impact may not be significant for small to medium-sized vectors or in scenarios where the sorting operation is not a performance-critical part of the application.
//
// However, if performance is a critical concern and the vectors being sorted are large, returning a mutable reference to the original vector can provide better performance by avoiding unnecessary allocations and copying.
//
// Ultimately, the choice between returning a new vector or a mutable reference depends on the specific requirements and trade-offs of your application, such as API design, performance considerations, and ownership management.



// Q:
// 1. What does Ord mean?
// 2. is i8 a subtype of Ord?

// A :
// 1. `Ord` is a trait in Rust that represents types that have a total order. It is defined in the `std::cmp` module. Types that implement the `Ord` trait can be compared using the `<`, `<=`, `>`, and `>=` operators, and they have a total ordering. This means that for any two values of the type, one value is either less than, equal to, or greater than the other value.
//
//    The `Ord` trait provides the following methods:
//    - `cmp(&self, other: &Self) -> Ordering`: Compares `self` to `other` and returns an `Ordering` value indicating their relative order.
//    - `max(self, other: Self) -> Self`: Returns the maximum of `self` and `other`.
//    - `min(self, other: Self) -> Self`: Returns the minimum of `self` and `other`.
//    - `clamp(self, min: Self, max: Self) -> Self`: Returns `self` clamped to the range `[min, max]`.
//
//    By using `T: Ord` in the generic function `selection_sort_no_return<T: Ord>(array: &mut [T])`, it specifies that the type `T` must implement the `Ord` trait. This allows the function to compare elements of type `T` using the `<` operator.
//
// 2. Yes, `i8` (8-bit signed integer) is a subtype of `Ord`. In fact, all primitive integer types in Rust, including `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`, implement the `Ord` trait. This means that they have a total ordering and can be compared using the `<`, `<=`, `>`, and `>=` operators.
//
//    In the given code:
//    ```rust
//    let mut arr: Vec<i8> = vec![4, 3, 1, 2];
//    selection_sort_no_return(&mut arr);
//    ```
//    `arr` is a `Vec<i8>`, which is a vector of `i8` values. Since `i8` implements the `Ord` trait, it satisfies the generic constraint `T: Ord` in the `selection_sort_no_return` function. Therefore, the function can be called with `&mut arr` as the argument, and it will perform the selection sort algorithm on the vector of `i8` values.