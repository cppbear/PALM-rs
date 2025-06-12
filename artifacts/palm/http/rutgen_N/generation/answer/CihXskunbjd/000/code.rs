// Answer 0

#[derive(Debug)]
struct Scheme2 {
    kind: SchemeKind,
}

#[derive(Debug)]
enum SchemeKind {
    None,
}

struct Scheme {
    inner: Scheme2,
}

impl Scheme {
    pub(super) fn empty() -> Self {
        Scheme {
            inner: Scheme2 { kind: SchemeKind::None },
        }
    }
}

#[test]
fn test_empty_scheme() {
    let scheme = Scheme::empty();
    assert_eq!(format!("{:?}", scheme.inner.kind), "None");
}

