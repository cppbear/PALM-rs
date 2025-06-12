// Answer 0

#[test]
fn test_push_empty_union() {
    struct ClassSetItem {
        start: usize,
        end: usize,
    }

    impl ClassSetItem {
        fn span(&self) -> Span {
            Span { start: self.start, end: self.end }
        }
    }

    struct Span {
        start: usize,
        end: usize,
    }

    struct Union {
        items: Vec<ClassSetItem>,
        span: Span,
    }

    impl Union {
        fn new() -> Self {
            Union {
                items: Vec::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        pub fn push(&mut self, item: ClassSetItem) {
            if self.items.is_empty() {
                self.span.start = item.span().start;
            }
            self.span.end = item.span().end;
            self.items.push(item);
        }
    }

    let mut union = Union::new();
    let item = ClassSetItem { start: 5, end: 10 };

    union.push(item);

    assert_eq!(union.span.start, 5);
    assert_eq!(union.span.end, 10);
    assert_eq!(union.items.len(), 1);
}

