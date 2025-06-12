// Answer 0

#[test]
fn test_num_bytes_empty() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    let set = LiteralSet { lits: vec![] };
    assert_eq!(set.num_bytes(), 0);
}

#[test]
fn test_num_bytes_single_lit() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    let set = LiteralSet { lits: vec!["a".to_string()] };
    assert_eq!(set.num_bytes(), 1);
}

#[test]
fn test_num_bytes_multiple_lits() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    let set = LiteralSet { lits: vec!["abc".to_string(), "def".to_string(), "gh".to_string()] };
    assert_eq!(set.num_bytes(), 9);
}

#[test]
fn test_num_bytes_lits_with_special_chars() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    let set = LiteralSet { lits: vec!["!@#".to_string(), "大".to_string()] };
    assert_eq!(set.num_bytes(), 7);
}

#[test]
fn test_num_bytes_large_lits() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    let set = LiteralSet { lits: vec!["a".repeat(1000), "b".repeat(2000)] };
    assert_eq!(set.num_bytes(), 3000);
}

