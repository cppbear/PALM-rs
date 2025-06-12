// Answer 0

#[derive(Debug)]
struct Pos {
    index: u64,
    hash: HashValue,
}

#[derive(Debug)]
struct HashValue(u64);

impl Pos {
    fn none() -> Self {
        Pos {
            index: !0,
            hash: HashValue(0),
        }
    }
}

#[test]
fn test_none_function() {
    let pos = Pos::none();
    assert_eq!(pos.index, !0);
    assert_eq!(pos.hash.0, 0);
}

