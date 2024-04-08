// trait BBB {}
//
// trait AAA : BBB {}
//
//
// fn main() {
    // let mut x = vec![1, 2, 3];
    //
    // x.push(4);
    // let last= x[x.len() - 1];
    //
    // println!("{:?}", last)
// }

// fn recursive(x: usize){
//     if x == 0 {
//         return;
//     }
//     let mut stuff = vec![0; x];
//     for i in 0..x {
//         stuff.push(i);
//     }
//     recursive(x - 1)
// }

// use std::collections::HashMap;

// mod code_example_study;

// struct MyString<'a> {
//     text: &'a str
// }

// fn main() {
// let str1 = String::from("this is my string");
// let b: &str = str1.as_str();
// let x = MyString{ text: b };
// println!("{:?}", b);
//
// let mut scores = HashMap::new();
// scores.get()
// recursive(50_000);

// let string1 = String::from("abcd");
// let string2 = String::from("xyz");
//
// let result = longest(string1.as_str(), string2.as_str());
// println!("The longest string is {}", result);

// let string1 = String::from("abcd");
// let result: &str;
// {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
// }
// println!("The longest string is {}", result);


// }

// fn longest<'a>(x: &'a str, y: & str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest(x: &str) -> &str {
//     x
// }

// fn print_list<T: std::fmt::Display>(list: &[T]) {
//     for item in list {
//         println!("{}", item);
//     }
// }
//
// fn is_string_list(list: &[impl std::any::Any]) -> bool {
//     list.iter().all(|item| item.is::<String>())
// }
//
// fn main() {
//     println!("{}", is_string_list(&[1, 2, 3])); // False
//     print_list(&["a", "b", "c"]);              // Prints a, b, c
// }

// pub fn calculate<F>(bar: F) -> i32 where F: FnMut(i32) -> i32 {
//     (0..5).map(bar).sum()
// }
//
// pub fn f(x: i32) -> i32 {
//     x + x
// }
//
// pub fn f_prime(x: i32) -> i32 {
//     x + x + x
// }
//
// fn main() {
//     calculate(f);
// }

// trait Overview {
//     fn overview(&self) -> String {
//         String::from("This is a Rust course")
//     }
// }
//
// struct Course {
//     headline: String,
//     author: String
// }
//
// struct AnotherCourse {
//     headline: String,
//     author: String
// }
//
// impl Overview for Course {}
//
// impl Overview for AnotherCourse {
//     fn overview(&self) -> String {
//         format!("{}, {}", self.author, self.headline)
//     }
// }

// fn call_overview<T: Overview>(item: &T) {
// things that implemented Overview,
// different from Context Bound,
// it's upper bound T <: Overview

//     println!("Overview: {}", item.overview())
// }
//
// fn call_overview_verbose(item: &impl Overview) { //things that implemented Overview
//     println!("Overview: {}", item.overview())
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T
// }
//
// pub trait CustomAdd<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
//
// impl<T> CustomAdd for Point<T> where T: CustomAdd<Output = T> {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y
//         }
//     }
// }

// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::collections::{binary_heap, BinaryHeap, HashMap};
// #[allow(dead_code)]

// mod find_and_replace;
// mod closure;

// fn main() {
    // find_and_replace::run();
    // closure::closure_fn_course();
    // let double = |x| x * 2;
    // let result = closure::fn_closure(5, double);
    // println!("Result: {}", result);  // Output: Result: 10

    // let mut count = 0;
    // let mut increment = |x| {
    //     count += x;
    //     println!("Count: {}", count);
    // };
    //
    // closure::fnmut_closure(5, &mut increment);  // Output: Count: 5
    // closure::fnmut_closure(3, &mut increment);  // Output: Count: 8

    // let file_path = "example.txt";
    // // let search_term = "Rust";
    // let search_term = String::from("Rust");
    //
    // closure::process_file(file_path, |contents| {
    //     if contents.contains(&search_term) {
    //         println!("Found '{}' in the file!", search_term);
    //         Ok(())
    //     } else {
    //         println!("Error here 2");
    //         Err(std::io::Error::new(
    //             std::io::ErrorKind::Other,
    //             format!("'{}' not found in the file.", search_term),
    //         ))
    //     }
    // });

    // println!("{}", search_term);

    // let x = 42;
    // let y = x;  // The value of `x` is automatically copied to `y` because integers implement `Copy`
    // println!("x: {}, y: {}", x, y);  // Output: x: 42, y: 42
    // let foo = String::from("Hello, ");
    // bar(foo);
// }
//
// fn bar(mut foo: String) {
//     foo.push_str("world!");
//     println!("{foo}")
// }


// let numbers = vec![1, 2, 3, 4, 5];

    // for num in &numbers {
    //     if num % 2 == 0 {
    //         numbers.retain(|&x| x != *num);
    //     }
    // }

    // let odd_number: Vec<i32> = numbers.into_iter().filter(|&num| num % 2 != 0).collect();
    //
    // println!("{:?}", odd_number);


    // let a = &numbers;
    // numbers.retain(|&x| x != *&5);
    //
    // a.first();
    // let mut nums: Vec<i32> = vec![];
    // let mut strNums: Vec<String> = vec![];
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);
    // strNums.push(String::from("hello"));
    // strNums.push(String::from("world"));
    //
    // let pop = nums.pop();
    // println!("{:?}", pop);
    //
    // let two = nums[1];
    // let twoString = &strNums[1];
    // // &nums[1], creates a reference if copy is not available
    // println!("{}", two);
    // println!("{}", twoString);
    //
    // let one = nums.first();
    // println!("{:?}", one);
    //
    // nums.shuffle(&mut thread_rng());
    // println!("shuffled values are : {:?}", nums);
    //
    // let mut bheap = BinaryHeap::new();
    // bheap.push(1);
    // bheap.push(15);
    // bheap.push(31);
    // bheap.push(11);
    // bheap.push(5);
    // bheap.push(3);
    //
    // // bheap.pop();
    //
    // println!("{:?}", bheap);
    // println!("{:?}", bheap.peek());
    // println!("{:?}", bheap);
    //
    // let mut hm = HashMap::new();
    // hm.insert(1, String::from("hello"));
    // hm.insert(2, String::from("world"));
    // let old = hm.insert(2, String::from("different"));
    // println!("map test: {:?}", old);
    // hm.get(&2);



    // let coord = Point { x: 5.0, y: 5.0 };
    // let coord2 = Point { x: 1.0, y: 2.0 };
    // let sum = coord + coord2;
    // println!("{:?}", sum);
    // let course1 = Course{ headline: String::from("h"), author: String::from("author")};
    // let course2 = AnotherCourse{ headline: String::from("ah"), author: String::from("a-author")};

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);
    // let some_number = Some(5);
    // let _some_string = Some("hello abc");
    // let _some_string_from_from = Some(String::from("hello abc"));

    // let some_number: Option<i32> = None;
    // let number = 6;
    // let sum = some_number.map(|n| n + number);
    // println!("{:?}", sum);
    //
    // match sum {
    //     Some(n) => println!("{:}", n),
    //     None => println!("{:}", "no value provided")
    // }

    // enum Alpha {
    //     A, B, C
    // }

    // let my_alphabet = Alpha::A;

    // if let Some(Alpha::B) = my_alphabet {
    //     println!("{}", "matched")
    // } else {
    //     println!("{}", "not matched")
    // }

    // if Alpha::B == my_alphabet {
    //     println!("{}", "matched")
    // } else {
    //     println!("{}", "not matched")
    // }

// trait BBB {}
//
// trait AAA : BBB {}
//
//
// fn main() {
// let mut x = vec![1, 2, 3];
//
// x.push(4);
// let last= x[x.len() - 1];
//
// println!("{:?}", last)
// }

// fn recursive(x: usize){
//     if x == 0 {
//         return;
//     }
//     let mut stuff = vec![0; x];
//     for i in 0..x {
//         stuff.push(i);
//     }
//     recursive(x - 1)
// }

// use std::collections::HashMap;

// mod code_example_study;

// struct MyString<'a> {
//     text: &'a str
// }

// fn main() {
// let str1 = String::from("this is my string");
// let b: &str = str1.as_str();
// let x = MyString{ text: b };
// println!("{:?}", b);
//
// let mut scores = HashMap::new();
// scores.get()
// recursive(50_000);

// let string1 = String::from("abcd");
// let string2 = String::from("xyz");
//
// let result = longest(string1.as_str(), string2.as_str());
// println!("The longest string is {}", result);

// let string1 = String::from("abcd");
// let result: &str;
// {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
// }
// println!("The longest string is {}", result);


// }

// fn longest<'a>(x: &'a str, y: & str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest(x: &str) -> &str {
//     x
// }

// fn print_list<T: std::fmt::Display>(list: &[T]) {
//     for item in list {
//         println!("{}", item);
//     }
// }
//
// fn is_string_list(list: &[impl std::any::Any]) -> bool {
//     list.iter().all(|item| item.is::<String>())
// }
//
// fn main() {
//     println!("{}", is_string_list(&[1, 2, 3])); // False
//     print_list(&["a", "b", "c"]);              // Prints a, b, c
// }

// pub fn calculate<F>(bar: F) -> i32 where F: FnMut(i32) -> i32 {
//     (0..5).map(bar).sum()
// }
//
// pub fn f(x: i32) -> i32 {
//     x + x
// }
//
// pub fn f_prime(x: i32) -> i32 {
//     x + x + x
// }
//
// fn main() {
//     calculate(f);
// }

// trait Overview {
//     fn overview(&self) -> String {
//         String::from("This is a Rust course")
//     }
// }
//
// struct Course {
//     headline: String,
//     author: String
// }
//
// struct AnotherCourse {
//     headline: String,
//     author: String
// }
//
// impl Overview for Course {}
//
// impl Overview for AnotherCourse {
//     fn overview(&self) -> String {
//         format!("{}, {}", self.author, self.headline)
//     }
// }

// fn call_overview<T: Overview>(item: &T) {
// things that implemented Overview,
// different from Context Bound,
// it's upper bound T <: Overview

//     println!("Overview: {}", item.overview())
// }
//
// fn call_overview_verbose(item: &impl Overview) { //things that implemented Overview
//     println!("Overview: {}", item.overview())
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T
// }
//
// pub trait CustomAdd<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
//
// impl<T> CustomAdd for Point<T> where T: CustomAdd<Output = T> {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y
//         }
//     }
// }

// use std::cell::{Ref, RefCell};
// use std::iter::Iterator;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::collections::{binary_heap, BinaryHeap, HashMap};
// #[allow(dead_code)]

// mod find_and_replace;
// mod closure;
// use std::rc::Rc;

// #[derive(Debug)]
// struct Range {
//     start: u32,
//     end: u32
// }
//
// #[derive(Debug)]
// struct Item {
//     name: String
// }

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for Range {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> { //where does this method come from?
//         if self.start >= self.end {
//             return None;
//         }
//         let result = Some(self.start);
//         self.start += 1;
//         result
//     }
// }
//
// fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
//     items.into_iter().filter(|i| i.name == product).collect()
// }

// struct ABC{num: u32}
// struct EEE(u32);

// #[derive(Debug)]
// struct RRR {
//     num: u32,
//     flag: bool,
// }
//
// impl Default for RRR {
//     fn default() -> Self {
//         RRR {
//             num: 0,
//             flag: false,
//         }
//     }
// }
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// use std::ops::Deref;
// struct MyBox<T>(T);
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
    // let x = 5;
    // let y = MyBox::new(x);
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    //
    // let s = &MyBox::new(String::from("Hello"));
    // let strings = &String::from("world");
    // hello(s);
    // hello(strings);
    // *s;
    // *strings;
    // a.deref();

    // let abc = RRR { num: 32, flag: false };
    // println!("{}", abc.num);
    // println!("{}", abc.flag);
    // println!("{:?}", abc);
    //
    // RRR::default(); //'for RRR' makes this possible

    // ABC::new{32};
    // ABC{num:32};
    // ABC{num: 32};
    // EEE(33);
    // let abc = RRR { num: 32, ..Default::default() };
    // let vec = vec![1 ,2, 3];
    // let mut iter = (&vec).into_iter();
    // while let Some(v) = iter.next() {
    //     println!("{}", v);
    // }

    // for val in vec.iter() {
    //     println!("{}", val)
    // }

    // let mut vec: Vec<Item> = Vec::new();
    // vec.push(Item {
    //     name: String::from("coat")
    // });
    // vec.push(Item {
    //     name: String::from("shirt")
    // });

    // let checked = check_inventory(vec, String::from("shoes"));
    // println!("{:?}", checked);
    // let mut range = Range {start: 0, end: 10};
    // // for r in range {
    // //     println!("{}", r);
    // // }
    // let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    // println!("{:?}", vec);
// }

    // find_and_replace::run();
    // closure::closure_fn_course();
    // let double = |x| x * 2;
    // let result = closure::fn_closure(5, double);
    // println!("Result: {}", result);  // Output: Result: 10

    // let mut count = 0;
    // let mut increment = |x| {
    //     count += x;
    //     println!("Count: {}", count);
    // };

    // closure::fnmut_closure(5, &mut increment);  // Output: Count: 5
    // closure::fnmut_closure(3, &mut increment);  // Output: Count: 8

    // let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    // for item in a.into_iter() {
    //     println!("{}", item);
    // }
// The original vector `a` is no longer accessible because it has been consumed.

    // println!("{:?}", a);

    // let mut a: Vec<i32> = vec![1, 2, 3, 4, 5];
    // for item in a.iter_mut() {
    //     *item *= 2;
    // }
    // println!("{:?}", a);
// Output: [2, 4, 6, 8, 10]
// The original vector `a` is modified in-place.

    // let e:i32 = 42;
    // let i = e;         // Implicitly copies the value of `x` to `y`
    // let o = e.copy();       // Explicitly copies the value of `x` to `z`
    // println!("{}", e);
    //
    // let y1 = String::from("Hello");
    // let y2 = y1.clone();  // Creates a new instance of `String` with a deep copy of the data
    //
    // let t = (12, "eggs");
    // let b = Box::new(t);
    // println!("{:?}", b);
    //
    // let x = 5;
    // let y = &x;
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    //
    // let x = 5;
    // let y = Box::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    //
    // println!("{:?}", y);
    //
    // let s1 = Rc::new(String::from("Pointers"));
    // let s2 = s1.clone();
    // let s3 = s2.clone();
    //
    // println!("{} {} {}", s1.contains("Point"), s2, s3.contains("ter"));
    // println!("s1 how many : {}", Rc::strong_count(&s1));
    // println!("s2 how many : {}", Rc::strong_count(&s2));
    // println!("s3 how many : {}", Rc::strong_count(&s3));
    // struct Flagger {
    //     is_true: Rc<RefCell<bool>>
    // }
    //
    // let flag = Flagger {is_true: Rc::new(RefCell::new(true))};
    //borrow returns Ref<T>
    //borrow_mut returns RefMut<T>

    // let reference = flag.is_true.borrow(); //panic!
    // let reference = flag.is_true.clone();
    // let reference = Rc::new(flag.is_true.clone());
    // println!("{:?}", reference);

    // let mut mut_ref = reference.borrow_mut();
    // let mut mut_ref = flag.is_true.borrow_mut();
    // *mut_ref = false; //dereference first access inside
    // println!("{}", mut_ref);

    //rc enables multiple owners with the same data, whereas box and
    //refcell only have single owners
    //box allows immutable and mutable borrows checked at compile time
    //rc allows only immutable borrows checked at compile time
    //refcell allows immutable or mutable borrow checked at run time
    //because refcell allows mutable borrows checked at runtime
    //you can mutate the value inside refcell even when the refcell is immutable
// }




// let numbers = vec![1, 2, 3, 4, 5];

// for num in &numbers {
//     if num % 2 == 0 {
//         numbers.retain(|&x| x != *num);
//     }
// }

// let odd_number: Vec<i32> = numbers.into_iter().filter(|&num| num % 2 != 0).collect();
//
// println!("{:?}", odd_number);


// let a = &numbers;
// numbers.retain(|&x| x != *&5);
//
// a.first();
// let mut nums: Vec<i32> = vec![];
// let mut strNums: Vec<String> = vec![];
// nums.push(1);
// nums.push(2);
// nums.push(3);
// strNums.push(String::from("hello"));
// strNums.push(String::from("world"));
//
// let pop = nums.pop();
// println!("{:?}", pop);
//
// let two = nums[1];
// let twoString = &strNums[1];
// // &nums[1], creates a reference if copy is not available
// println!("{}", two);
// println!("{}", twoString);
//
// let one = nums.first();
// println!("{:?}", one);
//
// nums.shuffle(&mut thread_rng());
// println!("shuffled values are : {:?}", nums);
//
// let mut bheap = BinaryHeap::new();
// bheap.push(1);
// bheap.push(15);
// bheap.push(31);
// bheap.push(11);
// bheap.push(5);
// bheap.push(3);
//
// // bheap.pop();
//
// println!("{:?}", bheap);
// println!("{:?}", bheap.peek());
// println!("{:?}", bheap);
//
// let mut hm = HashMap::new();
// hm.insert(1, String::from("hello"));
// hm.insert(2, String::from("world"));
// let old = hm.insert(2, String::from("different"));
// println!("map test: {:?}", old);
// hm.get(&2);



// let coord = Point { x: 5.0, y: 5.0 };
// let coord2 = Point { x: 1.0, y: 2.0 };
// let sum = coord + coord2;
// println!("{:?}", sum);
// let course1 = Course{ headline: String::from("h"), author: String::from("author")};
// let course2 = AnotherCourse{ headline: String::from("ah"), author: String::from("a-author")};

// println!("{}", course1.overview());
// println!("{}", course2.overview());

// call_overview(&course1);
// call_overview(&course2);
// let some_number = Some(5);
// let _some_string = Some("hello abc");
// let _some_string_from_from = Some(String::from("hello abc"));

// let some_number: Option<i32> = None;
// let number = 6;
// let sum = some_number.map(|n| n + number);
// println!("{:?}", sum);
//
// match sum {
//     Some(n) => println!("{:}", n),
//     None => println!("{:}", "no value provided")
// }

// enum Alpha {
//     A, B, C
// }

// let my_alphabet = Alpha::A;

// if let Some(Alpha::B) = my_alphabet {
//     println!("{}", "matched")
// } else {
//     println!("{}", "not matched")
// }

// if Alpha::B == my_alphabet {
//     println!("{}", "matched")
// } else {
//     println!("{}", "not matched")
// }

// fn main() {
//     trait Animal {
//         fn sound(&self);
//     }
//
//     struct Dog;
//     struct Cat;
//
//     impl Animal for Dog {
//         fn sound(&self) {
//             println!("Woof!");
//         }
//     }
//
//     impl Animal for Cat {
//         fn sound(&self) {
//             println!("Meow!");
//         }
//     }
//
//     fn static_dispatch<T: Animal>(animal: &T) {
//         animal.sound();
//     }
//
//     fn main() {
//         let dog = Dog;
//         let cat = Cat;
//
//         let animals = vec![dog, cat];
//
//         for animal in &animals {
//             static_dispatch(animal);
//         }
//     }
// }

#[tokio::main]
async fn main() {
    async fn fetch_data_async(id: i32) -> String {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        format!("Data {}", id)
    }

    async fn main() {
        let a = fetch_data_async(0);

        let futures = vec![
            fetch_data_async(1),
            fetch_data_async(2),
            fetch_data_async(3),
        ];

        let results = futures::future::join_all(futures).await;

        for data in results {
            println!("Data: {}", data);
        }
    }
}
