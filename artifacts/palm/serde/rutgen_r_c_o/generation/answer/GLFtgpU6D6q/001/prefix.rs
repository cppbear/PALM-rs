// Answer 0

#[test]
fn test_visit_u32_min() {
    let visitor = ContentVisitor { value: PhantomData };
    let _ = visitor.visit_u32(0u32);
}

#[test]
fn test_visit_u32_mid() {
    let visitor = ContentVisitor { value: PhantomData };
    let _ = visitor.visit_u32(2147483648u32);
}

#[test]
fn test_visit_u32_max() {
    let visitor = ContentVisitor { value: PhantomData };
    let _ = visitor.visit_u32(4294967295u32);
}

