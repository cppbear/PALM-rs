// Answer 0

#[derive(Debug)]
struct Method(u32);

impl Method {
    const Put: u32 = 0;
    const Delete: u32 = 1;
    const Get: u32 = 2;
    const Post: u32 = 3;

    fn is_safe(&self) -> bool {
        match self.0 {
            Self::Get => true,
            _ => false,
        }
    }

    pub fn is_idempotent(&self) -> bool {
        match self.0 {
            Self::Put | Self::Delete => true,
            _ => self.is_safe(),
        }
    }
}

#[test]
fn test_idempotent_put() {
    let method = Method(Method::Put);
    assert!(method.is_idempotent());
}

#[test]
fn test_idempotent_delete() {
    let method = Method(Method::Delete);
    assert!(method.is_idempotent());
}

#[test]
fn test_idempotent_get() {
    let method = Method(Method::Get);
    assert!(method.is_idempotent());
}

#[test]
fn test_idempotent_post() {
    let method = Method(Method::Post);
    assert!(!method.is_idempotent());
}

