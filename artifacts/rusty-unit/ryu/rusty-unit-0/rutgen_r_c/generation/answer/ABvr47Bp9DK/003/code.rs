// Answer 0

#[test]
#[should_panic]
fn test_log10_pow5_negative_input() {
    let result = log10_pow5(-1);
}

#[test]
fn test_log10_pow5_zero_input() {
    let result = log10_pow5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow5_boundary_input_2620() {
    let result = log10_pow5(2620);
    assert_eq!(result, 732923 * 2620 >> 20);
}

#[test]
fn test_log10_pow5_boundary_input_2619() {
    let result = log10_pow5(2619);
    assert_eq!(result, 732923 * 2619 >> 20);
}

#[test]
fn test_log10_pow5_middle_input() {
    let result = log10_pow5(1310);
    assert_eq!(result, 732923 * 1310 >> 20);
}

