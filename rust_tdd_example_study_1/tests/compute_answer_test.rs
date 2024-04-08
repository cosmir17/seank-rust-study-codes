use rust_tdd_example_study_1::compute_answer_to_be_tested_from_tests_package;

#[test]
fn answer_should_be_42() {
    // assert_eq!(compute_answer(1), 41); this should fail because we are not expecting 41

    // adding the error message for a future reference
    //
    //    Finished test [unoptimized + debuginfo] target(s) in 0.07s
    //      Running tests/compute_answer_test.rs (target/debug/deps/compute_answer_test-c54ad63a659084ac)
    //
    // assertion `left == right` failed
    //   left: 42
    //  right: 41
    //
    // Left:  42
    // Right: 41
    // <Click to see difference>
    //
    // thread 'answer_should_be_42' panicked at rust_tdd_example_study_1/tests/compute_answer_test.rs:6:5:
    // assertion `left == right` failed
    //   left: 42
    //  right: 41
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:647:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:72:14
    //    2: core::panicking::assert_failed_inner
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:342:17
    //    3: core::panicking::assert_failed
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:297:5
    //    4: compute_answer_test::answer_should_be_42
    //              at ./tests/compute_answer_test.rs:6:5
    //    5: compute_answer_test::answer_should_be_42::{{closure}}
    //              at ./tests/compute_answer_test.rs:4:25
    //    6: core::ops::function::FnOnce::call_once
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    //
    // error: test failed, to rerun pass `--test compute_answer_test`
    // error: 1 target failed:
    //     `--test compute_answer_test`
    //
    // Process finished with exit code 101

    assert_eq!(compute_answer_to_be_tested_from_tests_package(1), 42);

}