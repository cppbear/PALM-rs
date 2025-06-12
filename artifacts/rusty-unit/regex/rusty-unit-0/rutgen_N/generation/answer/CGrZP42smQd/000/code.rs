// Answer 0

#[derive(Debug, Clone)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct ClassSetItem {
    span: Span,
}

impl ClassSetItem {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Clone)]
struct Union {
    span: Span,
    items: Vec<ClassSetItem>,
}

impl Union {
    pub fn new() -> Self {
        Self {
            span: Span { start: 0, end: 0 },
            items: Vec::new(),
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

#[test]
fn test_push_first_item_updates_span() {
    let mut union = Union::new();
    let item = ClassSetItem { span: Span { start: 5, end: 10 } };
    union.push(item);

    assert_eq!(union.span.start, 5);
    assert_eq!(union.span.end, 10);
    assert_eq!(union.items.len(), 1);
}

#[test]
fn test_push_subsequent_item_updates_span_only_end() {
    let mut union = Union::new();
    let first_item = ClassSetItem { span: Span { start: 3, end: 7 } };
    let second_item = ClassSetItem { span: Span { start: 4, end: 12 } };
    
    union.push(first_item);
    union.push(second_item);

    assert_eq!(union.span.start, 3);
    assert_eq!(union.span.end, 12);
    assert_eq!(union.items.len(), 2);
}

#[test]
fn test_push_empty_union_starts_with_item_span() {
    let mut union = Union::new();
    let item = ClassSetItem { span: Span { start: 0, end: 0 } };
    union.push(item);

    assert_eq!(union.span.start, 0);
    assert_eq!(union.span.end, 0);
    assert_eq!(union.items.len(), 1);
}

