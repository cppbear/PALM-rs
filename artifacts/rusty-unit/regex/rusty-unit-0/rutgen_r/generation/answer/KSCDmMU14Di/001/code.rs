// Answer 0

#[derive(Default)]
struct Nfa {
    capture_name_idx: Arc<HashMap<String, usize>>,
}

#[derive(Default)]
struct Ro {
    nfa: Nfa,
}

struct TestStruct {
    ro: Ro,
}

impl TestStruct {
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
        &self.ro.nfa.capture_name_idx
    }
}

#[test]
fn test_capture_name_idx_empty() {
    let test_struct = TestStruct::default();
    let capture_name_idx = test_struct.capture_name_idx();
    assert!(capture_name_idx.is_empty());
}

#[test]
fn test_capture_name_idx_non_empty() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    
    let test_struct = TestStruct {
        ro: Ro {
            nfa: Nfa {
                capture_name_idx: Arc::new(map),
            },
        },
    };
    
    let capture_name_idx = test_struct.capture_name_idx();
    assert_eq!(capture_name_idx.len(), 2);
    assert_eq!(capture_name_idx.get("key1"), Some(&1));
    assert_eq!(capture_name_idx.get("key2"), Some(&2));
}

#[test]
fn test_capture_name_idx_specific_key() {
    let mut map = HashMap::new();
    map.insert("test_key".to_string(), 42);
    
    let test_struct = TestStruct {
        ro: Ro {
            nfa: Nfa {
                capture_name_idx: Arc::new(map),
            },
        },
    };
    
    let capture_name_idx = test_struct.capture_name_idx();
    assert_eq!(capture_name_idx.get("test_key"), Some(&42));
}

#[test]
#[should_panic]
fn test_capture_name_idx_panic_on_invalid_key() {
    let mut map = HashMap::new();
    map.insert("temp_key".to_string(), 10);
    
    let test_struct = TestStruct {
        ro: Ro {
            nfa: Nfa {
                capture_name_idx: Arc::new(map),
            },
        },
    };
    
    let capture_name_idx = test_struct.capture_name_idx();
    
    // Attempting to access a non-existent key to trigger panic
    let _ = capture_name_idx.get("non_existent_key").unwrap();
}

