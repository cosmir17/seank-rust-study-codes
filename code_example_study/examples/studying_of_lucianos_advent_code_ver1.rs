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

use std::str::FromStr;

enum Instr {
    Mask(String),
    Mem(u64, u64),
}

impl FromStr for Instr { //implementing FromStr trait, will make the parse() method available
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Instr::Mask(s[7..].to_string()))
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
    // ...

    let mask: Instr = "mask = 000000000000000X1001X".parse().unwrap();
    assert_eq!(mask, Instr::Mask("000000000000000X1001X".to_string()));

    let mem: Instr = "mem[42] = 100".parse().unwrap();
    assert_eq!(mem, Instr::Mem(42, 100));
}

//A.I.
//Tell me how implementing FromStr trait, will make the parse() method available.
//
//
// Copy code
// let mask: Instr = "mask = 000000000000000X1001X".parse().unwrap();
// assert_eq!(mask, Instr::Mask("000000000000000X1001X".to_string()));
//
// let mem: Instr = "mem[42] = 100".parse().unwrap();
// assert_eq!(mem, Instr::Mem(42, 100));
// In Rust, the FromStr trait is used to parse a string into a custom type. By implementing the FromStr trait for your custom type, you automatically gain access to the parse() method, which allows you to convert a string to an instance of your type.
//
// Here's how it works:
//
// The FromStr trait is defined in the standard library as follows:
// rust
//
//
// Copy code
// pub trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }
// It has an associated type Err for the error type and a from_str method that takes a string slice (&str) and returns a Result<Self, Self::Err>, where Self is the type implementing the trait.
// When you implement the FromStr trait for your custom type Instr, you define the from_str method to parse a string into an Instr instance. In your case, it looks like this:
// rust
//
//
// Copy code
// impl FromStr for Instr {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.starts_with("mask") {
//             Ok(Instr::Mask(s[7..].to_string()))
//         } else if s.starts_with("mem") {
//             let addr_value = &mut s[4..].split(|c| c == '[' || c == ']' || c == ' ');
//             let addr: u64 = addr_value.next().unwrap().parse().unwrap();
//             let value: u64 = addr_value.next().unwrap().parse().unwrap();
//             Ok(Instr::Mem(addr, value))
//         } else {
//             Err(format!("Invalid line found: {s}"))
//         }
//     }
// }
// The from_str method parses the input string and returns either an Ok(Instr) if the parsing is successful or an Err(String) if an invalid line is encountered.
// By implementing the FromStr trait for Instr, you automatically gain access to the parse() method on string slices (&str). The parse() method is defined in the standard library and internally calls the from_str method of the type specified as the type parameter. So, when you write:
// rust
//
//
// Copy code
// let mask: Instr = "mask = 000000000000000X1001X".parse().unwrap();
// The parse() method is called on the string slice "mask = 000000000000000X1001X", which internally invokes the from_str method of the Instr type. If the parsing is successful, an Instr instance is returned. The unwrap() method is used to extract the value from the Result returned by parse(), assuming the parsing is always successful. If the parsing fails, unwrap() will panic.
// By implementing the FromStr trait, you provide a way to parse strings into your custom type, and the parse() method becomes available automatically, making it convenient to convert strings to instances of your type.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// What does 'impl FromStr for Instr {' mean? how parse() method will yield Instr type? show me internal mechanism of FromStr trait.
//
// The line impl FromStr for Instr { means that you are implementing the FromStr trait for your custom type Instr. This implementation block defines how the from_str method should parse a string into an Instr instance.
//
// When you call the parse() method on a string slice, it internally calls the from_str method of the type specified as the type parameter. In this case, when you write "mask = 000000000000000X1001X".parse(), the parse() method is called with the type parameter Instr, so it invokes the from_str method defined in the FromStr implementation for Instr.
//
// Here's a step-by-step explanation of the internal mechanism of the FromStr trait:
//
// The FromStr trait is defined in the standard library as follows:
// rust
//
//
// Copy code
// pub trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }
// The trait has an associated type Err that represents the error type returned if the parsing fails.
// The from_str method takes a string slice (&str) as input and returns a Result<Self, Self::Err>, where Self is the type implementing the trait.
// When you implement the FromStr trait for Instr, you define the from_str method specific to parsing strings into Instr instances:
// rust
//
//
// Copy code
// impl FromStr for Instr {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Parsing logic here
//         // ...
//     }
// }
// You specify the associated type Err as String, indicating that parsing errors will be represented as String values.
// Inside the from_str method, you write the parsing logic to convert the input string slice into an Instr instance.
// If the parsing is successful, you return Ok(Instr) with the parsed Instr value.
// If the parsing fails, you return Err(String) with an error message.
// The parse() method is defined in the standard library for string slices (&str). It is a generic method with a type parameter F that specifies the type to parse into. The parse() method internally calls the from_str method of the specified type.
// rust
//
//
// Copy code
// pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
//     F::from_str(self)
// }
// When you call "mask = 000000000000000X1001X".parse(), the type parameter F is inferred as Instr based on the type annotation let mask: Instr = ....
// The parse() method internally calls Instr::from_str(self), passing the string slice as the argument to the from_str method defined in the FromStr implementation for Instr.
// The from_str method parses the string slice and returns a Result<Instr, String>.
// If the parsing is successful, Ok(Instr) is returned, and you can use unwrap() or other Result methods to extract the parsed Instr value.
// If the parsing fails, Err(String) is returned with an error message.
// By implementing the FromStr trait for your custom type Instr, you define how strings should be parsed into Instr instances. The parse() method, which is available on string slices, internally calls the from_str method defined in the trait implementation, allowing you to convert strings to Instr values using the parse() method.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// Here :
//
// """
//
// The line impl FromStr for Instr { means that you are implementing the FromStr trait for your custom type Instr. This implementation block defines how the from_str method should parse a string into an Instr instance.
//
// """
//
// From FromStr code, how does FromStr​ know it needs to return Instr instance? How does 'for' keyword works here?
//
// Great question! The FromStr trait itself doesn't know anything about the specific type it's being implemented for. The knowledge of returning an Instr instance comes from the implementation of the trait for the Instr type.
//
// Let's break down the line impl FromStr for Instr { to understand how it works:
//
// The impl keyword is used to define an implementation block. It's followed by the name of the trait being implemented, in this case, FromStr.
// The for keyword is used to specify the type for which the trait is being implemented. In this case, it's the Instr type.
// The { opens the implementation block, which contains the definition of the trait methods for the specific type.
// So, the line impl FromStr for Instr { tells the Rust compiler that you are providing an implementation of the FromStr trait specifically for the Instr type.
//
// Inside the implementation block, you define the from_str method, which is the only required method of the FromStr trait:
//
// rust
//
//
// Copy code
// fn from_str(s: &str) -> Result<Self, Self::Err>;
// Here, Self is a placeholder that refers to the type for which the trait is being implemented. In this case, Self represents the Instr type.
//
// When you implement the FromStr trait for Instr, the from_str method signature becomes:
//
// rust
//
//
// Copy code
// fn from_str(s: &str) -> Result<Instr, String>;
// The from_str method takes a string slice (&str) as input and returns a Result enum. The Result enum has two variants: Ok and Err. In this case, the Ok variant wraps an instance of Instr (the successfully parsed value), and the Err variant wraps a String (the error message if parsing fails).
//
// Inside the from_str method, you write the parsing logic specific to the Instr type. You define how to parse different string patterns into the corresponding Instr variants (Mask or Mem). If the parsing is successful, you return Ok(Instr::Variant(...)) with the parsed Instr instance. If the parsing fails, you return Err("error message") with an appropriate error message.
//
// By implementing the FromStr trait for Instr and defining the from_str method, you are telling the Rust compiler how to parse a string into an Instr instance. The from_str method is responsible for constructing and returning the Instr instance based on the input string.
//
// The for keyword in the trait implementation specifies the type for which the trait is being implemented, establishing the connection between the trait and the specific type (Instr in this case).
//
// I hope this clarifies how the impl FromStr for Instr syntax works and how the from_str method returns an Instr instance based on the parsing logic you define in the implementation.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// I am from Scala background, so help me here.
//
// """
//
// So, the line impl FromStr for Instr { tells the Rust compiler that you are providing an implementation of the FromStr trait specifically for the Instr type.
//
// """
//
// How come implementation of FromStr type is same as Instr?
//
// Apologies for the confusion. Let me clarify the relationship between the FromStr trait and the Instr type in a way that relates to your Scala background.
//
// In Scala, you have type classes, which are similar to traits in Rust. A type class defines a set of methods that can be implemented for different types. When you provide an implementation of a type class for a specific type, you are essentially creating an instance of the type class for that type.
//
// In Rust, the FromStr trait is analogous to a type class in Scala. It defines a set of methods (in this case, only the from_str method) that can be implemented for different types.
//
// When you write impl FromStr for Instr, you are providing an implementation of the FromStr trait for the Instr type. This is similar to creating an instance of the FromStr type class for the Instr type in Scala.
//
// The impl FromStr for Instr block defines how the methods of the FromStr trait should behave specifically for the Instr type. Inside this block, you provide the implementation of the from_str method, which defines how to parse a string into an Instr instance.
//
// In Scala, you might define a type class and provide an implementation for a specific type like this:
//
// scala
//
//
// Copy code
// trait FromStr[A] {
//   def fromStr(s: String): Option[A]
// }
//
// case class Instr(/* ... */)
//
// implicit object InstrFromStr extends FromStr[Instr] {
//   def fromStr(s: String): Option[Instr] = {
//     // Parsing logic here
//     // ...
//   }
// }
// In the above Scala code, FromStr is a type class, and InstrFromStr is an instance of the FromStr type class for the Instr type.
//
// Similarly, in Rust, FromStr is a trait, and impl FromStr for Instr provides an implementation of the FromStr trait for the Instr type.
//
// The implementation of FromStr for Instr is not the same as Instr itself. Instr is a separate type (like an enum or struct) that represents the structure and variants of the Instr data. The FromStr implementation for Instr defines how to parse a string into an Instr instance, but it doesn't change the definition or behavior of the Instr type itself.
//
// I hope this explanation helps clarify the relationship between the FromStr trait and the Instr type, and how it relates to the concept of type classes in Scala.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// This was what I meant, 'trait FromStr[A] {...}', 'FromStr[Instr]'. I understand better.
//
// My next question, where does does FromStr trait define [A] in the Rust code?
//
// In Rust, the FromStr trait is defined in the standard library as follows:
//
// rust
//
//
// Copy code
// pub trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }
// In this definition, the FromStr trait doesn't explicitly define a type parameter like [A] as in Scala. Instead, it uses the Self keyword, which is a special type alias that refers to the type implementing the trait.
//
// When you implement the FromStr trait for a specific type, like impl FromStr for Instr, the Self keyword within the trait definition automatically refers to the Instr type.
//
// So, when you write the from_str method in the implementation block:
//
// rust
//
//
// Copy code
// impl FromStr for Instr {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Parsing logic here
//         // ...
//     }
// }
// The Self keyword in Result<Self, Self::Err> is replaced with the actual type you're implementing the trait for, which is Instr in this case. Therefore, the from_str method returns a Result<Instr, String>, where Instr is the successfully parsed value, and String is the error type.
//
// In Rust, the type parameter is implicitly defined by the type you're implementing the trait for, rather than explicitly specifying it like [A] in Scala.
//
// However, if you want to define a trait with a type parameter in Rust, you can do so like this:
//
// rust
//
//
// Copy code
// trait MyTrait<T> {
//     fn my_method(&self, value: T) -> Result<T, String>;
// }
// In this case, MyTrait has a type parameter T, which can be specified when implementing the trait for a specific type:
//
// rust
//
//
// Copy code
// impl MyTrait<String> for MyType {
//     fn my_method(&self, value: String) -> Result<String, String> {
//         // Method implementation here
//         // ...
//     }
// }
// I hope this clarifies how Rust handles type parameters in traits and how it relates to the FromStr trait specifically.
//
// Copy
//
//
// Claude does not have the ability to run the code it generates yet.
// S
// so in Rust, the self type doesn't indicate the implementation of Trait, the type which is implemented 'for'(the for keyword in e.g. 'impl FromStr for Instr {') ?
// if '1' is true, is 'self' and 'Self type is MyType not String'? in the following code : 'impl MyTrait<String> for MyType { fn my_method(&self, value: String) -> Result<Self, String> {'
// Yes, you are correct. In Rust, the Self type within a trait definition refers to the type that is implementing the trait, not the trait itself. When you write impl FromStr for Instr, the Self type inside the FromStr trait definition will be replaced with Instr in the context of that specific implementation.
// Yes, your understanding is correct. In the code snippet you provided:
// rust
//
//
// Copy code
// impl MyTrait<String> for MyType {
//     fn my_method(&self, value: String) -> Result<Self, String> {
//         // Method implementation here
//         // ...
//     }
// }
// The self parameter in the method signature fn my_method(&self, value: String) refers to an instance of MyType. It allows you to access the methods and properties of MyType within the method implementation.
// The Self type in Result<Self, String> also refers to MyType, not String. In this context, Self is a type alias for the type that is implementing the trait, which is MyType.
// So, in the above code, my_method takes an instance of MyType as &self and a String value as value, and it returns a Result<MyType, String>, where MyType is the success variant and String is the error variant. The type parameter <String> in impl MyTrait<String> for MyType is used to specify the concrete type for the generic type parameter T in the trait definition trait MyTrait<T>. It means that, in this specific implementation, T is replaced with String. However, it doesn't affect the meaning of Self within the trait implementation.
// I hope this clarifies the distinction between self, Self, and the type parameter in the trait implementation.