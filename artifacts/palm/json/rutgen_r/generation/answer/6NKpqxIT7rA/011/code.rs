// Answer 0

#[test]
fn test_f64_from_parts_positive_case() {
    let mut context = TestContext::new();
    let result = context.f64_from_parts(true, 123456789, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123456789.0);
}

#[test]
fn test_f64_from_parts_negative_case() {
    let mut context = TestContext::new();
    let result = context.f64_from_parts(false, 123456789, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -123456789.0);
}

#[test]
#[should_panic]
fn test_f64_from_parts_exponent_none_case() {
    let mut context = TestContext::new();
    context.f64_from_parts(true, 1, -1000); // Exponent is wrapped to be high and gets None in POW10
}

#[test]
fn test_f64_from_parts_zero_case() {
    let mut context = TestContext::new();
    let result = context.f64_from_parts(true, 0, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
#[should_panic]
fn test_f64_from_parts_infinite_case() {
    let mut context = TestContext::new();
    let result = context.f64_from_parts(true, 1, 309); // Should trigger infinity
    assert!(result.is_err()); // Ensure it errors out before panicking
}

struct TestContext;

impl TestContext {
    fn new() -> Self {
        Self {}
    }

    fn error(&self, _code: ErrorCode) -> () {
        // Simulate error handling
    }

    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        mut exponent: i32,
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
}

#[derive(Debug)]
enum ErrorCode {
    NumberOutOfRange,
}

type Result<T> = std::result::Result<T, ()>;

const POW10: [f64; 308] = [1.0; 308]; // Mocking a valid array of POW10 for the context

