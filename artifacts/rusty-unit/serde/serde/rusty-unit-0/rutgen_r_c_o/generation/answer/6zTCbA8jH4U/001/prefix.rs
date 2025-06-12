// Answer 0

#[test]
fn test_visit_char_valid_range_1() {
    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let _ = visitor.visit_char('\u{0000}');
}

#[test]
fn test_visit_char_valid_range_2() {
    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let _ = visitor.visit_char('\u{D7FF}');
}

#[test]
fn test_visit_char_valid_range_3() {
    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let _ = visitor.visit_char('\u{E000}');
}

#[test]
fn test_visit_char_valid_range_4() {
    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let _ = visitor.visit_char('\u{FFFF}');
}

