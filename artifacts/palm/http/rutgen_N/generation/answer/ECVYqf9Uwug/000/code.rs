// Answer 0

#[derive(Debug)]
struct Extensions {
    map: Option<std::collections::HashMap<String, String>>,
}

impl Extensions {
    pub fn new() -> Extensions {
        Extensions { map: None }
    }
}

#[test]
fn test_extensions_new() {
    let extensions = Extensions::new();
    assert_eq!(extensions.map, None);
}

