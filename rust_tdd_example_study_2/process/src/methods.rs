use std::str::FromStr;
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero is not allowed")
    } else {
        a / b
    }
}
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl FromStr for OperationKind {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "add" => Ok(OperationKind::Add),
            "sub" => Ok(OperationKind::Subtract),
            "mul" => Ok(OperationKind::Multiply),
            "div" => Ok(OperationKind::Divide),
            v => Err(v.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_add_1_to_1_equals_2() {
        assert_eq!(add(1, 1), 2)
    }
    #[test]
    fn test_can_subtract_1_from_1_to_0() {
        assert_eq!(subtract(1, 1), 0)
    }
    #[test]
    fn test_can_subtract_1_from_2_to_negative_1() {
        println!("why so negative?");
        assert_eq!(subtract(1, 2), -1)
    }
    #[test]
    fn test_can_multiply_2_and_3_equals_6() {
        assert_eq!(multiply(2, 3), 6)
    }
    #[test]
    fn test_can_divide_4_by_2_equals_2() {
        let result = divide(4, 2);
        assert_eq!(result, 2)
    }
    #[test]
    #[should_panic]
    fn text_cannot_divide_by_zero() {
        let _result = divide(4, 0);
    }
}