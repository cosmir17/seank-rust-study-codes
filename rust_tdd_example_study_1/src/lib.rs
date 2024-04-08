// Rust Test and mocking
// https://www.youtube.com/watch?v=8XaVlL3lObQ
// typed myself

pub fn compute_answer_to_be_tested_from_tests_package(question: i32) -> i32 {
    42 // pretend that this is a long-running computation
}

fn compute_answer_internal_logic_test_example(question: i32) -> i32 {
    42 // pretend that this is a long-running computation
}

fn should_panic(question: i32) -> i32 {
    panic!("Oh, No!!!");
}

use mockall::predicate::*;
use mockall::*;

#[automock] //this generates a mock for us. It creates an implementation of that trait, prefix
pub trait BigComputer { //with term 'Mock'
    fn compute_answer(&self, question: i32) -> i32;
}

// impl BigComputer { // this was when BigComputer was a struct
//     fn compute_answer(&self, question: i32) -> i32 {
//         42
//     }
// }

pub fn get_answer(computer: &impl BigComputer, question: i32) -> String {
    format!("The answer is {}", computer.compute_answer(question))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_should_be_42() {
        assert_eq!(compute_answer_internal_logic_test_example(1), 42);
        assert_ne!(compute_answer_internal_logic_test_example(1), 40);
        assert!(compute_answer_internal_logic_test_example(2) > 1);
    }

    #[test]
    #[should_panic]
    fn should_cause_exception() {
        assert_eq!(should_panic(0), 42);
    }

    #[test]
    #[ignore]
    fn should_not_cause_exception() {
        assert_eq!(should_panic(0), 42);
    }

    #[test]
    fn answer_should_be_42_with_mocking() {
        let mut mock_computer = MockBigComputer::new();
        mock_computer.expect_compute_answer()
            .times(1)
            .return_const(42);

        //times(2) : the following error is expected, I only call once
        // MockBigComputer::compute_answer: Expectation(<anything>) called 1 time(s) which is fewer than expected 2
        // thread 'tests::answer_should_be_42_with_mocking' panicked at rust_tdd_example_study_1/src/lib.rs:18:1:
        // MockBigComputer::compute_answer: Expectation(<anything>) called 1 time(s) which is fewer than expected 2
        // stack backtrace:
        //    0: rust_begin_unwind
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:647:5
        //    1: core::panicking::panic_fmt
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:72:14
        //    2: <rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Common as core::ops::drop::Drop>::drop
        //              at ./src/lib.rs:18:1
        //    3: core::ptr::drop_in_place<rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Common>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //    4: core::ptr::drop_in_place<rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Expectation>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //    5: core::ptr::drop_in_place<[rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Expectation]>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //    6: <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/alloc/src/vec/mod.rs:3147:13
        //    7: core::ptr::drop_in_place<alloc::vec::Vec<rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Expectation>>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //    8: core::ptr::drop_in_place<rust_tdd_example_study_1::__mock_MockBigComputer_BigComputer::__compute_answer::Expectations>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //    9: core::ptr::drop_in_place<rust_tdd_example_study_1::MockBigComputer_BigComputer>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //   10: core::ptr::drop_in_place<rust_tdd_example_study_1::MockBigComputer>
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ptr/mod.rs:507:1
        //   11: rust_tdd_example_study_1::tests::answer_should_be_42_with_mocking
        //              at ./src/lib.rs:69:5
        //   12: rust_tdd_example_study_1::tests::answer_should_be_42_with_mocking::{{closure}}
        //              at ./src/lib.rs:57:42
        //   13: core::ops::function::FnOnce::call_once
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
        //   14: core::ops::function::FnOnce::call_once
        //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
        // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

        let answer = get_answer(
            &mock_computer,
            1,
        );

        assert_eq!(answer, "The answer is 42");
    }
}

// cargo test
// cargo test [SEARCH_KEYWORD]
// cargo test -- --..........                                       =>  passed to test binary
// cargo test -- --test-threads=1                                   =>  running sequentially
// cargo test -- --test-threads=5                                   =>  running in parallel
// cargo test -- --ignored                                          =>  run only ignored tests

// cargo add mockall