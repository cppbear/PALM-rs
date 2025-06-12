// Answer 0

#[test]
fn test_longest_common_suffix_empty() {
    struct TestStruct {
        lits: Vec<Vec<u8>>,
    }

    impl TestStruct {
        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }

        fn longest_common_suffix(&self) -> &[u8] {
            if self.is_empty() {
                return &[];
            }
            let lit0 = &*self.lits[0];
            let mut len = lit0.len();
            for lit in &self.lits[1..] {
                len = std::cmp::min(
                    len,
                    lit.iter()
                       .rev()
                       .zip(lit0.iter().rev())
                       .take_while(|&(a, b)| a == b)
                       .count());
            }
            &self.lits[0][self.lits[0].len() - len..]
        }
    }

    let test_struct = TestStruct { lits: Vec::new() };
    let result = test_struct.longest_common_suffix();
    assert_eq!(result, &[]);
}

