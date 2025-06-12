// Answer 0

fn test_try_insert2_success() {
    struct TestHeaderName(String);

    impl Hash for TestHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Into<HeaderName> for TestHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.0) }
        }
    }

    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(1);

    let key = TestHeaderName("test_key".to_string());
    let value = 42;

    let result = map.try_insert2(key, value);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

fn test_try_insert2_fail_reserve_one() {
    struct TestHeaderName(String);

    impl Hash for TestHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Into<HeaderName> for TestHeaderName {
        fn into(self) -> HeaderName {
            HeaderName { inner: Repr::Custom(self.0) }
        }
    }

    let mut map: HeaderMap<i32> = HeaderMap::try_with_capacity(0).unwrap_err();

    let key = TestHeaderName("test_key".to_string());
    let value = 42;

    let result = map.try_insert2(key, value);
    assert!(result.is_err());
}

