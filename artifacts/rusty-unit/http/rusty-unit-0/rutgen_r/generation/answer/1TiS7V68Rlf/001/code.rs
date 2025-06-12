// Answer 0

#[derive(Debug, PartialEq)]
struct HashValue(u64);

#[derive(Debug, PartialEq)]
struct Pos {
    index: u64,
    hash: HashValue,
}

impl Pos {
    fn none() -> Self {
        Pos {
            index: !0,
            hash: HashValue(0),
        }
    }
}

#[test]
fn test_none() {
    let expected = Pos {
        index: !0,
        hash: HashValue(0),
    };
    let result = Pos::none();
    assert_eq!(result, expected);
}

