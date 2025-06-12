// Answer 0

#[test]
fn test_as_f32_with_float() {
    // Construct a Number with a float value
    let num = Number {
        n: N::Float(3.14),
    };

    // Expect that as_f32 returns Some(3.14 as f32)
    assert_eq!(num.as_f32(), Some(3.14f32));
}

#[test]
fn test_as_f32_with_positive_int() {
    // Construct a Number with a positive integer value
    let num = Number {
        n: N::PosInt(10),
    };

    // Expect that as_f32 returns Some(10 as f32)
    assert_eq!(num.as_f32(), Some(10.0f32));
}

#[test]
fn test_as_f32_with_negative_int() {
    // Construct a Number with a negative integer value
    let num = Number {
        n: N::NegInt(-10),
    };

    // Expect that as_f32 returns Some(-10 as f32)
    assert_eq!(num.as_f32(), Some(-10.0f32));
}

