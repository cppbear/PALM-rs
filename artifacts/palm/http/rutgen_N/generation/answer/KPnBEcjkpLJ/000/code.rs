// Answer 0

#[test]
fn test_authority_with_string() {
    struct Builder {
        parts: Option<Authority>,
    }

    impl Builder {
        fn new() -> Self {
            Self { parts: None }
        }

        fn map<F>(self, f: F) -> Result<Self, crate::Error>
        where
            F: FnOnce(Parts) -> Result<Parts, crate::Error>,
        {
            let parts = Parts { authority: None };
            let new_parts = f(parts)?;
            self.parts = Some(new_parts.authority);
            Ok(self)
        }

        fn authority<T>(self, auth: T) -> Self
        where
            T: TryInto<Authority>,
            <T as TryInto<Authority>>::Error: Into<crate::Error>,
        {
            self.map(move |mut parts| {
                let auth = auth.try_into().map_err(Into::into)?;
                parts.authority = Some(auth);
                Ok(parts)
            }).unwrap()
        }

        fn build(self) -> Result<Self, crate::Error> {
            Ok(self)
        }
    }

    struct Authority(String);
    
    struct Parts {
        authority: Option<Authority>,
    }

    impl TryInto<Authority> for &str {
        type Error = &'static str;

        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority(self.to_string()))
        }
    }

    let builder = Builder::new();
    let uri = builder
        .authority("tokio.rs")
        .build()
        .unwrap();

    assert!(uri.parts.is_some());
}

#[test]
fn test_authority_with_invalid_string() {
    struct Builder {
        parts: Option<Authority>,
    }

    impl Builder {
        fn new() -> Self {
            Self { parts: None }
        }

        fn map<F>(self, f: F) -> Result<Self, crate::Error>
        where
            F: FnOnce(Parts) -> Result<Parts, crate::Error>,
        {
            let parts = Parts { authority: None };
            let new_parts = f(parts)?;
            self.parts = Some(new_parts.authority);
            Ok(self)
        }

        fn authority<T>(self, auth: T) -> Self
        where
            T: TryInto<Authority>,
            <T as TryInto<Authority>>::Error: Into<crate::Error>,
        {
            self.map(move |mut parts| {
                let auth = auth.try_into().map_err(Into::into)?;
                parts.authority = Some(auth);
                Ok(parts)
            }).unwrap()
        }

        fn build(self) -> Result<Self, crate::Error> {
            Ok(self)
        }
    }

    struct Authority(String);
    
    struct Parts {
        authority: Option<Authority>,
    }

    #[derive(Debug)]
    struct InvalidAuthority;

    impl TryInto<Authority> for InvalidAuthority {
        type Error = &'static str;

        fn try_into(self) -> Result<Authority, Self::Error> {
            Err("Invalid authority")
        }
    }

    let builder = Builder::new();

    let result = builder
        .authority(InvalidAuthority)
        .build();

    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_authority_should_panic() {
    struct Builder {
        parts: Option<Authority>,
    }

    impl Builder {
        fn new() -> Self {
            Self { parts: None }
        }

        fn map<F>(self, f: F) -> Result<Self, crate::Error>
        where
            F: FnOnce(Parts) -> Result<Parts, crate::Error>,
        {
            let parts = Parts { authority: None };
            let new_parts = f(parts)?;
            self.parts = Some(new_parts.authority);
            Ok(self)
        }

        fn authority<T>(self, auth: T) -> Self
        where
            T: TryInto<Authority>,
            <T as TryInto<Authority>>::Error: Into<crate::Error>,
        {
            self.map(move |mut parts| {
                let auth = auth.try_into().map_err(Into::into)?;
                parts.authority = Some(auth);
                Ok(parts)
            }).unwrap()
        }

        fn build(self) -> Result<Self, crate::Error> {
            self.unwrap();
            Ok(self)
        }
    }

    struct Authority(String);
    
    struct Parts {
        authority: Option<Authority>,
    }

    impl TryInto<Authority> for &str {
        type Error = &'static str;

        fn try_into(self) -> Result<Authority, Self::Error> {
            Ok(Authority(self.to_string()))
        }
    }

    let builder = Builder::new();
    
    builder
        .authority("")
        .build()
        .unwrap();
}

