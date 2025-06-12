// Answer 0

#[derive(Default)]
struct DummyErrorHandler;

impl DummyErrorHandler {
    fn error(&self, _: ErrorCode) -> String {
        String::from("Error occurred")
    }
}

#[derive(Debug)]
enum ErrorCode {
    NumberOutOfRange,
}

struct TestStruct {
    error_handler: DummyErrorHandler,
}

impl TestStruct {
    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        mut exponent: i32,
    ) -> Result<f64, String> {
        let mut f = significand as f64;
        loop {
            match POW10.get(exponent.wrapping_abs() as usize) {
                Some(&pow) => {
                    if exponent >= 0 {
                        f *= pow;
                        if f.is_infinite() {
                            return Err(self.error_handler.error(ErrorCode::NumberOutOfRange));
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
                        return Err(self.error_handler.error(ErrorCode::NumberOutOfRange));
                    }
                    f /= 1e308;
                    exponent += 308;
                }
            }
        }
        Ok(if positive { f } else { -f })
    }
}

lazy_static::lazy_static! {
    static ref POW10: Vec<f64> = (0..309).map(|x| 10f64.powi(x)).collect();
}

#[test]
fn test_f64_from_parts_zero_significand() {
    let mut test_struct = TestStruct::default();
    let result = test_struct.f64_from_parts(false, 0, -1);
    assert_eq!(result, Ok(0.0));  // f == 0.0 is true, should return Ok(0.0)
}

#[test]
fn test_f64_from_parts_non_zero_significand() {
    let mut test_struct = TestStruct::default();
    let result = test_struct.f64_from_parts(false, 1, -1);
    assert_eq!(result, Ok(-1e-1));  // positive is false, should return negative value
}

#[test]
fn test_f64_from_parts_out_of_range_exponent() {
    let mut test_struct = TestStruct::default();
    let result = test_struct.f64_from_parts(false, 1, 308);
    assert!(result.is_err());  // Exponent leads to out of range error
}

