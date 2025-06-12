// Answer 0

#[test]
fn test_hash_none_scheme() {
    let scheme = Scheme { inner: Scheme2::None };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scheme.hash(&mut hasher);
}

#[test]
fn test_hash_http_scheme() {
    #[derive(Clone, Debug)]
    enum Protocol {
        Http,
        Https,
    }
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Http) };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scheme.hash(&mut hasher);
}

#[test]
fn test_hash_https_scheme() {
    #[derive(Clone, Debug)]
    enum Protocol {
        Http,
        Https,
    }
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Https) };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scheme.hash(&mut hasher);
}

