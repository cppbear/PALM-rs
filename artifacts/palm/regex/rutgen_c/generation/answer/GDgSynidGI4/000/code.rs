// Answer 0

#[test]
fn test_match_end() {
    let match_instance = Match {
        text: "hello",
        start: 0,
        end: 5,
    };
    assert_eq!(match_instance.end(), 5);
}

#[test]
fn test_match_end_zero() {
    let match_instance = Match {
        text: "hello",
        start: 0,
        end: 0,
    };
    assert_eq!(match_instance.end(), 0);
}

#[test]
fn test_match_end_boundary() {
    let match_instance = Match {
        text: "",
        start: 0,
        end: 0,
    };
    assert_eq!(match_instance.end(), 0);
}

