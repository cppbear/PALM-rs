// Answer 0

#[test]
fn test_hash_with_scheme_only() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hasher, Hash};

    struct Scheme {
        inner: Option<String>,
    }

    impl Hash for Scheme {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if let Some(ref inner) = self.inner {
                inner.hash(state);
            }
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Option<String>,
        path: String,
        query: Option<String>,
    }

    impl Uri {
        fn authority(&self) -> Option<&String> {
            self.authority.as_ref()
        }

        fn path(&self) -> &String {
            &self.path
        }

        fn query(&self) -> Option<&String> {
            self.query.as_ref()
        }

        fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
        {
            if !self.scheme.inner.is_none() {
                self.scheme.hash(state);
                state.write_u8(0xff);
            }

            if let Some(auth) = self.authority() {
                auth.hash(state);
            }

            Hash::hash_slice(self.path().as_bytes(), state);

            if let Some(query) = self.query() {
                b'?'.hash(state);
                Hash::hash_slice(query.as_bytes(), state);
            }
        }
    }

    let mut hasher = DefaultHasher::new();
    let uri = Uri {
        scheme: Scheme { inner: Some("http".to_string()) },
        authority: None,
        path: "/path/to/resource".to_string(),
        query: Some("param=value".to_string()),
    };

    uri.hash(&mut hasher);
    let result = hasher.finish();
    assert_ne!(result, 0);
}

#[test]
fn test_hash_with_no_scheme() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hasher, Hash};

    struct Scheme {
        inner: Option<String>,
    }

    impl Hash for Scheme {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if let Some(ref inner) = self.inner {
                inner.hash(state);
            }
        }
    }

    struct Uri {
        scheme: Scheme,
        authority: Option<String>,
        path: String,
        query: Option<String>,
    }

    impl Uri {
        fn authority(&self) -> Option<&String> {
            self.authority.as_ref()
        }

        fn path(&self) -> &String {
            &self.path
        }

        fn query(&self) -> Option<&String> {
            self.query.as_ref()
        }

        fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
        {
            if !self.scheme.inner.is_none() {
                self.scheme.hash(state);
                state.write_u8(0xff);
            }

            if let Some(auth) = self.authority() {
                auth.hash(state);
            }

            Hash::hash_slice(self.path().as_bytes(), state);

            if let Some(query) = self.query() {
                b'?'.hash(state);
                Hash::hash_slice(query.as_bytes(), state);
            }
        }
    }

    let mut hasher = DefaultHasher::new();
    let uri = Uri {
        scheme: Scheme { inner: None },
        authority: None,
        path: "/path/to/resource".to_string(),
        query: None,
    };

    uri.hash(&mut hasher);
    let result = hasher.finish();
    assert_ne!(result, 0);
}

