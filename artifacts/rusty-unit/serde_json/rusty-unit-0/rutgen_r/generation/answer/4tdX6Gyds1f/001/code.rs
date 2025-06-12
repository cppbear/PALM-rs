// Answer 0

#[derive(Debug)]
struct Number {
    n: N,
}

#[derive(Debug)]
enum N {
    Float(f64),
}

#[test]
fn test_from_f32_valid_finite() {
    let values = vec![
        1.0_f32,
        3.14_f32,
        -2.71_f32,
        0.0_f32,
        12345.6789_f32,
    ];

    for &value in &values {
        let result = from_f32(value);
        assert!(result.is_some(), "Expected Some(Number) for finite value {}", value);
        if let Some(Number { n }) = result {
            match n {
                N::Float(fl) => assert_eq!(fl, value as f64, "Float mismatch for value {}", value),
            }
        }
    }
}

