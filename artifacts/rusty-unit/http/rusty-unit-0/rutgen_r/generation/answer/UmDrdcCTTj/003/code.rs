// Answer 0

#[test]
fn test_is_idempotent_put() {
    struct Method(Put);

    impl Method {
        pub fn is_safe(&self) -> bool {
            false
        }
        
        pub fn is_idempotent(&self) -> bool {
            match self.0 {
                Put | Delete => true,
                _ => self.is_safe(),
            }
        }
    }

    let method = Method(Put);
    assert!(method.is_idempotent());
}

#[test]
fn test_is_idempotent_delete() {
    struct Method(Delete);

    impl Method {
        pub fn is_safe(&self) -> bool {
            false
        }
        
        pub fn is_idempotent(&self) -> bool {
            match self.0 {
                Put | Delete => true,
                _ => self.is_safe(),
            }
        }
    }

    let method = Method(Delete);
    assert!(method.is_idempotent());
}

