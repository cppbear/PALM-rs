// Answer 0

#[test]
fn test_reset_size_empty_cache() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: vec![], num_byte_classes: 0 },
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    cache_inner.reset_size();
    
    assert_eq!(cache_inner.size, 0);
}

#[test]
fn test_reset_size_some_start_states() {
    let start_state: StatePtr = 1;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: vec![], num_byte_classes: 0 },
        states: vec![],
        start_states: vec![start_state],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    cache_inner.reset_size();
    
    assert_eq!(cache_inner.size, mem::size_of::<StatePtr>() * cache_inner.start_states.len());
}

#[test]
fn test_reset_size_some_stack() {
    let inst_ptr: InstPtr = 1;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: vec![], num_byte_classes: 0 },
        states: vec![],
        start_states: vec![],
        stack: vec![inst_ptr; 3], // 3 entries in the stack
        flush_count: 0,
        size: 0,
    };

    cache_inner.reset_size();
    
    assert_eq!(cache_inner.size, mem::size_of::<InstPtr>() * cache_inner.stack.len());
}

#[test]
fn test_reset_size_combined_size() {
    let start_state: StatePtr = 1;
    let inst_ptr: InstPtr = 1;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: vec![], num_byte_classes: 0 },
        states: vec![],
        start_states: vec![start_state; 2], // 2 start states
        stack: vec![inst_ptr; 4],          // 4 entries in the stack
        flush_count: 0,
        size: 0,
    };

    cache_inner.reset_size();
    
    assert_eq!(cache_inner.size, 
               (mem::size_of::<StatePtr>() * cache_inner.start_states.len()) +
               (mem::size_of::<InstPtr>() * cache_inner.stack.len()));
}

