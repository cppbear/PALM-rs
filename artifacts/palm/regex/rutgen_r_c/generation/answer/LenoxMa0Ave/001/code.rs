// Answer 0

#[test]
fn test_reset_size_with_empty_start_states_and_stack() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: Vec::new(),
            num_byte_classes: 0,
        },
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };
    
    cache.reset_size();
    assert_eq!(cache.size, 0);
}

#[test]
fn test_reset_size_with_non_empty_start_states_and_empty_stack() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: Vec::new(),
            num_byte_classes: 0,
        },
        states: Vec::new(),
        start_states: vec![1, 2, 3], // 3 start states
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };
    
    cache.reset_size();
    assert_eq!(cache.size, 3 * std::mem::size_of::<StatePtr>());
}

#[test]
fn test_reset_size_with_empty_start_states_and_non_empty_stack() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: Vec::new(),
            num_byte_classes: 0,
        },
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![1, 2, 3], // 3 inst pointers
        flush_count: 0,
        size: 0,
    };
    
    cache.reset_size();
    assert_eq!(cache.size, 3 * std::mem::size_of::<InstPtr>());
}

#[test]
fn test_reset_size_with_non_empty_start_states_and_non_empty_stack() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: Vec::new(),
            num_byte_classes: 0,
        },
        states: Vec::new(),
        start_states: vec![1, 2, 3], // 3 start states
        stack: vec![1, 2, 3, 4], // 4 inst pointers
        flush_count: 0,
        size: 0,
    };
    
    cache.reset_size();
    assert_eq!(cache.size, (3 * std::mem::size_of::<StatePtr>()) + (4 * std::mem::size_of::<InstPtr>()));
}

