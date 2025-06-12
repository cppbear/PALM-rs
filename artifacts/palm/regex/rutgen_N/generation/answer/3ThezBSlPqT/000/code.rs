// Answer 0

#[test]
fn test_capture_name_idx() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct NFA {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct Regex {
        ro: NFA,
    }

    impl Regex {
        pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
            &self.ro.capture_name_idx
        }
    }

    let mut map = HashMap::new();
    map.insert("group1".to_string(), 1);
    map.insert("group2".to_string(), 2);
    let capture_name_idx = Arc::new(map);
    let nfa = NFA { capture_name_idx };
    let regex = Regex { ro: nfa };

    let result = regex.capture_name_idx();
    assert_eq!(result.get("group1"), Some(&1));
    assert_eq!(result.get("group2"), Some(&2));
}

#[test]
fn test_capture_name_idx_empty() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct NFA {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct Regex {
        ro: NFA,
    }

    impl Regex {
        pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
            &self.ro.capture_name_idx
        }
    }

    let capture_name_idx = Arc::new(HashMap::new());
    let nfa = NFA { capture_name_idx };
    let regex = Regex { ro: nfa };

    let result = regex.capture_name_idx();
    assert!(result.is_empty());
}

