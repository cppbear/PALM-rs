// Answer 0

#[derive(Debug)]
struct LineColIterator<I> {
    iter: I,
    line: usize,
    col: usize,
    start_of_line: usize,
}

pub fn new<I>(iter: I) -> LineColIterator<I> {
    LineColIterator {
        iter,
        line: 1,
        col: 0,
        start_of_line: 0,
    }
}

#[test]
fn test_new_with_empty_iterator() {
    let empty_iter = std::iter::empty::<u8>();
    let line_col_iter = new(empty_iter);
    assert_eq!(line_col_iter.line, 1);
    assert_eq!(line_col_iter.col, 0);
    assert_eq!(line_col_iter.start_of_line, 0);
}

#[test]
fn test_new_with_non_empty_iterator() {
    let vec_iter = vec![1, 2, 3].into_iter();
    let line_col_iter = new(vec_iter);
    assert_eq!(line_col_iter.line, 1);
    assert_eq!(line_col_iter.col, 0);
    assert_eq!(line_col_iter.start_of_line, 0);
}

#[test]
fn test_new_with_string_iterator() {
    let string_iter = "hello".chars();
    let line_col_iter = new(string_iter);
    assert_eq!(line_col_iter.line, 1);
    assert_eq!(line_col_iter.col, 0);
    assert_eq!(line_col_iter.start_of_line, 0);
}

