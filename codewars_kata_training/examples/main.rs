#[cfg(test)]
mod array_diff_tests {
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
mod human_readable_duration_fomatter_tests {

    //copied from solution
    fn format_duration(seconds: u64) -> String {
        let result = [
            ("year", 31536000, 100000),
            ("day", 86400, 365),
            ("hour", 3600, 24),
            ("minute", 60, 60),
            ("second", 1, 60),
        ].iter()
            .map(|(unit, duration, modulo)| {
                return ((seconds / duration) % modulo, unit)
            })
            .filter_map(|(duration, unit)| match duration {
                _ if duration == 1 => Some(format!("{} {}", duration, unit)),
                _ if duration != 0 => Some(format!("{} {}s", duration, unit)),
                _ => None,
            }).collect::<Vec<String>>();

        match result.len() {
            0 => String::from("now"),
            1 => result.join(""),
            _ => result
                .split_last()
                .map(|(r, l)| l.join(", ") + " and " + r)
                .unwrap(),
        }
    }

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}

#[cfg(test)]
mod pete_the_baker_tests {
    use std::collections::HashMap;

    fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
        recipe
            .into_iter()
            .map(|(ingredient, &required_amount)| {
                let available_amount = available.get(ingredient).unwrap_or(&0);
                return available_amount / required_amount;
            })
            .min()
            .unwrap_or(0)
    }


    macro_rules! map {
        () => { HashMap::new() };
        ($($ingredient:ident : $amount:expr),+) => {{
            let mut map = HashMap::new();
            $( map.insert(stringify!($ingredient), $amount); )*
            map
        }};
    }

    fn test(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>, expected: u32) {
        let actual = cakes(recipe, available);
        assert!(actual == expected, "Expected to bake {expected} cakes, but got {actual} cakes instead.\nAvailable ingredients:\n  {available:?}\nRecipe:\n  {recipe:?}\n\n");
    }

    #[test]
    fn test_example() {
        let recipe = map!(flour: 500, sugar: 200, eggs: 1);
        let available = map!(flour: 1200, sugar: 1200, eggs: 5, milk: 200);
        test(&recipe, &available, 2);

        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000);
        test(&recipe, &available, 0);
    }
}


#[cfg(test)]
mod equal_sides_array_tests {
    fn find_even_index(arr: &[i32]) -> Option<usize> {
        let total_sum: i32 = arr.iter().sum();
        let mut left_sum = 0;

        for (i, &num) in arr.iter().enumerate() {
            if left_sum == total_sum - (left_sum + num) {
                return Some(i);
            }
            left_sum += num;
        }

        None
    }

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected: Option<usize>) {
        assert_eq!(find_even_index(arr), expected, "{ERR_MSG} with arr = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 3, 2, 1], Some(3));
        dotest(&[1, 100, 50, -51, 1, 1], Some(1));
        dotest(&[1, 2, 3, 4, 5, 6], None);
        dotest(&[20, 10, 30, 10, 10, 15, 35], Some(3));
        dotest(&[20, 10, -80, 10, 10, 15, 35], Some(0));
        dotest(&[10, -80, 10, 10, 15, 35, 20], Some(6));
        dotest(&(1..100).collect::<Vec<_>>(), None);
        dotest(&[0, 0, 0, 0, 0], Some(0));
        dotest(&[-1, -2, -3, -4, -3, -2, -1], Some(3));
        dotest(&(-100..-1).collect::<Vec<_>>(), None);
        dotest(&[8, 8], None);
        dotest(&[8, 0], Some(0));
        dotest(&[0, 8], Some(1));
        dotest(&[7, 3, -3], Some(0));
        dotest(&[8], Some(0));
        dotest(&[10, -10], None);
        dotest(&[-3, 2, 1, 0], Some(3));
        dotest(&[-15, 5, 11, 17, 19, -17, 20, -6, 17, -17, 19, 16, -15, -6, 20, 17], Some(8));
    }
}


#[cfg(test)]
mod rectangle_into_squares_tests {

    fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
        if lng == wdth {
            return None;
        }

        let mut squares = Vec::new();
        let mut length = lng;
        let mut width = wdth;

        while length > 0 && width > 0 {
            if length > width {
                squares.push(width);
                length -= width;
            } else {
                squares.push(length);
                width -= length;
            }
        }

        Some(squares)
    }

    fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
        assert_eq!(sq_in_rect(lng, wdth), exp)
    }
    #[test]
    fn tests_sq_in_rect() {

        testing(5, 3, Some(vec![3, 2, 1, 1]));
        testing(3, 5, Some(vec![3, 2, 1, 1]));
        testing(5, 5, None);

    }
}



#[cfg(test)]
mod directions_reduction_tests {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Direction {
        North,
        East,
        West,
        South,
    }

    fn get_last_result(result: &mut Vec<Direction>, dir: Direction) { //todo it doesn't work after refactoring
        match (result.last(), dir) {
            (Some(&result_direction), dir) if result_direction == dir => {
                result.pop();
            },
            (_, dir) => {
                result.push(dir);
            },
        }

    }

    fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
        use Direction::*;

        let mut result = Vec::new();

        for &dir in arr {
            match dir {
                North => get_last_result(&mut result, South),
                South => get_last_result(&mut result, North),
                East => get_last_result(&mut result, West),
                West => get_last_result(&mut result, East),
            }
        }

        result
    }

    #[ignore]
    fn basic() {
        use Direction::*;

        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}


#[cfg(test)]
mod fizz_buzz_tests {

    pub trait Fizzy {
        fn fizzy(&self);
    }

    impl Fizzy for i32 {
        fn fizzy(&self) {
            for i in 1..self + 1 {
                println!("{}", convert_to_fizz_buzz(i))
            }
        }
    }

    fn convert_to_fizz_buzz(n: i32) -> String {
        if n % (3 * 5) == 0 {
            return String::from("FizzBuzz")
        } else if n % 3 == 0 {
            return String::from("Fizz")
        } else if n % 5 == 0 {
            return String::from("Buzz")
        } else {
            return n.to_string()
        }
    }

    #[test]
    fn main() {
        for x in 1..=10 {
            x.fizzy()
        }
    }
}

#[cfg(test)]
mod duplicate_counter_tests {
    use std::collections::HashMap;

    fn count_duplicates(text: &str) -> u32 {
        let mut char_grouped = HashMap::new();

        for c in text.to_lowercase().chars() {
            if c.is_alphanumeric() {
                *char_grouped.entry(c).or_insert(0) += 1;
            }
        }

        char_grouped.values().filter(|count| **count > 1).count() as u32
    }

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}

#[cfg(test)]
mod break_camelcase_tests {
    fn solution(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!(" {}", c)
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}

#[cfg(test)]
mod find_outlier_tests {

    fn find_outlier(values: &[i32]) -> i32 {
        let (even, odd): (Vec<&i32>, Vec<&i32>) = values.iter().partition(|&x| x % 2 == 0);

        if even.len() == 1 {
            *even[0]
        } else {
            *odd[0]
        }
    }

    #[test]
    fn basic_test() {
        let t1 = [2,6,8,-10,3];
        let t2 = [206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}

#[cfg(test)]
mod like_description_text_tests {
    fn likes(names: &[&str]) -> String {
        match names.len() {
            0 => "no one likes this".to_string(),
            1 => format!("{} likes this", names[0]),
            2 => format!("{} and {} like this", names[0], names[1]),
            3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
            _ => format!("{}, {} and {} others like this", names[0], names[1], names.len() - 2),
        }
    }

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}

#[cfg(test)]
mod disem_vowel_tests {

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




fn main() {
}

#[cfg(test)]
mod narcissistic_tests {

    fn narcissistic(num: u64) -> bool {
        (num.to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .map(|n| (n as u64).pow(num.to_string().len() as u32))
            .sum::<u64>()) == num
    }

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

    fn solution(word: &str, ending: &str) -> bool {
        word.ends_with(ending)
    }

    #[test]
    fn test() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
        assert_eq!(true, solution("hello", "lo"));
    }
}