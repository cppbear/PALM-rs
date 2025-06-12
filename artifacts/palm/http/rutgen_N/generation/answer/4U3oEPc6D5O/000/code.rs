// Answer 0

#[derive(Debug)]
struct Parts;

#[derive(Debug)]
struct Builder {
    parts: Option<Parts>,
}

#[derive(Debug)]
struct Error;

impl Builder {
    fn map<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, Error>,
    {
        Builder {
            parts: self.parts.and_then(|p| func(p).ok()),
        }
    }
}

#[test]
fn test_map_success() {
    let initial_parts = Some(Parts);
    let builder = Builder { parts: initial_parts };

    let result = builder.map(|p| Ok(p));

    assert!(result.parts.is_some());
}

#[test]
fn test_map_failure() {
    let initial_parts = Some(Parts);
    let builder = Builder { parts: initial_parts };

    let result = builder.map(|_p| Err(Error));

    assert!(result.parts.is_none());
}

#[test]
fn test_map_with_none() {
    let builder = Builder { parts: None };

    let result = builder.map(|p| Ok(p));

    assert!(result.parts.is_none());
}

