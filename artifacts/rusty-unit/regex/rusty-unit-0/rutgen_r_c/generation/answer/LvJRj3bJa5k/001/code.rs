// Answer 0

#[test]
fn test_trim_suffix_empty_literals() {
    let literals = Literals::empty();
    assert_eq!(literals.trim_suffix(1), None);
}

#[test]
fn test_trim_suffix_all_lits_small_than_num_bytes() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
            Literal::Unicode('c'),
        ],
        limit_size: 10,
        limit_class: 5,
    };
    assert_eq!(literals.trim_suffix(1), None);
}

#[test]
fn test_trim_suffix_single_lit_equal_to_num_bytes() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'), // This literal is of length 1
        ],
        limit_size: 10,
        limit_class: 5,
    };
    assert_eq!(literals.trim_suffix(1), None);
}

#[test]
fn test_trim_suffix_multiple_lits_with_min_len_equal_to_num_bytes() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'), // Length: 1
            Literal::Unicode('b'), // Length: 1
        ],
        limit_size: 10,
        limit_class: 5,
    };
    assert_eq!(literals.trim_suffix(1), None);
}

