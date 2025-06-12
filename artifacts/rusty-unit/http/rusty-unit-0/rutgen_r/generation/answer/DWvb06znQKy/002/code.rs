// Answer 0

#[test]
fn test_query_with_valid_data() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }

    impl PathAndQuery {
        fn query(&self) -> Option<&str> {
            if self.query == usize::MAX {
                None
            } else {
                let i = self.query + 1;
                Some(&self.data[i as usize..])
            }
        }
    }

    let path_and_query = PathAndQuery {
        query: 16,
        data: String::from("/hello/world?key=value&foo=bar"),
    };
    
    assert_eq!(path_and_query.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_without_query_component() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }

    impl PathAndQuery {
        fn query(&self) -> Option<&str> {
            if self.query == usize::MAX {
                None
            } else {
                let i = self.query + 1;
                Some(&self.data[i as usize..])
            }
        }
    }

    let path_and_query = PathAndQuery {
        query: usize::MAX,
        data: String::from("/hello/world"),
    };

    assert!(path_and_query.query().is_none());
}

#[test]
#[should_panic(expected = "byte index out of range for `data`")]
fn test_query_panic_on_out_of_bounds() {
    struct PathAndQuery {
        query: usize,
        data: String,
    }

    impl PathAndQuery {
        fn query(&self) -> Option<&str> {
            if self.query == usize::MAX {
                None
            } else {
                let i = self.query + 1;
                Some(&self.data[i as usize..])
            }
        }
    }

    let path_and_query = PathAndQuery {
        query: 0,
        data: String::from("onlyquery"),
    };

    // This will panic as there are not enough bytes in `data` after the query index
    let _ = path_and_query.query();
}

