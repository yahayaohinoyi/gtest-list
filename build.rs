use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let gtest_dir = out_dir.join("googletest");

    // Clone googletest if not already present
    if !gtest_dir.join("googletest/include").exists() {
        let status = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "https://github.com/google/googletest.git",
                gtest_dir.to_str().unwrap(),
            ])
            .status()
            .expect("failed to run git clone");
        assert!(status.success(), "git clone googletest failed");
    }

    let gtest_src = gtest_dir.join("googletest");
    let gtest_include = gtest_src.join("include");
    let gtest_src_dir = gtest_src.join("src");

    // Compile gtest + test fixture into a single binary
    let fixture_src = manifest_dir.join("tests/fixture/sample_test.cpp");
    let binary_path = out_dir.join("sample_test_binary");

    let compiler = env::var("CXX").unwrap_or_else(|_| "c++".to_string());
    let status = Command::new(&compiler)
        .args([
            "-std=c++17",
            "-isystem",
            gtest_include.to_str().unwrap(),
            "-I",
            gtest_src.to_str().unwrap(),
            "-pthread",
            "-o",
            binary_path.to_str().unwrap(),
        ])
        .arg(gtest_src_dir.join("gtest-all.cc"))
        .arg(gtest_src_dir.join("gtest_main.cc"))
        .arg(&fixture_src)
        .status()
        .unwrap_or_else(|e| panic!("failed to run {compiler}: {e}"));

    assert!(status.success(), "failed to compile gtest test binary");

    // Tell cargo where to find the binary
    println!(
        "cargo::rustc-env=GTEST_SAMPLE_BINARY={}",
        binary_path.display()
    );

    // Rerun if the fixture changes
    println!("cargo::rerun-if-changed=tests/fixture/sample_test.cpp");
    println!("cargo::rerun-if-changed=build.rs");
}
