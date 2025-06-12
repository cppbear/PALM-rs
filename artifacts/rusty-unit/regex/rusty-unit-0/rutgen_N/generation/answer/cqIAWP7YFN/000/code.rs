// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use std::collections::HashMap;

    struct NamedGroups {
        named_groups: HashMap<usize, String>,
    }

    struct Locs {
        locs: Vec<Option<(usize, usize)>>,
        text: Vec<u8>,
    }

    struct TestStruct(NamedGroups, Locs);

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fn escape_bytes(bytes: &[u8]) -> String {
                fn escape_byte(byte: u8) -> String {
                    use std::ascii::escape_default;
                    let escaped: Vec<u8> = escape_default(byte).collect();
                    String::from_utf8_lossy(&escaped).into_owned()
                }

                let mut s = String::new();
                for &b in bytes {
                    s.push_str(&escape_byte(b));
                }
                s
            }

            let slot_to_name: HashMap<&usize, &String> =
                self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
            let mut map = f.debug_map();
            for (slot, m) in self.1.locs.iter().enumerate() {
                let m = m.map(|(s, e)| escape_bytes(&self.1.text[s..e]));
                if let Some(name) = slot_to_name.get(&slot) {
                    map.entry(name, &m);
                } else {
                    map.entry(&slot, &m);
                }
            }
            map.finish()
        }
    }

    let named_groups = HashMap::from([(0, String::from("Group1")), (1, String::from("Group2"))]);
    let locs = vec![Some((0, 3)), Some((4, 7)), None];
    let text = b"abc def".to_vec();
    
    let test_instance = TestStruct(NamedGroups { named_groups }, Locs { locs, text });

    let mut output = String::new();
    {
        let f = &mut fmt::Formatter::new(&mut output);
        test_instance.fmt(f).unwrap();
    }

    assert!(!output.is_empty());
}

