// Answer 0

#[test]
fn test_visit_bytes_eq_name() {
    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let input: &[u8] = "test".as_bytes();
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_eq_name_empty() {
    let visitor = TagOrContentVisitor {
        name: "",
        value: PhantomData,
    };
    let input: &[u8] = "".as_bytes();
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_eq_name_single_byte() {
    let visitor = TagOrContentVisitor {
        name: "a",
        value: PhantomData,
    };
    let input: &[u8] = "a".as_bytes();
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_eq_name_special_char() {
    let visitor = TagOrContentVisitor {
        name: "@#^%",
        value: PhantomData,
    };
    let input: &[u8] = "@#^%".as_bytes();
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_eq_name_numeric() {
    let visitor = TagOrContentVisitor {
        name: "123",
        value: PhantomData,
    };
    let input: &[u8] = "123".as_bytes();
    visitor.visit_bytes(input);
}

