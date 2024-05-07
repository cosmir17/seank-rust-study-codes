#![allow(dead_code, unused_variables)]

use std::future::Future;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("first line");

    let x = foo1().await;
    let y= foo2().await;
}


async fn foo1() -> usize {
    println!("foo1 - 1");
    sleep(Duration::from_secs(10)).await;
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo2 - 1");
        foo1().await;
        read_to_string().await;
        println!("foo2 - 2");
        0
    }
}

async fn read_to_string() -> String {
    let mut file = File::open("code_example_study/await-async-file.txt").await.expect("Unable to open await-async-file.txt file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.expect("Unable to read file");
    sleep(Duration::from_secs(5)).await;
    println!("file read and returning string");
    contents
}
