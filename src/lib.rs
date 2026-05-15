use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct FullTestName {
    pub suite: String,
    pub name: String,
}

impl std::fmt::Display for FullTestName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.suite, self.name)
    }
}

pub fn list_tests(binary: &Path) -> Result<Vec<FullTestName>, String> {
    let output = Command::new(binary)
        .arg("--gtest_list_tests")
        .output()
        .map_err(|e| format!("failed to run {}: {e}", binary.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "{} exited with {}: {stderr}",
            binary.display(),
            output.status
        ));
    }

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| format!("invalid UTF-8 in output: {e}"))?;

    parse_list_output(&stdout)
}

pub fn list_tests_json(_binary: &Path) -> Result<String, String> {
    Err("JSON output is not yet implemented".to_string())
}

pub fn list_tests_static(_binary: &Path) -> Result<Vec<FullTestName>, String> {
    Err("static listing is not yet implemented".to_string())
}

#[derive(Debug)]
pub struct TestInfo {
    pub suite: String,
    pub name: String,
    pub file: String,
    pub line: u64,
}

impl std::fmt::Display for TestInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{} ({}:{})", self.suite, self.name, self.file, self.line)
    }
}

pub fn list_tests_with_location(_binary: &Path) -> Result<Vec<TestInfo>, String> {
    Err("test location extraction is not yet implemented".to_string())
}

pub fn parse_list_output(output: &str) -> Result<Vec<FullTestName>, String> {
    let mut tests = Vec::new();
    let mut current_suite: Option<&str> = None;

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with(' ') || line.starts_with('\t') {
            let name = line.trim();
            let suite = current_suite
                .ok_or_else(|| format!("test '{name}' appears before any suite header"))?;
            tests.push(FullTestName {
                suite: suite.to_string(),
                name: name.to_string(),
            });
        } else if let Some(suite) = line.strip_suffix('.') {
            current_suite = Some(suite);
        }
    }

    Ok(tests)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_OUTPUT: &str = "\
MathTest.
  Addition
  Subtraction
StringTest.
  Length
  Concat
";

    #[test]
    fn parse_valid_output() {
        let tests = parse_list_output(SAMPLE_OUTPUT).unwrap();
        assert_eq!(tests.len(), 4);
    }

    #[test]
    fn correct_names() {
        let tests = parse_list_output(SAMPLE_OUTPUT).unwrap();
        let names: Vec<String> = tests.iter().map(|t| t.to_string()).collect();
        assert_eq!(
            names,
            vec![
                "MathTest.Addition",
                "MathTest.Subtraction",
                "StringTest.Length",
                "StringTest.Concat",
            ]
        );
    }

    #[test]
    fn single_suite() {
        let output = "OnlySuite.\n  Test1\n  Test2\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].to_string(), "OnlySuite.Test1");
        assert_eq!(tests[1].to_string(), "OnlySuite.Test2");
    }

    #[test]
    fn single_test() {
        let output = "Suite.\n  OnlyTest\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].to_string(), "Suite.OnlyTest");
    }

    #[test]
    fn empty_output() {
        let tests = parse_list_output("").unwrap();
        assert!(tests.is_empty());
    }

    #[test]
    fn skips_blank_lines() {
        let output = "Suite.\n\n  Test1\n\n  Test2\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests.len(), 2);
    }

    #[test]
    fn handles_tab_indentation() {
        let output = "Suite.\n\tTest1\n\tTest2\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].name, "Test1");
    }

    #[test]
    fn test_before_suite_fails() {
        let output = "  OrphanTest\nSuite.\n  Test1\n";
        let result = parse_list_output(output);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("before any suite header"));
    }

    #[test]
    fn ignores_non_suite_header_lines() {
        let output = "Running main() from gtest_main.cc\nSuite.\n  Test1\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].to_string(), "Suite.Test1");
    }

    #[test]
    fn disabled_test_names_preserved() {
        let output = "Suite.\n  DISABLED_Skipped\n  Normal\n";
        let tests = parse_list_output(output).unwrap();
        assert_eq!(tests[0].name, "DISABLED_Skipped");
        assert_eq!(tests[1].name, "Normal");
    }

    #[test]
    fn nonexistent_binary_fails() {
        let result = list_tests(Path::new("/nonexistent/binary"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("failed to run"));
    }

    #[test]
    fn display_format() {
        let name = FullTestName {
            suite: "Foo".to_string(),
            name: "Bar".to_string(),
        };
        assert_eq!(format!("{name}"), "Foo.Bar");
    }
}
