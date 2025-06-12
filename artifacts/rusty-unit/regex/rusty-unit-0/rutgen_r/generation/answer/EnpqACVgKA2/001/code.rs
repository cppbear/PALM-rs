// Answer 0

#[test]
fn test_should_suffix_scan_empty_suffixes() {
    struct Suffixes {
        suffixes: Vec<String>,
    }

    impl Suffixes {
        fn is_empty(&self) -> bool {
            self.suffixes.is_empty()
        }
        
        fn lcs(&self) -> LCS {
            LCS { length: 0 }
        }
    }

    struct LCS {
        length: usize,
    }

    impl LCS {
        fn char_len(&self) -> usize {
            self.length
        }
    }

    struct DFA {
        prefixes: Prefixes,
    }

    struct Prefixes;

    impl Prefixes {
        fn lcp(&self) -> LCP {
            LCP { length: 0 }
        }
    }

    struct LCP {
        length: usize,
    }

    impl LCP {
        fn char_len(&self) -> usize {
            self.length
        }
    }

    struct Context {
        suffixes: Suffixes,
        dfa: DFA,
    }

    impl Context {
        fn should_suffix_scan(&self) -> bool {
            if self.suffixes.is_empty() {
                return false;
            }
            let lcs_len = self.suffixes.lcs().char_len();
            lcs_len >= 3 && lcs_len > self.dfa.prefixes.lcp().char_len()
        }
    }

    let context = Context {
        suffixes: Suffixes { suffixes: vec![] }, // self.suffixes.is_empty() is true
        dfa: DFA { prefixes: Prefixes {} },
    };

    assert_eq!(context.should_suffix_scan(), false);
}

