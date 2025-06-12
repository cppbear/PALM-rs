// Answer 0

#[derive(Default)]
struct Parts;

struct Builder {
    inner: Result<Parts>,
}

impl Builder {
    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts>,
    {
        Builder {
            inner: self.inner.and_then(func),
        }
    }
}

#[test]
fn test_and_then_success() {
    let parts = Parts::default();
    let builder = Builder {
        inner: Ok(parts),
    };

    let new_builder = builder.and_then(|_| Ok(Parts::default()));
    assert!(new_builder.inner.is_ok());
}

#[test]
#[should_panic]
fn test_and_then_failure() {
    let parts = Parts::default();
    let builder = Builder {
        inner: Ok(parts),
    };

    let _new_builder = builder.and_then(|_| Err("error".into())).inner.unwrap();
}

