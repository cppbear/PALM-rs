// Answer 0

#[test]
fn test_longest_common_suffix_empty() {
    struct FakeHir;
    
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };

    let result = literals.longest_common_suffix();
    
    assert_eq!(result, &[]);
}

