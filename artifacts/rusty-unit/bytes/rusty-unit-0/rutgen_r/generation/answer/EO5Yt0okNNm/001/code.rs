// Answer 0

#[test]
fn test_from_iter_empty() {
    let result: BytesMut = from_iter(vec![]);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_from_iter_single_element() {
    let result: BytesMut = from_iter(vec![42]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 42);
}

#[test]
fn test_from_iter_multiple_elements() {
    let result: BytesMut = from_iter(vec![1, 2, 3, 4, 5]);
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], 1);
    assert_eq!(result[1], 2);
    assert_eq!(result[2], 3);
    assert_eq!(result[3], 4);
    assert_eq!(result[4], 5);
}

#[test]
fn test_from_iter_large_input() {
    let large_vec: Vec<u8> = (0..255).collect();
    let result: BytesMut = from_iter(large_vec);
    assert_eq!(result.len(), 255);
    for i in 0..255 {
        assert_eq!(result[i], i as u8);
    }
}

#[test]
#[should_panic]
fn test_from_iter_panic_on_infinite_iterator() {
    let infinite_iter = std::iter::repeat(1);
    let _result: BytesMut = from_iter(infinite_iter);
}

