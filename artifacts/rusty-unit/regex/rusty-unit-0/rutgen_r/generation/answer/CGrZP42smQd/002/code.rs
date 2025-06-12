// Answer 0

#[test]
fn test_push_non_empty_items() {
    struct Span {
        start: usize,
        end: usize,
    }
    
    struct ClassSetItem {
        span: Span,
    }
    
    impl ClassSetItem {
        fn span(&self) -> &Span {
            &self.span
        }
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

    // Pre-populate the union with a ClassSetItem to meet the constraint.
    let initial_item = ClassSetItem {
        span: Span { start: 0, end: 5 },
    };
    
    union.push(initial_item);

    // Check the state after initial push
    assert_eq!(union.span.start, 0);
    assert_eq!(union.span.end, 5);
    assert_eq!(union.items.len(), 1);

    // Now perform the test for push on a non-empty union
    let new_item = ClassSetItem {
        span: Span { start: 5, end: 10 },
    };
    
    union.push(new_item);

    // Check that the push works correctly and the spans are updated
    assert_eq!(union.span.start, 0);  // Should remain the same
    assert_eq!(union.span.end, 10);   // Should be updated to the end of new_item
    assert_eq!(union.items.len(), 2); // Should have two items now
}

