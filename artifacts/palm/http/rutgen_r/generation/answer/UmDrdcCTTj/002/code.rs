// Answer 0

#[derive(Copy, Clone)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

struct Method(HttpMethod);

impl Method {
    pub fn is_safe(&self) -> bool {
        matches!(self.0, HttpMethod::Get)
    }

    pub fn is_idempotent(&self) -> bool {
        match self.0 {
            HttpMethod::Put | HttpMethod::Delete => true,
            _ => self.is_safe(),
        }
    }
}

#[test]
fn test_is_idempotent_delete() {
    let method = Method(HttpMethod::Delete);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_put() {
    let method = Method(HttpMethod::Put);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_get() {
    let method = Method(HttpMethod::Get);
    assert_eq!(method.is_idempotent(), true);
}

#[test]
fn test_is_idempotent_post() {
    let method = Method(HttpMethod::Post);
    assert_eq!(method.is_idempotent(), false);
}

