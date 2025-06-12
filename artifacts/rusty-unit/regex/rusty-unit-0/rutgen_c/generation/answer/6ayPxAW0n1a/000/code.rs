// Answer 0

#[test]
fn test_inst_ptrs() {
    let state = State {
        data: Box::from(vec![0u8, 1, 2, 3, 4]),
    };
    
    let inst_ptrs = state.inst_ptrs();
    
    assert_eq!(inst_ptrs.base, 0);
    assert_eq!(inst_ptrs.data, &[1, 2, 3, 4]);
}

#[test]
fn test_inst_ptrs_empty_data() {
    let state = State {
        data: Box::from(vec![0u8]),
    };

    let inst_ptrs = state.inst_ptrs();
    
    assert_eq!(inst_ptrs.base, 0);
    assert!(inst_ptrs.data.is_empty());
}

