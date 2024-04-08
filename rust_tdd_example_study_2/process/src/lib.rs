use crate::methods::*;

pub mod methods;

pub fn execute(args: Vec<String>) -> i32 {

    if args.len() < 4 {
        panic!("incorrect amount of arguments: {:?}", args)
    }

    let _app_path = args[0].to_string();
    let method = args[1].parse().unwrap();
    let a = args[2].parse().unwrap_or_default();
    let b = args[3].parse().unwrap_or_default();

    let result = match method {
        OperationKind::Add => add(a, b),
        OperationKind::Subtract => subtract(a, b),
        OperationKind::Multiply => multiply(a, b),
        OperationKind::Divide => divide(a, b),
    };

    println!("{} + {} = {}", a, b, result);
    result
}