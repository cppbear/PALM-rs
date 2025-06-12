// Answer 0

#[test]
fn test_longest_common_prefix_empty_set() {
    struct StringSet {
        lits: Vec<Vec<u8>>,
    }

    impl StringSet {
        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }

        fn longest_common_prefix(&self) -> &[u8] {
            if self.is_empty() {
                return &[];
            }
            let lit0 = &*self.lits[0];
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

    let string_set = StringSet { lits: vec![] };
    let result = string_set.longest_common_prefix();
    assert_eq!(result, &[]);
}

