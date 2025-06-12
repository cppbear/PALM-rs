// Answer 0

#[test]
fn test_min_len_empty_literals() {
    struct TestStruct {
        lits: Vec<String>,
    }

    impl TestStruct {
        pub fn min_len(&self) -> Option<usize> {
            let mut min = None;
            for lit in &self.lits {
                match min {
                    None => min = Some(lit.len()),
                    Some(m) if lit.len() < m => min = Some(lit.len()),
                    _ => {}
                }
            }
            min
        }
    }

    let test_case = TestStruct { lits: Vec::new() };
    assert_eq!(test_case.min_len(), None);
}

#[test]
fn test_min_len_single_literal() {
    struct TestStruct {
        lits: Vec<String>,
    }

    impl TestStruct {
        pub fn min_len(&self) -> Option<usize> {
            let mut min = None;
            for lit in &self.lits {
                match min {
                    None => min = Some(lit.len()),
                    Some(m) if lit.len() < m => min = Some(lit.len()),
                    _ => {}
                }
            }
            min
        }
    }

    let test_case = TestStruct { lits: vec!["a".to_string()] };
    assert_eq!(test_case.min_len(), Some(1));
}

#[test]
fn test_min_len_multiple_literals() {
    struct TestStruct {
        lits: Vec<String>,
    }

    impl TestStruct {
        pub fn min_len(&self) -> Option<usize> {
            let mut min = None;
            for lit in &self.lits {
                match min {
                    None => min = Some(lit.len()),
                    Some(m) if lit.len() < m => min = Some(lit.len()),
                    _ => {}
                }
            }
            min
        }
    }

    let test_case = TestStruct { lits: vec!["abc".to_string(), "a".to_string(), "ab".to_string()] };
    assert_eq!(test_case.min_len(), Some(1));
}

#[test]
fn test_min_len_identical_literals() {
    struct TestStruct {
        lits: Vec<String>,
    }

    impl TestStruct {
        pub fn min_len(&self) -> Option<usize> {
            let mut min = None;
            for lit in &self.lits {
                match min {
                    None => min = Some(lit.len()),
                    Some(m) if lit.len() < m => min = Some(lit.len()),
                    _ => {}
                }
            }
            min
        }
    }

    let test_case = TestStruct { lits: vec!["abc".to_string(), "abc".to_string()] };
    assert_eq!(test_case.min_len(), Some(3));
}

#[test]
fn test_min_len_edge_case_empty_lits() {
    struct TestStruct {
        lits: Vec<String>,
    }

    impl TestStruct {
        pub fn min_len(&self) -> Option<usize> {
            let mut min = None;
            for lit in &self.lits {
                match min {
                    None => min = Some(lit.len()),
                    Some(m) if lit.len() < m => min = Some(lit.len()),
                    _ => {}
                }
            }
            min
        }
    }

    let test_case = TestStruct { lits: vec!["".to_string()] };
    assert_eq!(test_case.min_len(), Some(0));
}

