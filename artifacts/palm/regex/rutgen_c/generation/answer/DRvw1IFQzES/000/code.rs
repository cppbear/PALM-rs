// Answer 0

#[test]
fn test_match_start() {
    let test_haystack = "hello, world";
    let test_start = 7;
    let test_end = 12;
    let m = Match {
        text: test_haystack,
        start: test_start,
        end: test_end,
    };

    assert_eq!(m.start(), test_start);
}

#[test]
fn test_match_start_boundary() {
    let test_haystack = "boundary test";
    let test_start = 0;
    let test_end = 8;
    let m = Match {
        text: test_haystack,
        start: test_start,
        end: test_end,
    };

    assert_eq!(m.start(), test_start);
}

