// Answer 0

#[test]
fn test_try_insert2_empty_map() {
    struct MyHeaderName(String);

    impl Hash for MyHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<MyHeaderName> for HeaderName {
        fn from(name: MyHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(name.0) }
        }
    }

    // Create an instance of HeaderMap
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(1);

    // Attempt to insert a value into the empty map
    let key = MyHeaderName("Test-Header".to_string());
    let result = header_map.try_insert2(key, "Test-Value".to_string());

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_try_insert2_existing_key() {
    struct MyHeaderName(String);

    impl Hash for MyHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<MyHeaderName> for HeaderName {
        fn from(name: MyHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(name.0) }
        }
    }

    // Create an instance of HeaderMap and insert an initial value
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(1);
    let key = MyHeaderName("Test-Header".to_string());
    header_map.try_insert2(key.clone(), "First-Value".to_string()).unwrap();

    // Attempt to insert a value with the same key
    let result = header_map.try_insert2(key, "Second-Value".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("First-Value".to_string()));
}

#[test]
#[should_panic]
fn test_try_insert2_panic_on_reserve() {
    struct FailingHeaderName;

    impl Hash for FailingHeaderName {
        fn hash<H: Hasher>(&self, _: &mut H) {
            // This will cause a panic if `try_reserve_one` checks for capacity issues
            panic!("This should panic");
        }
    }

    impl From<FailingHeaderName> for HeaderName {
        fn from(_: FailingHeaderName) -> Self {
            HeaderName { inner: Repr::Custom("Failing-Header".to_string()) }
        }
    }

    // Create an instance of HeaderMap but we will try to reserve when it's already full
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(0); // Failing on reserve

    let key = FailingHeaderName;
    header_map.try_insert2(key, "Value".to_string()).unwrap();
}

#[test]
fn test_try_insert2_exceed_max_size() {
    struct MyHeaderName(String);

    impl Hash for MyHeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl From<MyHeaderName> for HeaderName {
        fn from(name: MyHeaderName) -> Self {
            HeaderName { inner: Repr::Custom(name.0) }
        }
    }

    // Create an instance of HeaderMap with maximum size
    let mut header_map: HeaderMap<String> = HeaderMap::try_with_capacity(MAX_SIZE).unwrap();

    for i in 0..MAX_SIZE {
        let key = MyHeaderName(format!("Header-{}", i));
        header_map.try_insert2(key, format!("Value-{}", i)).unwrap();
    }

    // Now trying to insert one more should exceed maximum size
    let result = header_map.try_insert2(MyHeaderName("Overflow".to_string()), "Too-Much".to_string());
    assert!(result.is_err());
}

