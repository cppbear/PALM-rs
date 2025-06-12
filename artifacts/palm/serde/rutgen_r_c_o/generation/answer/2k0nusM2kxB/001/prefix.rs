// Answer 0

#[test]
fn test_visit_i8_min() {
    let visitor = TagOrContentVisitor { name: "tag", value: PhantomData };
    let input = -128i8;
    let _ = visitor.visit_i8(input);
}

#[test]
fn test_visit_i8_zero() {
    let visitor = TagOrContentVisitor { name: "tag", value: PhantomData };
    let input = 0i8;
    let _ = visitor.visit_i8(input);
}

#[test]
fn test_visit_i8_max() {
    let visitor = TagOrContentVisitor { name: "tag", value: PhantomData };
    let input = 127i8;
    let _ = visitor.visit_i8(input);
}

#[test]
fn test_visit_i8_positive() {
    let visitor = TagOrContentVisitor { name: "tag", value: PhantomData };
    let input = 64i8;
    let _ = visitor.visit_i8(input);
}

#[test]
fn test_visit_i8_negative() {
    let visitor = TagOrContentVisitor { name: "tag", value: PhantomData };
    let input = -64i8;
    let _ = visitor.visit_i8(input);
}

