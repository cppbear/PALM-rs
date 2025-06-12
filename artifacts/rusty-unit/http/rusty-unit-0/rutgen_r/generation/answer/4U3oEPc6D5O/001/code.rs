// Answer 0

#[derive(Debug)]
struct Parts {
    data: Option<String>,
}

#[derive(Debug)]
struct Error {
    message: String,
}

struct Builder {
    parts: Result<Parts, Error>,
}

impl Builder {
    fn map<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, Error>,
    {
        Builder {
            parts: self.parts.and_then(func),
        }
    }
}

#[test]
fn test_map_with_successful_function() {
    let initial_parts = Parts { data: Some("initial".to_string()) };
    let initial_builder = Builder {
        parts: Ok(initial_parts),
    };

    let result = initial_builder.map(|parts| {
        let new_data = parts.data.unwrap() + "_mapped";
        Ok(Parts { data: Some(new_data) })
    });

    assert_eq!(result.parts.is_ok(), true);
    assert_eq!(result.parts.unwrap().data, Some("initial_mapped".to_string()));
}

#[test]
fn test_map_with_failure_function() {
    let initial_parts = Parts { data: Some("initial".to_string()) };
    let initial_builder = Builder {
        parts: Ok(initial_parts),
    };

    let result = initial_builder.map(|_| Err(Error { message: "error occurred".to_string() }));

    assert_eq!(result.parts.is_err(), true);
    assert_eq!(result.parts.unwrap_err().message, "error occurred");
}

#[test]
fn test_map_with_none_data() {
    let initial_parts = Parts { data: None };
    let initial_builder = Builder {
        parts: Ok(initial_parts),
    };

    let result = initial_builder.map(|parts| {
        if parts.data.is_none() {
            Err(Error { message: "data is None".to_string() })
        } else {
            Ok(parts)
        }
    });

    assert_eq!(result.parts.is_err(), true);
    assert_eq!(result.parts.unwrap_err().message, "data is None");
}

#[test]
fn test_map_with_initial_error() {
    let initial_builder = Builder {
        parts: Err(Error { message: "initial error".to_string() }),
    };

    let result = initial_builder.map(|parts| Ok(parts));

    assert_eq!(result.parts.is_err(), true);
    assert_eq!(result.parts.unwrap_err().message, "initial error");
}

