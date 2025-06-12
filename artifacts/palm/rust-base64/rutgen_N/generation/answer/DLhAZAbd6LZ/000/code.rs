// Answer 0

#[derive(Debug)]
struct Alphabet {
    value: String,
}

impl Alphabet {
    fn new(value: &str) -> Result<Self, &'static str> {
        if value.is_empty() {
            Err("value cannot be empty")
        } else {
            Ok(Alphabet {
                value: value.to_string(),
            })
        }
    }
}

#[test]
fn test_try_from_valid_string() {
    let result = Alphabet::new("valid");
    assert!(result.is_ok());
    if let Ok(alphabet) = result {
        assert_eq!(alphabet.value, "valid");
    }
}

#[test]
fn test_try_from_empty_string() {
    let result = Alphabet::new("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "value cannot be empty");
}

