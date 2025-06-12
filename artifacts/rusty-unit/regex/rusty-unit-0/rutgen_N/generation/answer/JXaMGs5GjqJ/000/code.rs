// Answer 0

#[test]
fn test_shortest_nfa_found() {
    struct NfaTester;

    impl NfaTester {
        fn shortest_nfa_type(&self, _: MatchNfaType, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
        
        fn shortest_nfa(&self, text: &[u8], start: usize) -> Option<usize> {
            self.shortest_nfa_type(MatchNfaType::Auto, text, start)
        }
    }

    let tester = NfaTester;
    let text = b"abc";
    
    assert_eq!(tester.shortest_nfa(text, 0), Some(0));
}

#[test]
fn test_shortest_nfa_not_found() {
    struct NfaTester;

    impl NfaTester {
        fn shortest_nfa_type(&self, _: MatchNfaType, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
        
        fn shortest_nfa(&self, text: &[u8], start: usize) -> Option<usize> {
            self.shortest_nfa_type(MatchNfaType::Auto, text, start)
        }
    }

    let tester = NfaTester;
    let text = b"xyz";
    
    assert_eq!(tester.shortest_nfa(text, 0), None);
}

#[test]
fn test_shortest_nfa_edge_case() {
    struct NfaTester;

    impl NfaTester {
        fn shortest_nfa_type(&self, _: MatchNfaType, text: &[u8], start: usize) -> Option<usize> {
            if start < text.len() && text[start] == b'a' {
                return Some(start);
            }
            None
        }
        
        fn shortest_nfa(&self, text: &[u8], start: usize) -> Option<usize> {
            self.shortest_nfa_type(MatchNfaType::Auto, text, start)
        }
    }

    let tester = NfaTester;
    let text = b"a";
    
    assert_eq!(tester.shortest_nfa(text, 0), Some(0));
    assert_eq!(tester.shortest_nfa(text, 1), None);
}

