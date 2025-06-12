// Answer 0

#[derive(Debug)]
struct Method(u8);

impl Method {
    const Put: u8 = 1;
    const Delete: u8 = 2;
    const Get: u8 = 3; // Safe
    const Head: u8 = 4; // Safe
    const Post: u8 = 5; // Not idempotent

    pub fn is_safe(&self) -> bool {
        matches!(self.0, Self::Get | Self::Head)
    }

    pub fn is_idempotent(&self) -> bool {
        match self.0 {
            Self::Put | Self::Delete => true,
            _ => self.is_safe(),
        }
    }
}

#[test]
fn test_idempotent_with_get() {
    let method = Method(Method::Get);
    assert!(method.is_idempotent());
}

#[test]
fn test_idempotent_with_head() {
    let method = Method(Method::Head);
    assert!(method.is_idempotent());
}

#[test]
fn test_idempotent_with_post() {
    let method = Method(Method::Post);
    assert!(!method.is_idempotent());
}

