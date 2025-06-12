// Answer 0

#[test]
fn test_as_f64_negative_int_negint() {
    let number_neg1 = Number::from(-1i64);
    let result_neg1 = number_neg1.as_f64();

    let number_neg100 = Number::from(-100i64);
    let result_neg100 = number_neg100.as_f64();

    let number_neg2147483648 = Number::from(-2147483648i64);
    let result_neg2147483648 = number_neg2147483648.as_f64();
}

#[test]
fn test_as_f64_negative_float() {
    let number_neg1_0 = Number::from_f64(-1.0).unwrap();
    let result_neg1_0 = number_neg1_0.as_f64();

    let number_neg100_5 = Number::from_f64(-100.5).unwrap();
    let result_neg100_5 = number_neg100_5.as_f64();

    let number_neg3_14 = Number::from_f64(-3.14).unwrap();
    let result_neg3_14 = number_neg3_14.as_f64();
}

