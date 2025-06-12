// Answer 0

#[test]
fn test_expand_str_with_no_variable() {
    use re_unicode::{Captures, Regex};
    use std::mem;

    struct MyCaptures {
        values: Vec<Option<String>>,
    }

    impl MyCaptures {
        fn get(&self, index: usize) -> Option<&String> {
            self.values.get(index).and_then(|v| v.as_ref())
        }

        fn name(&self, _name: &str) -> Option<&String> {
            None
        }
    }

    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let caps = MyCaptures {
        values: vec![Some("value".to_string())],
    };

    let mut result = String::new();
    expand_str(
        &caps,
        "No match here",
        &mut result,
    );
    assert_eq!(result, "No match here");
}

#[test]
fn test_expand_str_with_escaped_dollar() {
    use re_unicode::{Captures, Regex};
    use std::mem;

    struct MyCaptures {
        values: Vec<Option<String>>,
    }

    impl MyCaptures {
        fn get(&self, index: usize) -> Option<&String> {
            self.values.get(index).and_then(|v| v.as_ref())
        }

        fn name(&self, _name: &str) -> Option<&String> {
            None
        }
    }

    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let caps = MyCaptures {
        values: vec![Some("value".to_string())],
    };

    let mut result = String::new();
    expand_str(
        &caps,
        "Value is $$ and it should be here.",
        &mut result,
    );
    assert_eq!(result, "Value is $ and it should be here.");
}

#[test]
fn test_expand_str_with_missing_capture() {
    use re_unicode::{Captures, Regex};
    use std::mem;

    struct MyCaptures {
        values: Vec<Option<String>>,
    }

    impl MyCaptures {
        fn get(&self, index: usize) -> Option<&String> {
            self.values.get(index).and_then(|v| v.as_ref())
        }

        fn name(&self, _name: &str) -> Option<&String> {
            None
        }
    }

    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let caps = MyCaptures {
        values: vec![None],
    };

    let mut result = String::new();
    expand_str(
        &caps,
        "Capture is $1 and it should be here.",
        &mut result,
    );
    assert_eq!(result, "Capture is  and it should be here.");
}

