// Answer 0

#[test]
fn test_capture_name_idx() {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct NFA {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct Regex {
        ro: RO,
    }

    struct RO {
        nfa: NFA,
    }

    let mut capture_map = HashMap::new();
    capture_map.insert("captured_name".to_string(), 1);
    let capture_name_idx = Arc::new(capture_map);

    let nfa = NFA {
        capture_name_idx: capture_name_idx.clone(),
    };

    let ro = RO { nfa };
    let regex = Regex { ro };

    assert_eq!(regex.capture_name_idx().get("captured_name"), Some(&1));
}

#[test]
fn test_capture_name_idx_empty() {
    use std::sync::{Arc, HashMap};

    struct NFA {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct Regex {
        ro: RO,
    }

    struct RO {
        nfa: NFA,
    }

    let capture_name_idx = Arc::new(HashMap::new());

    let nfa = NFA {
        capture_name_idx: capture_name_idx.clone(),
    };

    let ro = RO { nfa };
    let regex = Regex { ro };

    assert!(regex.capture_name_idx().is_empty());
}

