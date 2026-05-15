use std::path::PathBuf;

use gtest_list::list_tests;

fn main() {
    let binary = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: gtest-list <test-binary>");
            std::process::exit(1);
        });

    let tests = list_tests(&binary).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    println!("found {} test(s):", tests.len());
    for test in &tests {
        println!("  {test}");
    }
}
