// Answer 0

#[test]
fn test_from_str_valid_floats() {
    let inputs = [
        "0",
        "1",
        "-1",
        "NaN",
        "Infinity",
        "-Infinity",
        "1e-300",
        "1e+300",
        "2.5",
        "3.14",
        "4.0e10",
        "9.999999999999999e300",
    ];

    for &input in &inputs {
        let _ = Number::from_str(input);
    }
}

#[test]
fn test_from_str_valid_i64() {
    let inputs = [
        "-9223372036854775808",
        "-1",
        "0",
        "1",
        "9223372036854775807",
    ];

    for &input in &inputs {
        let _ = Number::from_str(input);
    }
}

#[test]
fn test_from_str_valid_u64() {
    let inputs = [
        "0",
        "1",
        "18446744073709551615",
    ];

    for &input in &inputs {
        let _ = Number::from_str(input);
    }
}

#[should_panic]
#[test]
fn test_from_str_invalid_float() {
    let inputs = [
        "invalid",
        "1.0.0",
        "1e+",
        "1e-",
        "Infinityx",
    ];

    for &input in &inputs {
        let _ = Number::from_str(input);
    }
}

