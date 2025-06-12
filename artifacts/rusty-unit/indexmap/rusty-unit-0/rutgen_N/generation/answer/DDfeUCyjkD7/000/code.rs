// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use indexmap::IndexMap;

    // Define a struct that will use the method being tested
    struct MyMap {
        map: IndexMap<i32, i32>,
    }

    impl fmt::Debug for MyMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.map.clone()).finish()
        }
    }

    // Initialize the map with some values
    let mut my_map = MyMap {
        map: IndexMap::new(),
    };
    my_map.map.insert(1, 10);
    my_map.map.insert(2, 20);
    my_map.map.insert(3, 30);

    // Test the formatting
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", my_map);
    
    assert!(result.is_ok());
    assert!(output.contains("1")); // Checking if it contained the key
    assert!(output.contains("2")); // Checking if it contained the key
    assert!(output.contains("3")); // Checking if it contained the key
}

