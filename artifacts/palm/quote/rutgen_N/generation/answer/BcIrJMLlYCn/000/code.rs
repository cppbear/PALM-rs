// Answer 0

#[derive(Debug)]
struct Span {
    // Stub fields for illustration; adjust as needed.
    start: usize,
    end: usize,
}

struct MyJoiner {
    data: Vec<usize>,
}

impl MyJoiner {
    fn new(data: Vec<usize>) -> Self {
        MyJoiner { data }
    }

    fn join(&self) -> Span {
        Span {
            start: *self.data.first().unwrap_or(&0),
            end: *self.data.last().unwrap_or(&0),
        }
    }
}

impl MyJoiner {
    fn __span(&self) -> Span {
        self.join()
    }
}

#[test]
fn test_span_with_empty_data() {
    let joiner = MyJoiner::new(vec![]);
    let span = joiner.__span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

#[test]
fn test_span_with_single_element() {
    let joiner = MyJoiner::new(vec![5]);
    let span = joiner.__span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 5);
}

#[test]
fn test_span_with_multiple_elements() {
    let joiner = MyJoiner::new(vec![2, 4, 6]);
    let span = joiner.__span();
    assert_eq!(span.start, 2);
    assert_eq!(span.end, 6);
}

