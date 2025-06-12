// Answer 0

#[test]
fn test_fmt_output() {
    use std::fmt;

    struct MockEntry {
        key_value: i32,
        value: i32,
    }

    impl MockEntry {
        fn key(&self) -> &i32 {
            &self.key_value
        }

        fn get(&self) -> &i32 {
            &self.value
        }
    }

    struct MockStruct {
        value: i32,
        entry: MockEntry,
    }

    impl fmt::Debug for MockStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MockStruct")
        }
    }

    impl fmt::Display for MockStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "failed to insert {:?}, key {:?} already exists with value {:?}",
                self.value,
                self.entry.key(),
                self.entry.get(),
            )
        }
    }

    let entry = MockEntry {
        key_value: 1,
        value: 10,
    };
    
    let instance = MockStruct {
        value: 20,
        entry,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", instance);
    
    assert!(result.is_ok());
    assert_eq!(output, "failed to insert 20, key 1 already exists with value 10");
}

