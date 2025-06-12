// Answer 0

#[test]
fn test_entry_inserts_correctly() {
    struct HeaderNameStub {
        name: &'static str,
    }
    
    impl IntoHeaderName for HeaderNameStub {
        fn try_entry(self, map: &mut HeaderMap<u32>) -> Result<Entry<'_, u32>, InvalidHeaderName> {
            Ok(map.entry(self.name).or_insert(0))
        }
    }
    
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    
    let headers = &[HeaderNameStub { name: "content-length" }, HeaderNameStub { name: "x-hello" }];
    
    for header in headers {
        let counter = map.entry(header.clone()).or_insert(0);
        *counter += 1;
    }

    assert_eq!(map.get("content-length").unwrap(), &1);
    assert_eq!(map.get("x-hello").unwrap(), &1);
}

#[test]
fn test_entry_panics_on_exceeding_capacity() {
    struct HeaderNameStub {
        name: &'static str,
    }

    impl IntoHeaderName for HeaderNameStub {
        fn try_entry(self, map: &mut HeaderMap<u32>) -> Result<Entry<'_, u32>, InvalidHeaderName> {
            Ok(map.entry(self.name).or_insert(0))
        }
    }
    
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(1);
    
    let header = HeaderNameStub { name: "content-length" };
    map.entry(header.clone()).or_insert(0);
    
    let result = std::panic::catch_unwind(|| {
        map.entry(header).or_insert(0);
    });

    assert!(result.is_err());
}

#[test]
fn test_entry_increments_existing_value() {
    struct HeaderNameStub {
        name: &'static str,
    }

    impl IntoHeaderName for HeaderNameStub {
        fn try_entry(self, map: &mut HeaderMap<u32>) -> Result<Entry<'_, u32>, InvalidHeaderName> {
            Ok(map.entry(self.name).or_insert(0))
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
 
    let header = HeaderNameStub { name: "x-hello" };

    let counter = map.entry(header.clone()).or_insert(0);
    *counter += 1;
    
    let second_counter = map.entry(header).or_insert(0);
    *second_counter += 2;

    assert_eq!(map.get("x-hello").unwrap(), &3);
}

