// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct Bucket<'a> {
        key: &'a str,
    }

    struct TestStruct<'a> {
        iter: Vec<Bucket<'a>>,
    }

    impl<'a> TestStruct<'a> {
        fn new(keys: Vec<&'a str>) -> TestStruct<'a> {
            TestStruct {
                iter: keys.into_iter().map(|key| Bucket { key }).collect(),
            }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| bucket.key);
            f.debug_list().entries(iter).finish()
        }
    }

    let keys = vec!["key1", "key2", "key3"];
    let test_struct = TestStruct::new(keys);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_struct);
    
    assert!(result.is_ok());
    assert_eq!(output, "[\"key1\", \"key2\", \"key3\"]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket<'a> {
        key: &'a str,
    }

    struct TestStruct<'a> {
        iter: Vec<Bucket<'a>>,
    }

    impl<'a> TestStruct<'a> {
        fn new(keys: Vec<&'a str>) -> TestStruct<'a> {
            TestStruct {
                iter: keys.into_iter().map(|key| Bucket { key }).collect(),
            }
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(|bucket| bucket.key);
            f.debug_list().entries(iter).finish()
        }
    }

    let test_struct = TestStruct::new(vec![]);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_struct);
    
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

