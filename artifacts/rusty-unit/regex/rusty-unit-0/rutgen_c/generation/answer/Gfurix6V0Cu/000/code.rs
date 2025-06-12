// Answer 0

#[test]
fn test_deref_mut() {
    let mut literal = Literal { v: vec![1, 2, 3], cut: false };
    
    let vec_ref: &mut Vec<u8> = literal.deref_mut();
    vec_ref.push(4);
    
    assert_eq!(literal.v, vec![1, 2, 3, 4]);
}

#[test]
fn test_deref_mut_empty_vector() {
    let mut literal = Literal { v: vec![], cut: false };
    
    let vec_ref: &mut Vec<u8> = literal.deref_mut();
    vec_ref.push(5);
    
    assert_eq!(literal.v, vec![5]);
}

#[test]
fn test_deref_mut_change_cut() {
    let mut literal = Literal { v: vec![10, 20], cut: true };

    let vec_ref: &mut Vec<u8> = literal.deref_mut();
    vec_ref.push(30);

    assert_eq!(literal.v, vec![10, 20, 30]);
    assert!(literal.cut);
}

