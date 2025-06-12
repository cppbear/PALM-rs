// Answer 0

#[test]
fn test_build_with_none_parts() {
    struct Builder {
        parts: Option<Result<(), ()>>,
    }

    impl Builder {
        fn parts(mut self) -> Self {
            self.parts = None;
            self
        }

        fn build(self) -> Result<Uri, crate::Error> {
            let parts = self.parts?;
            Uri::from_parts(parts).map_err(Into::into)
        }
    }

    let builder = Builder { parts: Some(Ok(())) };
    assert!(builder.build().is_err());
}

#[test]
fn test_build_with_err_parts() {
    struct Builder {
        parts: Option<Result<(), ()>>,
    }

    impl Builder {
        fn parts(mut self) -> Self {
            self.parts = Some(Err(()));
            self
        }

        fn build(self) -> Result<Uri, crate::Error> {
            let parts = self.parts?;
            Uri::from_parts(parts).map_err(Into::into)
        }
    }

    let builder = Builder { parts: Some(Err(())) };
    assert!(builder.build().is_err());
}

