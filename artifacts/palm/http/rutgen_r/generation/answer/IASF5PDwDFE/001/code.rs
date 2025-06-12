// Answer 0

#[derive(Debug)]
struct Parts;

#[derive(Debug)]
struct Inner {
    value: Option<Parts>,
}

impl Inner {
    fn and_then<F>(&self, func: F) -> Result<Parts, &'static str>
    where
        F: FnOnce(Parts) -> Result<Parts, &'static str>,
    {
        match &self.value {
            Some(parts) => func(parts.clone()),
            None => Err("Inner is None"),
        }
    }
}

#[derive(Debug)]
struct Builder {
    inner: Inner,
}

impl Builder {
    fn new(inner: Inner) -> Self {
        Builder { inner }
    }

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, &'static str>,
    {
        Builder {
            inner: self.inner.and_then(func).unwrap_or_else(|_| Parts),
        }
    }
}

#[test]
fn test_and_then_with_some_inner() {
    let parts = Parts;
    let inner = Inner { value: Some(parts) };
    let builder = Builder::new(inner);

    let new_builder = builder.and_then(|_| Ok(Parts));
    assert!(new_builder.inner.value.is_some());
}

#[test]
fn test_and_then_with_none_inner() {
    let inner = Inner { value: None };
    let builder = Builder::new(inner);

    let new_builder = builder.and_then(|_| Ok(Parts));
    assert!(new_builder.inner.value.is_some()); // Should still be some Parts
}

#[test]
fn test_and_then_with_panic_condition() {
    let parts = Parts;
    let inner = Inner { value: Some(parts) };
    let builder = Builder::new(inner);

    let result = std::panic::catch_unwind(|| {
        builder.and_then(|_| panic!("Simulated panic"));
    });

    assert!(result.is_err());
}

