// Answer 0

fn test_fmt_empty_locs() {
    use std::fmt;
    use std::collections::HashMap;

    struct TestStruct {
        named_groups: HashMap<usize, String>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    struct Wrapper(TestStruct);

    impl fmt::Debug for Wrapper {
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

    let named_groups = HashMap::new();
    let locs = vec![]; // Empty locs to meet the constraint
    let text = String::from("Sample text");

    let wrapper = Wrapper(TestStruct {
        named_groups,
        locs,
        text,
    });

    let result = format!("{:?}", wrapper);
    
    assert_eq!(result, "{}");
}

fn test_fmt_single_loc() {
    use std::fmt;
    use std::collections::HashMap;

    struct TestStruct {
        named_groups: HashMap<usize, String>,
        locs: Vec<Option<(usize, usize)>>,
        text: String,
    }

    struct Wrapper(TestStruct);

    impl fmt::Debug for Wrapper {
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
    named_groups.insert(0, String::from("group0"));
    let locs = vec![Some((0, 4))]; // One loc present
    let text = String::from("Text");

    let wrapper = Wrapper(TestStruct {
        named_groups,
        locs,
        text,
    });

    let result = format!("{:?}", wrapper);
    
    assert_eq!(result, "{\"group0\": Some(\"Text\")}");
}

