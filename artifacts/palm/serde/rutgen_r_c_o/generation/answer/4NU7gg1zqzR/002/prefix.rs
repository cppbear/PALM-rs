// Answer 0

#[test]
fn test_iterator_len_hint_zero() {
    let vec = vec![];
    let iter = vec.iter();
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_one() {
    let vec = vec![1];
    let iter = vec.iter();
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_two() {
    let vec = vec![1, 2];
    let iter = vec.iter();
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_large() {
    let vec = (0..100).collect::<Vec<_>>();
    let iter = vec.iter();
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_empty_range() {
    let iter = (0..0).into_iter();
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_infinite() {
    let iter = (0..).take(10);
    iterator_len_hint(&iter);
}

