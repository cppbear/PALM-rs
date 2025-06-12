// Answer 0

#[test]
fn test_version_ref_default() {
    struct Request {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        version: Version,
    }

    enum Version {
        HTTP_11,
        HTTP_2,
    }

    impl Request {
        pub fn builder() -> Self {
            Self {
                inner: Ok(Inner {
                    version: Version::HTTP_11,
                }),
            }
        }

        pub fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let req = Request::builder();
    assert_eq!(req.version_ref().unwrap(), &Version::HTTP_11);
}

#[test]
fn test_version_ref_http2() {
    struct Request {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        version: Version,
    }

    enum Version {
        HTTP_11,
        HTTP_2,
    }

    impl Request {
        pub fn builder() -> Self {
            Self {
                inner: Ok(Inner {
                    version: Version::HTTP_11,
                }),
            }
        }

        pub fn version(&mut self, version: Version) -> &mut Self {
            if let Ok(inner) = &mut self.inner {
                inner.version = version;
            }
            self
        }

        pub fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let mut req = Request::builder();
    req.version(Version::HTTP_2);
    assert_eq!(req.version_ref().unwrap(), &Version::HTTP_2);
}

#[test]
fn test_version_ref_err() {
    struct Request {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        version: Version,
    }

    enum Version {
        HTTP_11,
        HTTP_2,
    }

    impl Request {
        pub fn builder() -> Self {
            Self {
                inner: Err(()),
            }
        }

        pub fn version_ref(&self) -> Option<&Version> {
            self.inner.as_ref().ok().map(|h| &h.version)
        }
    }

    let req = Request::builder();
    assert_eq!(req.version_ref(), None);
}

