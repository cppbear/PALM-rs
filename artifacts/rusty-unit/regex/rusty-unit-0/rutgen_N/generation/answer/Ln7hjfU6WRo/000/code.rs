// Answer 0

#[test]
fn test_longest_common_prefix_non_empty() {
    struct LitSet {
        lits: Vec<Vec<u8>>,
    }
    
    impl LitSet {
        fn longest_common_prefix(&self) -> &[u8] {
            if self.lits.is_empty() {
                return &[];
            }
            let lit0 = &self.lits[0];
            let mut len = lit0.len();
            for lit in &self.lits[1..] {
                len = std::cmp::min(
                    len,
                    lit.iter()
                        .zip(lit0)
                        .take_while(|&(a, b)| a == b)
                        .count());
            }
            &self.lits[0][..len]
        }
    }

    let lit_set = LitSet {
        lits: vec![
            b"hello".to_vec(),
            b"helicopter".to_vec(),
            b"helium".to_vec(),
        ],
    };
    
    assert_eq!(lit_set.longest_common_prefix(), b"hel");
}

#[test]
fn test_longest_common_prefix_empty() {
    struct LitSet {
        lits: Vec<Vec<u8>>,
    }
    
    impl LitSet {
        fn longest_common_prefix(&self) -> &[u8] {
            if self.lits.is_empty() {
                return &[];
            }
            let lit0 = &self.lits[0];
            let mut len = lit0.len();
            for lit in &self.lits[1..] {
                len = std::cmp::min(
                    len,
                    lit.iter()
                        .zip(lit0)
                        .take_while(|&(a, b)| a == b)
                        .count());
            }
            &self.lits[0][..len]
        }
    }

    let lit_set = LitSet { lits: vec![] };

    assert_eq!(lit_set.longest_common_prefix(), b"");
}

#[test]
fn test_longest_common_prefix_full_match() {
    struct LitSet {
        lits: Vec<Vec<u8>>,
    }
    
    impl LitSet {
        fn longest_common_prefix(&self) -> &[u8] {
            if self.lits.is_empty() {
                return &[];
            }
            let lit0 = &self.lits[0];
            let mut len = lit0.len();
            for lit in &self.lits[1..] {
                len = std::cmp::min(
                    len,
                    lit.iter()
                        .zip(lit0)
                        .take_while(|&(a, b)| a == b)
                        .count());
            }
            &self.lits[0][..len]
        }
    }

    let lit_set = LitSet {
        lits: vec![
            b"match".to_vec(),
            b"match".to_vec(),
            b"match".to_vec(),
        ],
    };
    
    assert_eq!(lit_set.longest_common_prefix(), b"match");
}

#[test]
fn test_longest_common_prefix_partial_match() {
    struct LitSet {
        lits: Vec<Vec<u8>>,
    }
    
    impl LitSet {
        fn longest_common_prefix(&self) -> &[u8] {
            if self.lits.is_empty() {
                return &[];
            }
            let lit0 = &self.lits[0];
            let mut len = lit0.len();
            for lit in &self.lits[1..] {
                len = std::cmp::min(
                    len,
                    lit.iter()
                        .zip(lit0)
                        .take_while(|&(a, b)| a == b)
                        .count());
            }
            &self.lits[0][..len]
        }
    }

    let lit_set = LitSet {
        lits: vec![
            b"prefix".to_vec(),
            b"preget".to_vec(),
            b"presume".to_vec(),
        ],
    };
    
    assert_eq!(lit_set.longest_common_prefix(), b"pre");
}

