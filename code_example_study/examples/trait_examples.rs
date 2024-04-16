use std::fmt::{Debug, Display, Formatter};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String { //default implementation
        format!("(Read more from {}...)", self.summarize_author())
        //String::from -> format!
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String { using the default implementation
    // format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable_wrong(switch: bool) -> impl Summary { //if else have incompatible types
    // if switch {
    //     NewsArticle {
    //         headline: String::from("penguins win the cup championship!"),
    //         location: String::from("Pittsburgh"),
    //         author: String::from("Iceburgh"),
    //         content: String::from("The Pittsburgh penguins are cute"),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }

    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
// }

// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
// }

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    65
}

pub fn some_function_refactored<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    65
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());
    // notify(&tweet);
    // returns_summarizable_wrong(false);
}