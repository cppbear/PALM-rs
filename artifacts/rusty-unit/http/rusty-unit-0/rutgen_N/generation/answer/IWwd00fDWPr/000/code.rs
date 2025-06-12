// Answer 0

#[test]
fn test_scheme_valid() {
    struct Builder {
        scheme: Option<Scheme>,
    }

    impl Builder {
        fn new() -> Self {
            Self { scheme: None }
        }
    
        fn scheme<T>(self, scheme: T) -> Self
        where
            T: TryInto<Scheme>,
            <T as TryInto<Scheme>>::Error: Into<crate::Error>,
        {
            let scheme_result = scheme.try_into().map_err(Into::into);
            if let Ok(valid_scheme) = scheme_result {
                self.scheme = Some(valid_scheme);
            }
            self
        }
    }

    struct Scheme {
        name: String,
    }

    impl TryInto<Scheme> for &str {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            if self.is_empty() {
                Err(crate::Error::InvalidScheme)
            } else {
                Ok(Scheme { name: self.to_string() })
            }
        }
    }

    let builder = Builder::new();
    let updated_builder = builder.scheme("https");
    assert!(updated_builder.scheme.is_some());
    assert_eq!(updated_builder.scheme.unwrap().name, "https");
}

#[test]
#[should_panic]
fn test_scheme_invalid() {
    struct Builder {
        scheme: Option<Scheme>,
    }

    impl Builder {
        fn new() -> Self {
            Self { scheme: None }
        }

        fn scheme<T>(self, scheme: T) -> Self
        where
            T: TryInto<Scheme>,
            <T as TryInto<Scheme>>::Error: Into<crate::Error>,
        {
            let scheme_result = scheme.try_into().map_err(Into::into);
            if let Ok(valid_scheme) = scheme_result {
                self.scheme = Some(valid_scheme);
            }
            self
        }
    }

    struct Scheme {
        name: String,
    }

    impl TryInto<Scheme> for &str {
        type Error = crate::Error;

        fn try_into(self) -> Result<Scheme, Self::Error> {
            if self.is_empty() {
                Err(crate::Error::InvalidScheme)
            } else {
                Ok(Scheme { name: self.to_string() })
            }
        }
    }

    let builder = Builder::new();
    let _updated_builder = builder.scheme("");
}

