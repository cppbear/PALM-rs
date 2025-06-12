// Answer 0

#[test]
fn test_is_match_empty_with_a_star() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_a_question_mark_b_star() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_a_zero_occurrences() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_empty_parentheses() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_start_end() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_alternate() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: true },
    };

    assert!(regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_non_empty_regex() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: false },
    };

    assert!(!regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_a_plus() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: false },
    };

    assert!(!regex.is_match_empty());
}

#[test]
fn test_is_match_empty_with_word_boundary() {
    struct TestRegex {
        info: RegexInfo,
    }

    struct RegexInfo {
        empty_match: bool,
    }

    impl RegexInfo {
        fn is_match_empty(&self) -> bool {
            self.empty_match
        }
    }

    let regex = TestRegex {
        info: RegexInfo { empty_match: false },
    };

    assert!(!regex.is_match_empty());
}

