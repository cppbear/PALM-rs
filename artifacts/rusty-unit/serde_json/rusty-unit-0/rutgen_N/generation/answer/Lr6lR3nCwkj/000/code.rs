// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Number;

    #[test]
    fn test_from_f64_finite() {
        let value = 256.0;
        let result = Number::from_f64(value);
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_string(), "256");
    }

    #[test]
    fn test_from_f64_negative_finite() {
        let value = -256.0;
        let result = Number::from_f64(value);
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_string(), "-256");
    }

    #[test]
    fn test_from_f64_infinity() {
        let value = f64::INFINITY;
        let result = Number::from_f64(value);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_f64_negative_infinity() {
        let value = f64::NEG_INFINITY;
        let result = Number::from_f64(value);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_f64_nan() {
        let value = f64::NAN;
        let result = Number::from_f64(value);
        assert!(result.is_none());
    }
}

