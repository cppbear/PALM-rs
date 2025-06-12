// Answer 0

#[test]
fn test_fmt_with_named_groups() {
    use std::fmt;
    use std::collections::HashMap;

    struct TestStruct {
        named_groups: HashMap<String, usize>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    impl TestStruct {
        fn new(named_groups: HashMap<String, usize>, locs: Vec<Option<(usize, usize)>>, text: String) -> Self {
            TestStruct { named_groups, locs, text }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let slot_to_name: HashMap<&usize, &String> =
                self.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.locs.iter().enumerate() {
                let m = m.map(|(s, e)| &self.text[s..e]);
                if let Some(name) = slot_to_name.get(&slot) {
                    map.entry(name, &m);
                } else {
                    map.entry(&slot, &m);
                }
            }
            map.finish()
        }
    }

    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    named_groups.insert("group2".to_string(), 1);

    let locs = vec![Some((0, 5)), Some((6, 11)), None];
    let text = "Hello World".to_string();

    let test_struct = TestStruct::new(named_groups, locs, text);
    
    let result = format!("{:?}", test_struct);
    assert!(result.contains("group1"));
    assert!(result.contains("Hello"));
    assert!(result.contains("group2"));
    assert!(result.contains("World"));
}

#[test]
fn test_fmt_with_no_named_groups() {
    use std::fmt;
    use std::collections::HashMap;

    struct TestStruct {
        named_groups: HashMap<String, usize>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    impl TestStruct {
        fn new(named_groups: HashMap<String, usize>, locs: Vec<Option<(usize, usize)>>, text: String) -> Self {
            TestStruct { named_groups, locs, text }
        }
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let slot_to_name: HashMap<&usize, &String> =
                self.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.locs.iter().enumerate() {
                let m = m.map(|(s, e)| &self.text[s..e]);
                if let Some(name) = slot_to_name.get(&slot) {
                    map.entry(name, &m);
                } else {
                    map.entry(&slot, &m);
                }
            }
            map.finish()
        }
    }

    let named_groups = HashMap::new();

    let locs = vec![Some((0, 5)), Some((6, 11)), None];
    let text = "Hello World".to_string();

    let test_struct = TestStruct::new(named_groups, locs, text);
    
    let result = format!("{:?}", test_struct);
    assert!(result.contains("0"));
    assert!(result.contains("Hello"));
    assert!(result.contains("1"));
    assert!(result.contains("World"));
}

