// Answer 0

#[test]
fn test_uri_builder_creation() {
    struct Builder {
        scheme: Option<String>,
        authority: Option<String>,
        path_and_query: Option<String>,
    }

    impl Builder {
        fn new() -> Self {
            Builder {
                scheme: None,
                authority: None,
                path_and_query: None,
            }
        }

        fn scheme(mut self, scheme: &str) -> Self {
            self.scheme = Some(scheme.to_string());
            self
        }

        fn authority(mut self, authority: &str) -> Self {
            self.authority = Some(authority.to_string());
            self
        }

        fn path_and_query(mut self, path_and_query: &str) -> Self {
            self.path_and_query = Some(path_and_query.to_string());
            self
        }

        fn build(self) -> Result<String, &'static str> {
            if self.scheme.is_none() || self.authority.is_none() {
                return Err("Scheme and authority must be set");
            }
            let uri = format!("{}://{}{}", self.scheme.unwrap(), self.authority.unwrap(), self.path_and_query.unwrap_or_else(|| "".to_string()));
            Ok(uri)
        }
    }

    let uri = Builder::new()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/")
        .build()
        .unwrap();

    assert_eq!(uri, "https://hyper.rs/");
} 

#[test]
#[should_panic(expected = "Scheme and authority must be set")]
fn test_uri_builder_creation_without_scheme_or_authority() {
    struct Builder {
        scheme: Option<String>,
        authority: Option<String>,
        path_and_query: Option<String>,
    }

    impl Builder {
        fn new() -> Self {
            Builder {
                scheme: None,
                authority: None,
                path_and_query: None,
            }
        }

        fn scheme(mut self, scheme: &str) -> Self {
            self.scheme = Some(scheme.to_string());
            self
        }

        fn authority(mut self, authority: &str) -> Self {
            self.authority = Some(authority.to_string());
            self
        }

        fn path_and_query(mut self, path_and_query: &str) -> Self {
            self.path_and_query = Some(path_and_query.to_string());
            self
        }

        fn build(self) -> Result<String, &'static str> {
            if self.scheme.is_none() || self.authority.is_none() {
                return Err("Scheme and authority must be set");
            }
            let uri = format!("{}://{}{}", self.scheme.unwrap(), self.authority.unwrap(), self.path_and_query.unwrap_or_else(|| "".to_string()));
            Ok(uri)
        }
    }

    let _uri = Builder::new()
        .path_and_query("/")
        .build()
        .unwrap();
}

