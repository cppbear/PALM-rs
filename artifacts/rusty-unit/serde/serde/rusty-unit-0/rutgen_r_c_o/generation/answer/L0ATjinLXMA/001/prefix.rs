// Answer 0

#[test]
fn test_visit_i8_neg_boundary() {
    let visitor = ContentVisitor { value: PhantomData };
    let _: Result<Content, _> = visitor.visit_i8(-128);
}

#[test]
fn test_visit_i8_neg_middle() {
    let visitor = ContentVisitor { value: PhantomData };
    let _: Result<Content, _> = visitor.visit_i8(-1);
}

#[test]
fn test_visit_i8_zero() {
    let visitor = ContentVisitor { value: PhantomData };
    let _: Result<Content, _> = visitor.visit_i8(0);
}

#[test]
fn test_visit_i8_pos_middle() {
    let visitor = ContentVisitor { value: PhantomData };
    let _: Result<Content, _> = visitor.visit_i8(1);
}

#[test]
fn test_visit_i8_pos_boundary() {
    let visitor = ContentVisitor { value: PhantomData };
    let _: Result<Content, _> = visitor.visit_i8(127);
}

