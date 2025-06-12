// Answer 0

#[test]
fn test_try_from_with_valid_map() {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    struct HeaderName(String);
    
    impl TryFrom<&str> for HeaderName {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("HeaderName cannot be empty")
            } else {
                Ok(HeaderName(value.to_string()))
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct HeaderValue(i32);

    impl TryFrom<i32> for HeaderValue {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value < 0 {
                Err("HeaderValue cannot be negative")
            } else {
                Ok(HeaderValue(value))
            }
        }
    }
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Content-Length", 42);
    map.insert("Content-Type", 0);
    
    let result: Result<Vec<(HeaderName, HeaderValue)>, &'static str> = try_from(&map);
    
    assert_eq!(result, Ok(vec![
        (HeaderName("Content-Length".to_string()), HeaderValue(42)),
        (HeaderName("Content-Type".to_string()), HeaderValue(0)),
    ]));
}

#[test]
#[should_panic(expected = "HeaderName cannot be empty")]
fn test_try_from_with_empty_header_name() {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    struct HeaderName(String);
    
    impl TryFrom<&str> for HeaderName {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("HeaderName cannot be empty")
            } else {
                Ok(HeaderName(value.to_string()))
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct HeaderValue(i32);

    impl TryFrom<i32> for HeaderValue {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value < 0 {
                Err("HeaderValue cannot be negative")
            } else {
                Ok(HeaderValue(value))
            }
        }
    }
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("", 42); // Invalid header name, must be empty

    let _result: Vec<(HeaderName, HeaderValue)> = try_from(&map).unwrap();
}

#[test]
#[should_panic(expected = "HeaderValue cannot be negative")]
fn test_try_from_with_negative_header_value() {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    struct HeaderName(String);
    
    impl TryFrom<&str> for HeaderName {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("HeaderName cannot be empty")
            } else {
                Ok(HeaderName(value.to_string()))
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct HeaderValue(i32);

    impl TryFrom<i32> for HeaderValue {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value < 0 {
                Err("HeaderValue cannot be negative")
            } else {
                Ok(HeaderValue(value))
            }
        }
    }
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Content-Length", -1); // Invalid value, must be non-negative

    let _result: Vec<(HeaderName, HeaderValue)> = try_from(&map).unwrap();
}

