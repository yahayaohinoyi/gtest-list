use std::path::Path;

use gtest_list::list_tests;

fn sample_binary() -> &'static Path {
    Path::new(env!("GTEST_SAMPLE_BINARY"))
}

#[test]
fn list_tests_from_binary() {
    let tests = list_tests(sample_binary()).unwrap();
    assert_eq!(tests.len(), 5);
}

#[test]
fn correct_test_names() {
    let tests = list_tests(sample_binary()).unwrap();
    let names: Vec<String> = tests.iter().map(|t| t.to_string()).collect();
    assert_eq!(
        names,
        vec![
            "MathTest.Addition",
            "MathTest.Subtraction",
            "StringTest.Length",
            "StringTest.Empty",
            "EdgeCaseTest.SingleTest",
        ]
    );
}

#[test]
fn correct_suite_grouping() {
    let tests = list_tests(sample_binary()).unwrap();
    let math: Vec<_> = tests.iter().filter(|t| t.suite == "MathTest").collect();
    let string: Vec<_> = tests.iter().filter(|t| t.suite == "StringTest").collect();
    let edge: Vec<_> = tests.iter().filter(|t| t.suite == "EdgeCaseTest").collect();
    assert_eq!(math.len(), 2);
    assert_eq!(string.len(), 2);
    assert_eq!(edge.len(), 1);
}
