// Answer 0

#[derive(Debug)]
struct KeyClass {
    value: String,
}

impl KeyClass {
    fn Map(s: String) -> Self {
        KeyClass { value: s }
    }

    #[cfg(feature = "arbitrary_precision")]
    fn Number() -> Self {
        KeyClass { value: "number".to_string() }
    }

    #[cfg(feature = "raw_value")]
    fn RawValue() -> Self {
        KeyClass { value: "raw_value".to_string() }
    }
}

struct TestError;
impl de::Error for TestError {
    fn custom<T>(_msg: T) -> Self {
        TestError
    }
}

fn visit_string<E>(s: String) -> Result<KeyClass, E>
where
    E: de::Error,
{
    match s.as_str() {
        #[cfg(feature = "arbitrary_precision")]
        "number" => Ok(KeyClass::Number()),
        #[cfg(feature = "raw_value")]
        "raw_value" => Ok(KeyClass::RawValue()),
        _ => Ok(KeyClass::Map(s)),
    }
}

#[test]
fn test_visit_string_map() {
    let result = visit_string::<TestError>("some_key".to_string()).unwrap();
    assert_eq!(result.value, "some_key");
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_visit_string_number() {
    let result = visit_string::<TestError>("number".to_string()).unwrap();
    assert_eq!(result.value, "number");
}

#[test]
#[cfg(feature = "raw_value")]
fn test_visit_string_raw_value() {
    let result = visit_string::<TestError>("raw_value".to_string()).unwrap();
    assert_eq!(result.value, "raw_value");
}

