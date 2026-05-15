# gtest-list

A Rust CLI tool that lists tests from a GoogleTest binary by parsing the output of `--gtest_list_tests`.

## Usage

```bash
gtest-list <path-to-gtest-binary>
```

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

The build script automatically clones GoogleTest and compiles a sample test binary for integration tests.
