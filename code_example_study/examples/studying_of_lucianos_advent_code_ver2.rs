// Disclaimer I copied the code examples from the "What I learned by solving 50 Advent of Code challenges in Rust - Luciano Mammino"
// presentation video (https://www.youtube.com/watch?v=udHjmno-tfA) and studied them with the help of an AI.
// This file contains those example code snippets that illustrate various Rust concepts, syntax, and language
// features covered in the video. The file has sections with explanatory comments that correspond to different
// segments of the video on topics such as Rust's ownership model, data structures, control flow, error handling,
// and other core language mechanisms. The code samples are designed to be educational, showcasing idiomatic Rust
// coding patterns and best practices presented in the video.
//
// With the assistance of an AI, I studied this code to reinforce my understanding of the material from the linked
// video presentation.

// impl<'a> From<&'a str> for Instr<'a> {
//     fn from(s: &'a str) -> Self {
//         if s.starts_with("mask") {
//             Instr::Mask(&s[7..])
//         } else if s.starts_with("mem") {
//             let mut splits = s[4..].split("] = ");
//             let _addr = splits.next().unwrap().parse().unwrap();
//             let _value: u64 = splits.last().unwrap().parse().unwrap();
//             Instr::Mem(_addr, _value)
//         } else {
//             panic!("Invalid line found: {s}")
//         }
//     }
// }

#[derive(PartialEq, Debug)]
enum Instr<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

use std::convert::{TryFrom, TryInto};

impl<'a> TryFrom<&'a str> for Instr<'a> { //implementing TryFrom trait, will make 'try_into()' method available
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.starts_with("mask") {
            Ok(Instr::Mask(&s[7..]))
        } else if s.starts_with("mem") {
            let addr_value = &mut s[4..].split("] = ");
            let addr: u64 = addr_value.next().unwrap().parse().unwrap();
            let value: u64 = addr_value.next().unwrap().parse().unwrap();
            Ok(Instr::Mem(addr, value))
        } else {
            Err(format!("Invalid line found: {s}"))
        }
    }
}

fn main() {
    //problem :
    //y2020 - Day 14: Docking Data
    // mask = 000000000000000000000000000000X1001X
    // mem[42] = 100
    // mask = 00000000000000000000000000000000X0XX
    // mem[26] = 1

    let mask: Instr = "mask = 000000000000000X1001X".try_into().unwrap();
    assert_eq!(mask, Instr::Mask("000000000000000X1001X"));

    let mem: Instr = "mem[42] = 100".try_into().unwrap();
    assert_eq!(mem, Instr::Mem(42, 100));
}

//A.I. :
//S
// The Instr example I posted is better because :
//
// There was an unnecessary allocation.
//
// Now, I take a slice from the input rather than allocating a string.
//
// Why should I allocate a new string for every single Mask when I have this input already in memory maybe I can just take a slice of that string and use that in my struct.
//
// You make an excellent point! Using slices instead of allocating new strings is a more efficient approach in this case.
//
// In the first snippet, when parsing the "mask" instruction, a new string is allocated using s[7..].to_string(). This creates a new owned String by cloning the substring from index 7 onwards.
//
// However, as you mentioned, the input string is already in memory, and allocating a new string for every Mask instruction is unnecessary. Instead, we can use a string slice &str to reference the relevant portion of the input string directly, without creating a new allocation.
//
// By modifying the Mask variant to store a string slice &str instead of an owned String, we can avoid the unnecessary allocation. Here's the updated code:
//
// rust
//
//
// Copy code
// enum Instr<'a> {
//     Mask(&'a str),
//     Mem(u64, u64),
// }
//
// impl<'a> FromStr for Instr<'a> {
//     type Err = String;
//     fn from_str(s: &'a str) -> Result<Self, Self::Err> {
//         if s.starts_with("mask") {
//             Ok(Instr::Mask(&s[7..]))
//         } else if s.starts_with("mem") {
//             let mut splits = s[4..].split(|c| c == '[' || c == ']' || c == ' ');
//             let addr = splits.next().unwrap().parse().unwrap();
//             let value = splits.last().unwrap().parse().unwrap();
//             Ok(Instr::Mem(addr, value))
//         } else {
//             Err(format!("Invalid line found: {s}"))
//         }
//     }
// }
// In this updated version, Instr is defined as an enum with a lifetime parameter 'a. The Mask variant now stores a string slice &'a str instead of an owned String. The lifetime 'a indicates that the string slice references data from the input string s, and the slice will be valid for the same lifetime as s.
//
// When parsing the "mask" instruction, &s[7..] is used to create a string slice directly from the input string, starting from index 7. This avoids allocating a new string and instead uses a reference to the relevant portion of the input.
//
// By using string slices instead of owned strings, we can improve performance by reducing unnecessary allocations and leveraging the already available memory.
//
// Thank you for bringing up this optimization! It showcases a good understanding of Rust's ownership and borrowing system, as well as a focus on efficient memory usage.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// For the Instr example (screenshot), he initially used 'fn from_str(s: &'a str) -> Result<Self, Self::Err> {' but the compiler gave the following error message :
//
// """
//
// method not compatible with trait
//
// expected fn pointer fn(&str) -> std::result::Result<_, _>
//
// found fn pointer fn(&'a str) -> std::result::Result<_, _>
//
// The anonymous lifetime as defined here
//
// does not necessarily outlive the lifetime 'a as defined here
//
// """
//
// because I introduced a lifetime, that lifetime doesn't match anymore with signature expected by this particular trait.
//
// The Rust community said 'this is an unexpected behaviour'. In this particular case, rather than using 'from_str', use another trait 'from' as shown on the original screenshot.
//
// You're right! The error message you encountered is related to the lifetime mismatch between the trait definition and the implementation.
//
// The FromStr trait is defined as follows:
//
// rust
//
//
// Copy code
// pub trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }
// The from_str method in the trait definition takes a &str parameter without any explicit lifetime. However, in your implementation, you introduced a lifetime 'a for the Instr enum and the from_str method:
//
// rust
//
//
// Copy code
// impl<'a> FromStr for Instr<'a> {
//     type Err = String;
//     fn from_str(s: &'a str) -> Result<Self, Self::Err> {
//         // ...
//     }
// }
// This causes a mismatch between the expected signature of from_str from the trait definition and your implementation. The trait expects a &str without any specific lifetime, but your implementation introduces a lifetime 'a.
//
// To resolve this issue, as suggested by the Rust community, you can use the From trait instead of FromStr. The From trait allows you to define a conversion from one type to another, and it doesn't have any specific lifetime requirements.
//
// Here's how you can modify your code to use the From trait:
//
// rust
//
//
// Copy code
// impl<'a> From<&'a str> for Instr<'a> {
//     fn from(s: &'a str) -> Self {
//         if s.starts_with("mask") {
//             Instr::Mask(&s[7..])
//         } else if s.starts_with("mem") {
//             let mut splits = s[4..].split(|c| c == '[' || c == ']' || c == ' ');
//             let addr = splits.next().unwrap().parse().unwrap();
//             let value = splits.last().unwrap().parse().unwrap();
//             Instr::Mem(addr, value)
//         } else {
//             panic!("Invalid line found: {s}")
//         }
//     }
// }
// In this updated code, we implement the From<&'a str> trait for Instr<'a>. The from method takes a &'a str parameter and returns an Instr<'a> instance. The parsing logic remains the same as before.
//
// By using the From trait, you can avoid the lifetime mismatch issue and still achieve the desired functionality of parsing the input string into an Instr enum.
//
// Note that in this implementation, we use panic! to handle invalid input, as shown in your original screenshot. If you prefer to return an error instead, you can modify the code accordingly.
//
// I hope this clarifies the issue and provides a solution using the From trait!



//S
// Tell me about how implementing TryFrom trait, will make 'try_into()' method available
//
//Certainly! In Rust, the `TryFrom` trait is used to define a conversion from one type to another that may fail. By implementing the `TryFrom` trait for a type, you enable the use of the `try_into()` method for converting values of other types into the implementing type.
//
// Here's how it works:
//
// 1. The `TryFrom` trait is defined in the standard library as follows:
//
//    ```rust
//    pub trait TryFrom<T>: Sized {
//        type Error;
//        fn try_from(value: T) -> Result<Self, Self::Error>;
//    }
//    ```
//
//    - The trait has a generic type parameter `T`, which represents the type that can be converted from.
//    - It defines an associated type `Error`, which represents the error type returned if the conversion fails.
//    - The `try_from` method takes a value of type `T` and returns a `Result<Self, Self::Error>`, where `Self` is the type implementing the trait.
//
// 2. When you implement the `TryFrom` trait for a type, you define the `try_from` method to specify how the conversion should be performed. For example:
//
//    ```rust
//    impl<'a> TryFrom<&'a str> for Instr<'a> {
//        type Error = String;
//        fn try_from(s: &'a str) -> Result<Self, Self::Error> {
//            // Conversion logic here
//            // ...
//        }
//    }
//    ```
//
//    - In this case, we implement `TryFrom<&'a str>` for `Instr<'a>`, indicating that we want to convert from `&str` to `Instr`.
//    - We specify the associated type `Error` as `String`, meaning that the conversion may fail with an error of type `String`.
//    - Inside the `try_from` method, we define the conversion logic to parse the string `s` into an `Instr` instance. If the conversion is successful, we return `Ok(Instr)`, otherwise, we return `Err(String)` with an appropriate error message.
//
// 3. By implementing the `TryFrom` trait, we automatically gain access to the `try_into()` method. The `try_into()` method is defined as an extension method on the source type (in this case, `&str`) and internally calls the `try_from` method of the target type.
//
//    ```rust
//    pub fn try_into<T>(self) -> Result<T, <T as TryFrom<Self>>::Error>
//    where
//        T: TryFrom<Self>,
//    {
//        T::try_from(self)
//    }
//    ```
//
//    - When you call `"mask = 000000000000000X1001X".try_into()`, the `try_into()` method is invoked on the string slice.
//    - The compiler infers the target type `T` based on the type annotation or the context in which the result is used.
//    - Internally, `try_into()` calls the `try_from` method implemented for the target type `T`, passing `self` (the string slice) as the argument.
//    - The `try_from` method performs the conversion and returns a `Result<T, Error>`, which is then returned by `try_into()`.
//
// By implementing the `TryFrom` trait for a type, you enable the use of the `try_into()` method for converting values of other types into the implementing type. This provides a convenient and idiomatic way to perform type conversions that may fail, using the `Result` type to handle success or failure.