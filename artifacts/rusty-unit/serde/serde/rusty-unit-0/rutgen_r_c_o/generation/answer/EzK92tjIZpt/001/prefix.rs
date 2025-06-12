// Answer 0

#[test]
fn test_visit_string_equal_empty() {
    let visitor = TagOrContentVisitor {
        name: "",
        value: PhantomData,
    };
    let result = visitor.visit_string(String::from(""));
}

#[test]
fn test_visit_string_equal_single_char() {
    let visitor = TagOrContentVisitor {
        name: "a",
        value: PhantomData,
    };
    let result = visitor.visit_string(String::from("a"));
}

#[test]
fn test_visit_string_equal_unicode() {
    let visitor = TagOrContentVisitor {
        name: "é",
        value: PhantomData,
    };
    let result = visitor.visit_string(String::from("é"));
}

#[test]
fn test_visit_string_equal_long() {
    let long_string = "a".repeat(100);
    let visitor = TagOrContentVisitor {
        name: &long_string,
        value: PhantomData,
    };
    let result = visitor.visit_string(long_string);
}

#[test]
fn test_visit_string_equal_with_special_chars() {
    let visitor = TagOrContentVisitor {
        name: "!@#$%^&*()",
        value: PhantomData,
    };
    let result = visitor.visit_string(String::from("!@#$%^&*()"));
}

