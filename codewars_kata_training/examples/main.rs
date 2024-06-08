#[cfg(test)]
mod array_diff_tests {
    use either::Either;
    fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        a.into_iter().filter(|x| !b.contains(x)).collect()
    }

    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
        assert_eq!(array_diff(vec![1,2,3], vec![1,2]), vec![3]);
    }
}


#[cfg(test)]
mod disem_vowel_tests {
    use either::Either;

    fn better_disem_vowel(s: &str) -> String {
        s.chars()
            .filter(|c| !"aeiou".contains(c.to_ascii_lowercase()))
            .collect()
    }

    fn disem_vowel(s: &str) -> String {
        s.chars().filter(|c| !is_vowel(c)).collect()
    }

    fn is_vowel(c: &char) -> bool {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }

    #[test]
    fn example_test() {
        assert_eq!(disem_vowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
        assert_eq!(better_disem_vowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}


#[cfg(test)]
mod string_rank_tests {
    use either::Either;
    fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
        if st.is_empty() {
            return "No participants";
        }

        let names: Vec<&str> = st.split(',').collect();

        if n > names.len() {
            return "Not enough participants";
        }

        let mut scores: Vec<(String, i32)> = names
            .into_iter()
            .zip(we.iter())
            .map(|(name, &weight)| {
                let name_score = name.chars().map(|c| c.to_ascii_uppercase() as usize - 64).sum::<usize>();
                let sum = name_score + name.len();
                let winning_number = sum as i32 * weight;
                (name.to_string(), winning_number)
            })
            .collect();

        scores.sort_by(|(name1, score1), (name2, score2)| {
            score2.cmp(score1).then(name1.cmp(name2))
        });

        let result = scores[n - 1].0.clone();
        Box::leak(result.into_boxed_str())
    }

    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(rank(st, we, n), exp)
    }

    #[test]
    fn basics_rank() {
        testing("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4, "Benjamin");
        testing("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2, "Matthew");
        testing("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4, "Abigail");
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }
}


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