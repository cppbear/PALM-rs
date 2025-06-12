// Answer 0

#[test]
fn test_version_ref_default() {
    struct MockBuilder {
        inner: Result<Parts>,
    }
    
    impl Builder {
        fn new() -> Self {
            Self {
                inner: Ok(Parts { version: Version(Http), ..Default::default() }),
            }
        }
    }
    
    let req = MockBuilder::new();
    assert_eq!(req.version_ref().unwrap(), &Version(Http));
}

#[test]
fn test_version_ref_after_setting() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl Builder {
        fn new() -> Self {
            Self {
                inner: Ok(Parts { version: Version(Http), ..Default::default() }),
            }
        }
        
        fn version(self, version: Version) -> Self {
            Self {
                inner: self.inner.map(|mut parts| {
                    parts.version = version;
                    parts
                }),
            }
        }
    }
    
    let mut req = MockBuilder::new();
    req = req.version(Version(Http));
    assert_eq!(req.version_ref().unwrap(), &Version(Http));
}

