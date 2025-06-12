// Answer 0

#[test]
fn test_expand_with_valid_index() {
    struct CaptureGroups {
        groups: Vec<String>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<String>) -> Self {
            CaptureGroups { groups }
        }
    }

    let captures = CaptureGroups::new(vec![
        "whole_match".to_string(),
        "first_group".to_string(),
    ]);
    let mut dst = String::new();
    captures.expand("$0 and $1", &mut dst);
    assert_eq!(dst, "whole_match and first_group");
}

#[test]
fn test_expand_with_named_capture_group() {
    struct CaptureGroups {
        groups: Vec<(String, String)>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<(String, String)>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroups {
        fn expand(&self, replacement: &str, dst: &mut String) {
            // Minimal implementation for mock-up
            let mut result = replacement.to_string();
            for (name, value) in &self.groups {
                result = result.replace(&format!("${}", name), value);
            }
            dst.push_str(&result);
        }
    }

    let captures = CaptureGroups::new(vec![
        ("name_group".to_string(), "named_value".to_string()),
    ]);
    let mut dst = String::new();
    captures.expand("$name_group", &mut dst);
    assert_eq!(dst, "named_value");
}

#[test]
fn test_expand_with_invalid_index() {
    struct CaptureGroups {
        groups: Vec<String>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<String>) -> Self {
            CaptureGroups { groups }
        }
    }

    let captures = CaptureGroups::new(vec![
        "match".to_string(),
    ]);
    let mut dst = String::new();
    captures.expand("$2", &mut dst);
    assert_eq!(dst, "");
}

#[test]
fn test_expand_with_invalid_named_capture() {
    struct CaptureGroups {
        groups: Vec<(String, String)>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<(String, String)>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroups {
        fn expand(&self, replacement: &str, dst: &mut String) {
            let mut result = replacement.to_string();
            for (name, value) in &self.groups {
                result = result.replace(&format!("${}", name), value);
            }
            dst.push_str(&result);
        }
    }

    let captures = CaptureGroups::new(vec![]);
    let mut dst = String::new();
    captures.expand("$non_existent_name", &mut dst);
    assert_eq!(dst, "");
}

#[test]
fn test_expand_with_literal_dollar() {
    struct CaptureGroups {
        groups: Vec<String>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<String>) -> Self {
            CaptureGroups { groups }
        }
    }

    let captures = CaptureGroups::new(vec![
        "match".to_string(),
    ]);
    let mut dst = String::new();
    captures.expand("$$ is a literal dollar", &mut dst);
    assert_eq!(dst, "$ is a literal dollar");
}

