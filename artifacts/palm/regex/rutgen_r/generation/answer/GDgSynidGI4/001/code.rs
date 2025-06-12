// Answer 0

#[test]
fn test_end_function_valid_input() {
    struct MatchResult {
        end: usize,
    }
    
    impl MatchResult {
        pub fn end(&self) -> usize {
            self.end
        }
    }

    let match_result = MatchResult { end: 5 };
    assert_eq!(match_result.end(), 5);
}

#[test]
fn test_end_function_zero_offset() {
    struct MatchResult {
        end: usize,
    }

    impl MatchResult {
        pub fn end(&self) -> usize {
            self.end
        }
    }

    let match_result = MatchResult { end: 0 };
    assert_eq!(match_result.end(), 0);
} 

#[test]
fn test_end_function_large_offset() {
    struct MatchResult {
        end: usize,
    }

    impl MatchResult {
        pub fn end(&self) -> usize {
            self.end
        }
    }

    let match_result = MatchResult { end: usize::MAX };
    assert_eq!(match_result.end(), usize::MAX);
}

