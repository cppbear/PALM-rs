// Answer 0

#[test]
fn test_version_ref_default() {
    struct Inner {
        version: Version,
    }

    struct Request {
        inner: Result<Inner, ()>,
    }

    impl Request {
        fn builder() -> Self {
            Request { 
                inner: Ok(Inner { version: Version::HTTP_11 }) 
            }
        }

        fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let req = Request::builder();
    assert_eq!(req.version_ref().unwrap(), &Version::HTTP_11);
}

#[test]
fn test_version_ref_set_version() {
    struct Inner {
        version: Version,
    }

    struct Request {
        inner: Result<Inner, ()>,
    }

    impl Request {
        fn builder() -> Self {
            Request { 
                inner: Ok(Inner { version: Version::HTTP_11 }) 
            }
        }

        fn version(mut self, version: Version) -> Self {
            if let Ok(inner) = self.inner.as_mut() {
                inner.version = version;
            }
            self
        }

        fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let mut req = Request::builder();
    req = req.version(Version::HTTP_2);
    assert_eq!(req.version_ref().unwrap(), &Version::HTTP_2);
}

