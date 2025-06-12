// Answer 0

#[test]
fn test_capture_name_idx_valid() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct InnerStruct {
        capture_map: Arc<HashMap<String, usize>>,
    }

    impl InnerStruct {
        fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
            &self.capture_map
        }
    }

    struct OuterStruct(InnerStruct);

    let mut map = HashMap::new();
    map.insert("name1".to_string(), 1);
    map.insert("name2".to_string(), 2);
    
    let inner = InnerStruct {
        capture_map: Arc::new(map),
    };
    
    let outer = OuterStruct(inner);
    
    let result = outer.capture_name_idx();
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("name1"), Some(&1));
    assert_eq!(result.get("name2"), Some(&2));
}

#[test]
#[should_panic]
fn test_capture_name_idx_empty() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct InnerStruct {
        capture_map: Arc<HashMap<String, usize>>,
    }

    impl InnerStruct {
        fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
            &self.capture_map
        }
    }

    struct OuterStruct(InnerStruct);

    let inner = InnerStruct {
        capture_map: Arc::new(HashMap::new()),
    };

    let outer = OuterStruct(inner);
    
    // This should panic since the map is empty
    assert_eq!(outer.capture_name_idx().get("nonexistent"), None);
}

