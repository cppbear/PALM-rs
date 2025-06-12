// Answer 0

#[test]
fn test_prefix_at_with_valid_input() {
    struct Prog {
        prefixes: Vec<(usize, usize)>,
    }

    impl Prog {
        fn new() -> Self {
            Self { prefixes: vec![(4, 1), (3, 2)] }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prefixes.iter().find(|&&(s, _)| at + s <= text.len()).map(|&(s, _)| at + s)
        }
    }

    let prog = Prog::new();
    let text = b"abcdefg";
    
    assert_eq!(prog.prefix_at(text, 0), Some(4));
    assert_eq!(prog.prefix_at(text, 1), Some(4));
    assert_eq!(prog.prefix_at(text, 2), Some(4));
    assert_eq!(prog.prefix_at(text, 3), Some(4));
    assert_eq!(prog.prefix_at(text, 4), Some(4));
}

#[test]
#[should_panic]
fn test_prefix_at_panic_out_of_bounds() {
    struct Prog {
        prefixes: Vec<(usize, usize)>,
    }

    impl Prog {
        fn new() -> Self {
            Self { prefixes: vec![(4, 1)] }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prefixes.iter().find(|&&(s, _)| at + s <= text.len()).map(|&(s, _)| at + s)
        }
    }

    let prog = Prog::new();
    let text = b"abc";
    
    // This will panic since at + s (4 + 0) exceeds text length
    prog.prefix_at(text, 4);
}

#[test]
fn test_prefix_at_with_no_found_prefixes() {
    struct Prog {
        prefixes: Vec<(usize, usize)>,
    }

    impl Prog {
        fn new() -> Self {
            Self { prefixes: vec![] }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prefixes.iter().find(|&&(s, _)| at + s <= text.len()).map(|&(s, _)| at + s)
        }
    }

    let prog = Prog::new();
    let text = b"abcdefg";
    
    // Expect None since there are no prefixes
    assert_eq!(prog.prefix_at(text, 0), None);
    assert_eq!(prog.prefix_at(text, 3), None);
}

