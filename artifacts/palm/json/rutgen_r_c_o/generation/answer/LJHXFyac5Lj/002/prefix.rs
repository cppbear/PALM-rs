// Answer 0

#[test]
fn test_as_i64_neg_int_with_min_value() {
    let num = Number { n: N::NegInt(-128) };
    let _result = num.as_i64();
}

#[test]
fn test_as_i64_neg_int_with_middle_value() {
    let num = Number { n: N::NegInt(-64) };
    let _result = num.as_i64();
}

#[test]
fn test_as_i64_neg_int_with_max_value() {
    let num = Number { n: N::NegInt(-1) };
    let _result = num.as_i64();
}

