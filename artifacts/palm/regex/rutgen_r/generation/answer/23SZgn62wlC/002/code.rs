// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_no_match() {
    struct TestStruct {
        ro: TestRo,
        cache: TestCache,
    }

    struct TestRo {
        dfa_reverse: TestDfaReverse,
        suffixes: TestSuffixes,
    }

    struct TestDfaReverse;

    struct TestSuffixes {
        lcs_length: usize,
    }

    impl TestSuffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![0] // lcs.len() == 1
        }
    }

    struct TestCache;

    impl TestStruct {
        fn exec_dfa_reverse_suffix(
            &self,
            text: &[u8],
            original_start: usize,
        ) -> Option<dfa::Result<(usize, usize)>> {
            use dfa::Result::*;

            let lcs = self.ro.suffixes.lcs();
            debug_assert!(lcs.len() >= 1);
            let mut start = original_start;
            let mut end = start;
            while end <= text.len() {
                start = end;
                end += match lcs.iter().position(|&x| x == text[end]) {
                    None => return Some(NoMatch(text.len())),
                    Some(index) => index + lcs.len(),
                };
                match dfa::Fsm::reverse(
                    &self.ro.dfa_reverse,
                    self.cache,
                    false,
                    &text[start..end],
                    end - start,
                ) {
                    Match(0) | NoMatch(0) => return None,
                    Match(s) => return Some(Match((s + start, end))),
                    NoMatch(_) => continue,
                    Quit => return Some(Quit),
                };
            }
            Some(NoMatch(text.len()))
        }
    }

    let test_struct = TestStruct {
        ro: TestRo {
            dfa_reverse: TestDfaReverse,
            suffixes: TestSuffixes { lcs_length: 1 },
        },
        cache: TestCache,
    };

    let text: &[u8] = b"example";
    let original_start = 7; // end will be equal to text.len(), which should lead to NoMatch.
    let result = test_struct.exec_dfa_reverse_suffix(text, original_start);
    assert_eq!(result, Some(dfa::Result::NoMatch(text.len())));
}

