// Answer 0

#[test]
fn test_fmt_with_named_groups() {
    use std::collections::HashMap;
    use std::fmt;

    struct TestData {
        named_groups: HashMap<usize, String>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    struct TestWrapper(TestData);

    impl fmt::Debug for TestWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let slot_to_name: HashMap<&usize, &String> =
                self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.0.locs.iter().enumerate() {
                let m = m.map(|(s, e)| &self.0.text[s..e]);
                if let Some(name) = slot_to_name.get(&slot) {
                    map.entry(&name, &m);
                } else {
                    map.entry(&slot, &m);
                }
            }
            map.finish()
        }
    }

    let mut named_groups = HashMap::new();
    named_groups.insert(0, "first".to_string());
    named_groups.insert(1, "second".to_string());

    let locs = vec![
        Some((0, 5)),
        Some((6, 11)),
        None, // Trigger the condition where (slot, m) is false
    ];

    let text = "hello world".to_string();

    let data = TestWrapper(TestData {
        named_groups,
        locs,
        text,
    });

    let result = format!("{:?}", data);
    assert!(result.contains("first"));
    assert!(result.contains("second"));
    assert!(result.contains("0"));
    assert!(result.contains("1"));
    assert!(result.contains("None"));
} 

#[test]
fn test_fmt_with_empty_named_groups() {
    use std::collections::HashMap;
    use std::fmt;

    struct TestData {
        named_groups: HashMap<usize, String>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    struct TestWrapper(TestData);

    impl fmt::Debug for TestWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let slot_to_name: HashMap<&usize, &String> =
                self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.0.locs.iter().enumerate() {
                let m = m.map(|(s, e)| &self.0.text[s..e]);
                if let Some(name) = slot_to_name.get(&slot) {
                    map.entry(&name, &m);
                } else {
                    map.entry(&slot, &m);
                }
            }
            map.finish()
        }
    }

    let named_groups = HashMap::new(); // No named groups

    let locs = vec![
        Some((0, 5)),
        Some((6, 11)),
    ];

    let text = "hello world".to_string();

    let data = TestWrapper(TestData {
        named_groups,
        locs,
        text,
    });

    let result = format!("{:?}", data);
    assert!(result.contains("0"));
    assert!(result.contains("1"));
    assert!(result.contains("Some"));
} 

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    use std::collections::HashMap;
    use std::fmt;

    struct TestData {
        named_groups: HashMap<usize, String>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    struct TestWrapper(TestData);

    impl fmt::Debug for TestWrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let slot_to_name: HashMap<&usize, &String> =
                self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.0.locs.iter().enumerate() {
                let m = m.map(|(s, e)| &self.0.text[s..e]);
                // Trigger panic by trying to access a slot that doesn't exist in named_groups
                let name = slot_to_name.get(&slot).unwrap(); // Will panic if slot isn't found
                map.entry(&name, &m);
            }
            map.finish()
        }
    }

    let mut named_groups = HashMap::new();
    named_groups.insert(0, "first".to_string());

    let locs = vec![
        Some((0, 5)), // This will trigger a panic when trying to find slot 1
    ];

    let text = "hello".to_string();

    let data = TestWrapper(TestData {
        named_groups,
        locs,
        text,
    });

    let _result = format!("{:?}", data); // This should panic
}

