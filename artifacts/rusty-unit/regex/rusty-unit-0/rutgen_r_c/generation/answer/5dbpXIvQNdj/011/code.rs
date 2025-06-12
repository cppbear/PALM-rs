// Answer 0

#[test]
fn test_cross_add_non_empty_lits_max_limit() {
    let mut literals = {
        let mut lits = Literals {
            lits: vec![Literal::new(vec![b'a']), Literal::new(vec![b'b'])],
            limit_size: 4,
            limit_class: 1,
        };
        lits.set_limit_size(4);
        lits
    };
    let bytes = b"cd"; // bytes.len() == 2

    let result = literals.cross_add(bytes);
    
    assert!(result);
    assert_eq!(literals.lits.len(), 2);
    assert_eq!(literals.lits[0].is_cut(), true);
    assert_eq!(literals.lits[0].v, vec![b'a', b'c']);
    assert_eq!(literals.lits[1].is_cut(), true);
    assert_eq!(literals.lits[1].v, vec![b'b', b'd']);
}

#[test]
fn test_cross_add_increasing_size() {
    let mut literals = {
        let mut lits = Literals {
            lits: vec![Literal::new(vec![b'x'])],
            limit_size: 6,
            limit_class: 1,
        };
        lits.set_limit_size(6);
        lits
    };
    let bytes = b"yz"; // bytes.len() == 2

    let result = literals.cross_add(bytes);

    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].is_cut(), true);
    assert_eq!(literals.lits[0].v, vec![b'x', b'y', b'z']);
}

#[test]
fn test_cross_add_special_case_exact_limit() {
    let mut literals = {
        let mut lits = Literals {
            lits: vec![Literal::new(vec![b'1'])],
            limit_size: 3, // 3 is the limit
            limit_class: 1,
        };
        lits.set_limit_size(3);
        lits
    };
    let bytes = b"2"; // Adding a single byte, reaching the limit exactly

    let result = literals.cross_add(bytes);
    
    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].is_cut(), true);
    assert_eq!(literals.lits[0].v, vec![b'1', b'2']);
}

