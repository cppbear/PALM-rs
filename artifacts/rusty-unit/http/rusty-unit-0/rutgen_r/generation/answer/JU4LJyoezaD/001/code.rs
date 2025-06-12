// Answer 0

#[derive(Debug, PartialEq)]
struct PathAndQuery {
    data: ByteStr,
    query: ByteStr,
}

struct ByteStr {
    content: &'static str,
}

impl ByteStr {
    fn from_static(s: &'static str) -> Self {
        ByteStr { content: s }
    }
}

const NONE: ByteStr = ByteStr { content: "" };

#[test]
fn test_star_function() {
    let result = star();
    let expected = PathAndQuery {
        data: ByteStr::from_static("*"),
        query: NONE,
    };
    assert_eq!(result, expected);
}

fn star() -> PathAndQuery {
    PathAndQuery {
        data: ByteStr::from_static("*"),
        query: NONE,
    }
}

