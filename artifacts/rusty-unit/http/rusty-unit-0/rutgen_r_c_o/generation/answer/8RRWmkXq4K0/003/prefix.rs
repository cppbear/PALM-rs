// Answer 0

#[test]
fn test_hash_http_scheme() {
    use std::collections::hash_map::DefaultHasher;
    
    struct Protocol;
    impl Protocol {
        const Http: Self = Self;
        const Https: Self = Self;
    }
    
    let mut hasher = DefaultHasher::new();
    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    };
    scheme.hash(&mut hasher);
}

#[test]
fn test_hash_https_scheme() {
    use std::collections::hash_map::DefaultHasher;
    
    struct Protocol;
    impl Protocol {
        const Http: Self = Self;
        const Https: Self = Self;
    }
    
    let mut hasher = DefaultHasher::new();
    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Https),
    };
    scheme.hash(&mut hasher);
}

