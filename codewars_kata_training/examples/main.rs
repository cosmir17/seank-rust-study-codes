
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