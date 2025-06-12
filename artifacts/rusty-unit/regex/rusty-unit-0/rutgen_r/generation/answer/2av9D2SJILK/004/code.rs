// Answer 0

#[derive(Default)]
struct Flags {
    unicode: bool,
    dot_matches_new_line: bool,
}

impl Flags {
    fn unicode(&self) -> bool {
        self.unicode
    }

    fn dot_matches_new_line(&self) -> bool {
        self.dot_matches_new_line
    }
}

struct Trans {
    allow_invalid_utf8: bool,
}

impl Trans {
    fn allow_invalid_utf8(&self) -> bool {
        self.allow_invalid_utf8
    }
}

#[derive(Debug, PartialEq)]
enum Hir {
    Any(bool),
    Dot(bool),
}

struct Span;

struct TestStruct {
    flags: Flags,
    trans: Trans,
}

impl TestStruct {
    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn trans(&self) -> &Trans {
        &self.trans
    }

    fn error(&self, _span: Span, _kind: ErrorKind) -> String {
        "Error".to_string()
    }

    fn hir_dot(&self, span: Span) -> Result<Hir, String> {
        let unicode = self.flags().unicode();
        if !unicode && !self.trans().allow_invalid_utf8 {
            return Err(self.error(span, ErrorKind::InvalidUtf8));
        }
        Ok(if self.flags().dot_matches_new_line() {
            Hir::Any(!unicode)
        } else {
            Hir::Dot(!unicode)
        })
    }
}

#[derive(Debug)]
enum ErrorKind {
    InvalidUtf8,
}

#[test]
fn test_hir_dot() {
    let test_struct = TestStruct {
        flags: Flags {
            unicode: false,
            dot_matches_new_line: false,
        },
        trans: Trans {
            allow_invalid_utf8: true,
        },
    };
    let span = Span;

    let result = test_struct.hir_dot(span);
    assert_eq!(result, Ok(Hir::Dot(true)));
}

