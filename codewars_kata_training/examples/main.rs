#[cfg(test)]
mod div_con_tests {
    use either::Either;
    fn div_con(arr: &[Either<i32, String>]) -> i32 {
        arr.iter().fold(0, |acc, item| {
            match item {
                Either::Left(num) => acc + num,
                Either::Right(s) => acc - s.parse::<i32>().unwrap_or(0),
            }
        })
    }

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = div_con(arr);
        assert!(actual == expected, "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[either::Left(9), either::Left(3), either::Right("7".to_string()), either::Right("3".to_string())], 2);
        dotest(&[Either::Right("5".to_string()), Either::Right("0".to_string().to_string()), Either::Left(9), Either::Left(3), Either::Left(2), Either::Left(1), Either::Right("9".to_string()), Either::Left(6), Either::Left(7)], 14);
        dotest(&[Either::Right("3".to_string()), Either::Left(6), Either::Left(6), Either::Left(0), Either::Right("5".to_string()), Either::Left(8), Either::Left(5), Either::Right("6".to_string()), Either::Left(2), Either::Right("0".to_string())], 13);
        dotest(&[Either::Right("1".to_string()), Either::Right("5".to_string()), Either::Right("8".to_string()), Either::Left(8), Either::Left(9), Either::Left(9), Either::Left(2), Either::Right("3".to_string())], 11);
        dotest(&[Either::Left(8), Either::Left(0), Either::Left(0), Either::Left(8), Either::Left(5), Either::Left(7), Either::Left(2), Either::Left(3), Either::Left(7), Either::Left(8), Either::Left(6), Either::Left(7)], 61);
    }
}


fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

fn narcissistic(num: u64) -> bool {
    (num.to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .map(|n| (n as u64).pow(num.to_string().len() as u32))
        .sum::<u64>()) == num
}

fn main() {
}

#[cfg(test)]
mod narcissistic_tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(actual, expected, "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}", input)
    }

    #[test]
    fn basic_tests() {
        dotest(   7,  true);
        dotest( 371,  true);
        dotest( 122, false);
        dotest(4887, false);
    }
}

#[cfg(test)]
mod last_word_tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
        assert_eq!(true, solution("hello", "lo"));
    }
}