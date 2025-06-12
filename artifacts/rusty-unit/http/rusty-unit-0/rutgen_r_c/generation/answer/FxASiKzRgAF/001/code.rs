// Answer 0

#[test]
fn test_as_str_with_standard_header() {
    struct Custom;
    enum Repr<T> {
        Standard(String),
        Custom(T),
    }

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct HeaderName {
        inner: Repr<Custom>,
    }

    impl HeaderName {
        pub fn from_standard(source: &str) -> HeaderName {
            HeaderName {
                inner: Repr::Standard(source.to_string()),
            }
        }

        pub fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(v) => v,
                Repr::Custom(_) => panic!("Called as_str on a Custom type"),
            }
        }
    }

    let header = HeaderName::from_standard("Content-Type");
    assert_eq!(header.as_str(), "Content-Type");
}

#[test]
#[should_panic(expected = "Called as_str on a Custom type")]
fn test_as_str_with_custom_header() {
    struct CustomData(String);
    struct Custom;
    enum Repr<T> {
        Standard(String),
        Custom(T),
    }

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct HeaderName {
        inner: Repr<Custom>,
    }

    impl HeaderName {
        pub fn from_custom(data: CustomData) -> HeaderName {
            HeaderName {
                inner: Repr::Custom(data),
            }
        }

        pub fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(v) => v,
                Repr::Custom(_) => panic!("Called as_str on a Custom type"),
            }
        }
    }

    let header = HeaderName::from_custom(CustomData("custom_value".to_string()));
    header.as_str();
}

