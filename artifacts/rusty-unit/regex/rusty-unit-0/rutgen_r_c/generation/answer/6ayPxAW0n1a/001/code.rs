// Answer 0

#[test]
fn test_inst_ptrs_non_empty_data() {
    let state = State {
        data: Box::from([0u8, 1, 2, 3]),
    };
    
    let inst_ptrs = state.inst_ptrs();
    
    assert_eq!(inst_ptrs.base, 0);
    assert_eq!(inst_ptrs.data, &[1, 2, 3]);
}

#[test]
#[should_panic]
fn test_inst_ptrs_empty_data() {
    let state = State {
        data: Box::from([0u8]),
    };
    
    let _ = state.inst_ptrs(); // This should panic due to self.data[1..]
}

