//! B4n1Web Test Runner - Playwright-style test framework
//!
//! Provides test(), expect(), describe() for browser-based testing.

use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Test result
#[derive(Debug, Clone)]
pub enum TestResult {
    Pass,
    Fail(String),
    Skip,
}

/// A single test case
pub struct TestCase {
    pub name: String,
    pub run: Box<dyn Fn() -> TestResult + Send>,
}

/// Test suite
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<TestCase>,
    pub before: Vec<Box<dyn Fn() + Send>>,
    pub after: Vec<Box<dyn Fn() + Send>>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), tests: vec![], before: vec![], after: vec![] }
    }

    pub fn test<F: Fn() -> TestResult + 'static + Send>(&mut self, name: &str, f: F) {
        self.tests.push(TestCase { name: name.to_string(), run: Box::new(f) });
    }

    pub fn before_each<F: Fn() + 'static + Send>(&mut self, f: F) {
        self.before.push(Box::new(f));
    }

    pub fn after_each<F: Fn() + 'static + Send>(&mut self, f: F) {
        self.after.push(Box::new(f));
    }

    pub fn run(&self) -> TestReport {
        let mut report = TestReport::new(&self.name);
        for test in &self.tests {
            for b in &self.before { b(); }
            let result = (test.run)();
            for a in &self.after { a(); }
            report.add(test.name.clone(), result);
        }
        report
    }
}

/// Test report with statistics
pub struct TestReport {
    pub suite: String,
    pub results: Vec<(String, TestResult)>,
    pub passed: usize,
    pub failed: usize,
}

impl TestReport {
    pub fn new(suite: &str) -> Self {
        Self { suite: suite.to_string(), results: vec![], passed: 0, failed: 0 }
    }

    pub fn add(&mut self, name: String, result: TestResult) {
        match result {
            TestResult::Pass => self.passed += 1,
            TestResult::Fail(_) => self.failed += 1,
            TestResult::Skip => {}
        }
        self.results.push((name, result));
    }

    pub fn print(&self) {
        println!("\n=== Test Suite: {} ===", self.suite);
        for (name, result) in &self.results {
            match result {
                TestResult::Pass => println!("  ✅ {}", name),
                TestResult::Fail(msg) => println!("  ❌ {}: {}", name, msg),
                TestResult::Skip => println!("  ⏭️  {}", name),
            }
        }
        println!("\n  ✅ {} passed, ❌ {} failed", self.passed, self.failed);
    }

    pub fn to_json(&self) -> String {
        let mut entries = vec![];
        for (name, result) in &self.results {
            let status = match result {
                TestResult::Pass => "passed",
                TestResult::Fail(_) => "failed",
                TestResult::Skip => "skipped",
            };
            let msg = match result {
                TestResult::Fail(m) => format!(",\"message\":\"{}\"", m.replace('"', "\\\"")),
                _ => String::new(),
            };
            entries.push(format!("{{\"name\":\"{}\",\"status\":\"{}\"{}}}", name.replace('"', "\\\""), status, msg));
        }
        format!(
            r#"{{"suite":"{}","passed":{},"failed":{},"tests":[{}]}}"#,
            self.suite.replace('"', "\\\""),
            self.passed, self.failed,
            entries.join(",")
        )
    }

    pub fn to_html(&self) -> String {
        let mut rows = String::new();
        for (name, result) in &self.results {
            let (icon, color) = match result {
                TestResult::Pass => ("✅", "green"),
                TestResult::Fail(_) => ("❌", "red"),
                TestResult::Skip => ("⏭️", "gray"),
            };
            let msg = match result {
                TestResult::Fail(m) => format!(": {}", m),
                _ => String::new(),
            };
            rows.push_str(&format!(
                "<tr style='color:{}'><td>{}</td><td>{}{}</td></tr>", color, icon, name, msg
            ));
        }
        format!(
            r#"<!DOCTYPE html><html><head><title>Test Report</title></head><body>
            <h1>Test Suite: {}</h1>
            <p>✅ {} passed | ❌ {} failed</p>
            <table border='1' cellpadding='8'>{}</table></body></html>"#,
            self.suite, self.passed, self.failed, rows
        )
    }
}

/// Expect assertion utilities
pub struct Expect<T> {
    actual: T,
}

impl Expect<String> {
    pub fn new(actual: String) -> Self {
        Self { actual }
    }

    pub fn to_contain(self, expected: &str) -> TestResult {
        if self.actual.contains(expected) {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected '{}' to contain '{}'", self.actual, expected))
        }
    }

    pub fn to_equal(self, expected: &str) -> TestResult {
        if self.actual == expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected '{}', got '{}'", expected, self.actual))
        }
    }

    pub fn not(self) -> ExpectNot<String> {
        ExpectNot { expect: self }
    }
}

pub struct ExpectNot<T> {
    expect: Expect<T>,
}

impl ExpectNot<String> {
    pub fn to_contain(self, expected: &str) -> TestResult {
        if !self.expect.actual.contains(expected) {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected not to contain '{}'", expected))
        }
    }
}

impl Expect<bool> {
    pub fn new(actual: bool) -> Self {
        Self { actual }
    }

    pub fn to_be_true(self) -> TestResult {
        if self.actual {
            TestResult::Pass
        } else {
            TestResult::Fail("Expected true, got false".to_string())
        }
    }

    pub fn to_be_false(self) -> TestResult {
        if !self.actual {
            TestResult::Pass
        } else {
            TestResult::Fail("Expected false, got true".to_string())
        }
    }
}

impl Expect<usize> {
    pub fn new(actual: usize) -> Self {
        Self { actual }
    }

    pub fn to_be(self, expected: usize) -> TestResult {
        if self.actual == expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected {}, got {}", expected, self.actual))
        }
    }

    pub fn to_be_gt(self, expected: usize) -> TestResult {
        if self.actual > expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected {} > {}", self.actual, expected))
        }
    }

    pub fn to_be_lt(self, expected: usize) -> TestResult {
        if self.actual < expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected {} < {}", self.actual, expected))
        }
    }
}

/// Helper to create expect(value)
pub fn expect_str(value: String) -> Expect<String> { Expect::<String>::new(value) }
pub fn expect_bool(value: bool) -> Expect<bool> { Expect::<bool>::new(value) }
pub fn expect_usize(value: usize) -> Expect<usize> { Expect::<usize>::new(value) }

/// Create a test runner and return a builder
pub fn describe(name: &str) -> TestSuite {
    TestSuite::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_string_contains() {
        let result = expect_str("Hello World".into()).to_contain("World");
        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_expect_string_equal() {
        let result = expect_str("Hi".into()).to_equal("Hi");
        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_expect_bool_true() {
        let result = expect_bool(true).to_be_true();
        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_expect_usize() {
        let result = expect_usize(5).to_be(5);
        assert!(matches!(result, TestResult::Pass));
        let result = expect_usize(5).to_be_gt(3);
        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_test_suite() {
        let mut suite = describe("Test Suite");
        suite.test("passing test", || TestResult::Pass);
        suite.test("failing test", || TestResult::Fail("fail".into()));
        let report = suite.run();
        assert_eq!(report.passed, 1);
        assert_eq!(report.failed, 1);
    }

    #[test]
    fn test_report_json() {
        let mut report = TestReport::new("test");
        report.add("t1".into(), TestResult::Pass);
        let json = report.to_json();
        assert!(json.contains("passed"));
    }

    #[test]
    fn test_report_html() {
        let mut report = TestReport::new("test");
        report.add("t1".into(), TestResult::Pass);
        let html = report.to_html();
        assert!(html.contains("<table"));
    }

    #[test]
    fn test_expect_not() {
        let result = expect_str("Hello".into()).not().to_contain("World");
        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_expect_failures() {
        let result = expect_str("Hi".into()).to_equal("Bye");
        assert!(matches!(result, TestResult::Fail(_)));
    }
}
