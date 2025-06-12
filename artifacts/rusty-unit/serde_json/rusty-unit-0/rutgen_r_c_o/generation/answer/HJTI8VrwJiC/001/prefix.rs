// Answer 0

#[test]
fn test_byte_offset_zeroes() {
    let data = vec![Ok(0)];
    let iter = data.into_iter();
    let line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 0;
    line_col_iterator.col = 0;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_small_values() {
    let data = vec![Ok(0), Ok(1)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 5;
    line_col_iterator.col = 10;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_max_start_of_line() {
    let data = vec![Ok(0)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 1000;
    line_col_iterator.col = 255;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_col_boundary() {
    let data = vec![Ok(0)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 100;
    line_col_iterator.col = 255;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_large_col() {
    let data = vec![Ok(0)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 0;
    line_col_iterator.col = 255;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_large_start_of_line() {
    let data = vec![Ok(0)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 999;
    line_col_iterator.col = 0;
    let _ = line_col_iterator.byte_offset();
}

#[test]
fn test_byte_offset_multiple_values() {
    let data = vec![Ok(0), Ok(1), Ok(2)];
    let iter = data.into_iter();
    let mut line_col_iterator = LineColIterator::new(iter);
    line_col_iterator.start_of_line = 300;
    line_col_iterator.col = 100;
    let _ = line_col_iterator.byte_offset();
}

