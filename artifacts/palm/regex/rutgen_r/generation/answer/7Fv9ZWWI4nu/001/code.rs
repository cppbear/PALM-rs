// Answer 0

#[derive(Debug)]
struct Substring<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

impl<'t> Substring<'t> {
    pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }
}

#[test]
fn test_as_str_valid_range() {
    let substring = Substring {
        text: "Hello, world!",
        start: 0,
        end: 5,
    };
    assert_eq!(substring.as_str(), "Hello");
}

#[test]
fn test_as_str_full_range() {
    let substring = Substring {
        text: "Rust programming",
        start: 0,
        end: 16,
    };
    assert_eq!(substring.as_str(), "Rust programming");
}

#[test]
fn test_as_str_empty_range() {
    let substring = Substring {
        text: "Sample text",
        start: 5,
        end: 5,
    };
    assert_eq!(substring.as_str(), "");
}

#[should_panic]
#[test]
fn test_as_str_start_out_of_bounds() {
    let substring = Substring {
        text: "Out of bounds",
        start: 15,
        end: 16,
    };
    substring.as_str();
}

#[should_panic]
#[test]
fn test_as_str_end_out_of_bounds() {
    let substring = Substring {
        text: "Another sample",
        start: 8,
        end: 20,
    };
    substring.as_str();
}

#[should_panic]
#[test]
fn test_as_str_start_greater_than_end() {
    let substring = Substring {
        text: "Invalid range",
        start: 5,
        end: 3,
    };
    substring.as_str();
}

