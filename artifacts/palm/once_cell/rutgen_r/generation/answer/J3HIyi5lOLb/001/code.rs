// Answer 0

#[test]
fn test_fmt() {
    struct OnceRef {
        inner: Option<String>,
    }

    impl core::fmt::Debug for OnceRef {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "OnceRef({:?})", self.inner)
        }
    }

    // Test for a normal case with a valid inner value
    {
        let value = OnceRef {
            inner: Some("test".to_string()),
        };
        let mut output = String::new();
        let result = core::fmt::write(&mut output, |f| value.fmt(f));
        assert!(result.is_ok());
        assert_eq!(output, "OnceRef(Some(\"test\"))");
    }

    // Test for an empty inner value
    {
        let value = OnceRef {
            inner: None,
        };
        let mut output = String::new();
        let result = core::fmt::write(&mut output, |f| value.fmt(f));
        assert!(result.is_ok());
        assert_eq!(output, "OnceRef(None)");
    }
    
    // Test for a long string to check for boundary conditions
    {
        let long_string = "a".repeat(1000);
        let value = OnceRef {
            inner: Some(long_string.clone()),
        };
        let mut output = String::new();
        let result = core::fmt::write(&mut output, |f| value.fmt(f));
        assert!(result.is_ok());
        assert_eq!(output, format!("OnceRef(Some(\"{}\"))", long_string));
    }
}

