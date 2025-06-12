// Answer 0

#[test]
fn test_min_len_with_no_literals() {
    struct Test {
        lits: Vec<String>,
    }
    
    impl Test {
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

    let test = Test { lits: vec![] };
    assert_eq!(test.min_len(), None);
}

#[test]
fn test_min_len_with_one_literal() {
    struct Test {
        lits: Vec<String>,
    }
    
    impl Test {
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

    let test = Test { lits: vec![String::from("a")] };
    assert_eq!(test.min_len(), Some(1));
}

#[test]
fn test_min_len_with_multiple_literals() {
    struct Test {
        lits: Vec<String>,
    }
    
    impl Test {
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

    let test = Test { lits: vec![String::from("abc"), String::from("a"), String::from("abcd")] };
    assert_eq!(test.min_len(), Some(1));
}

#[test]
fn test_min_len_with_identical_length_literals() {
    struct Test {
        lits: Vec<String>,
    }
    
    impl Test {
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

    let test = Test { lits: vec![String::from("abc"), String::from("xyz"), String::from("def")] };
    assert_eq!(test.min_len(), Some(3));
}

#[test]
fn test_min_len_with_empty_string_literal() {
    struct Test {
        lits: Vec<String>,
    }
    
    impl Test {
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

    let test = Test { lits: vec![String::from("abc"), String::from(""), String::from("def")] };
    assert_eq!(test.min_len(), Some(0));
}

