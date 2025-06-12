// Answer 0

#[test]
fn test_visit_f64() {
    let visitor = ContentVisitor { value: PhantomData };

    // Testing with positive finite f64 value
    let result = visitor.visit_f64(1.23);
    assert_eq!(result, Ok(Content::F64(1.23)));

    // Testing with negative finite f64 value
    let result = visitor.visit_f64(-4.56);
    assert_eq!(result, Ok(Content::F64(-4.56)));

    // Testing with zero
    let result = visitor.visit_f64(0.0);
    assert_eq!(result, Ok(Content::F64(0.0)));

    // Testing with positive infinity
    let result = visitor.visit_f64(f64::INFINITY);
    assert_eq!(result, Ok(Content::F64(f64::INFINITY)));

    // Testing with negative infinity
    let result = visitor.visit_f64(f64::NEG_INFINITY);
    assert_eq!(result, Ok(Content::F64(f64::NEG_INFINITY)));

    // Testing with NaN (Not-a-Number)
    let result = visitor.visit_f64(f64::NAN);
    assert_eq!(result, Ok(Content::F64(f64::NAN)));
}

