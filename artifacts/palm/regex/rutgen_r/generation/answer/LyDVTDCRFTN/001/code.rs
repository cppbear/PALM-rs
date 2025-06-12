// Answer 0

#[test]
fn test_start_char() {
    struct Range {
        start: char,
        end: char,
    }

    impl Range {
        pub fn start(&self) -> char {
            self.start
        }
    }

    let range_a = Range { start: 'a', end: 'z' };
    assert_eq!(range_a.start(), 'a');

    let range_b = Range { start: '0', end: '9' };
    assert_eq!(range_b.start(), '0');

    let range_c = Range { start: '!', end: '/' };
    assert_eq!(range_c.start(), '!');

    let range_d = Range { start: 'A', end: 'Z' };
    assert_eq!(range_d.start(), 'A');

    let range_e = Range { start: '\u{10FFFF}', end: '\u{10FFFF}' };
    assert_eq!(range_e.start(), '\u{10FFFF}');
}

