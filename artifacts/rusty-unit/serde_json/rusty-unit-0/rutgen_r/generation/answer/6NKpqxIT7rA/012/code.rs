// Answer 0

fn f64_from_parts(
    positive: bool,
    significand: u64,
    exponent: i32,
) -> Result<f64> {
    let mut f = significand as f64;
    loop {
        match POW10.get(exponent.wrapping_abs() as usize) {
            Some(&pow) => {
                if exponent >= 0 {
                    f *= pow;
                    if f.is_infinite() {
                        return Err(self.error(ErrorCode::NumberOutOfRange));
                    }
                } else {
                    f /= pow;
                }
                break;
            }
            None => {
                if f == 0.0 {
                    break;
                }
                if exponent >= 0 {
                    return Err(self.error(ErrorCode::NumberOutOfRange));
                }
                f /= 1e308;
                exponent += 308;
            }
        }
    }
    Ok(if positive { f } else { -f })
}

#[test]
fn test_f64_from_parts_with_positive_exponent() {
    let mut dummy = DummyStruct {}; // Dummy struct initialization
    let result = dummy.f64_from_parts(true, 123456789, 5); // Valid inputs
    assert!(result.is_ok());
    assert!(result.unwrap() > 0.0);
}

#[test]
fn test_f64_from_parts_zero_significand() {
    let mut dummy = DummyStruct {};
    let result = dummy.f64_from_parts(true, 0, -1); // Edge case with zero significand
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_f64_from_parts_large_negative_exponent() {
    let mut dummy = DummyStruct {};
    let result = dummy.f64_from_parts(true, 123456789, -310); // Large negative exponent
    assert!(result.is_ok());
    assert!(result.unwrap().is_finite());
}

#[test]
fn test_f64_from_parts_overflow() {
    let mut dummy = DummyStruct {};
    let result = dummy.f64_from_parts(true, 1, 308);
    assert!(result.is_err());
}

#[test]
fn test_f64_from_parts_with_negative_significand() {
    let mut dummy = DummyStruct {};
    let result = dummy.f64_from_parts(false, 123456789, 5); // Validate negative output
    assert!(result.is_ok());
    assert!(result.unwrap() < 0.0);
}

struct DummyStruct;

impl DummyStruct {
    fn error(&self, _: ErrorCode) -> () {
        // Dummy error handling
    }
}

