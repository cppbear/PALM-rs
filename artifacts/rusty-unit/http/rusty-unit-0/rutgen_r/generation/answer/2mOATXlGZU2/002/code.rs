// Answer 0

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_should_panic() {
    struct Header {
        data: std::collections::HashMap<String, String>,
    }

    impl Header {
        fn get2(&self, key: &str) -> Option<&String> {
            self.data.get(key)
        }

        fn index(&self, index: String) -> &String {
            match self.get2(&index) {
                Some(val) => val,
                None => panic!("no entry found for key {:?}", index),
            }
        }
    }

    let header = Header {
        data: std::collections::HashMap::new(),
    };

    let panic_key = "missing_key".to_string();
    header.index(panic_key);
}

