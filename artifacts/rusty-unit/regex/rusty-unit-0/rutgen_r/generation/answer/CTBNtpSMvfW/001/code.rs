// Answer 0

#[test]
fn test_locations() {
    struct TestSearcher {
        data: &'static str,
    }

    impl TestSearcher {
        fn searcher_str(&self) -> &str {
            self.data
        }
    }

    struct TestStruct(TestSearcher);

    impl TestStruct {
        pub fn locations(&self) -> Vec<usize> {
            let search_str = self.0.searcher_str();
            // Simulate finding all positions of a target substring as locations.
            let target = "a";
            let mut locations = Vec::new();
            for (i, c) in search_str.chars().enumerate() {
                if c.to_string() == target {
                    locations.push(i);
                }
            }
            locations
        }
    }

    let test_instance = TestStruct(TestSearcher { data: "abcabc" });
    let result = test_instance.locations();
    assert_eq!(result, vec![0, 3]);

    let empty_instance = TestStruct(TestSearcher { data: "" });
    let empty_result = empty_instance.locations();
    assert_eq!(empty_result, Vec::<usize>::new());

    let single_char_instance = TestStruct(TestSearcher { data: "x" });
    let single_char_result = single_char_instance.locations();
    assert_eq!(single_char_result, Vec::<usize>::new());

    let multi_char_instance = TestStruct(TestSearcher { data: "hello world" });
    let multi_char_result = multi_char_instance.locations();
    assert_eq!(multi_char_result, Vec::<usize>::new());
}

