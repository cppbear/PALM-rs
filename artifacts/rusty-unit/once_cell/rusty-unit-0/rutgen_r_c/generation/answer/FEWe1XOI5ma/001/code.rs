// Answer 0

#[test]
fn test_from_usize_with_1() {
    let value = NonZeroUsize::new(1).unwrap();
    assert_eq!(OnceBool::from_usize(value), true);
}

#[test]
fn test_from_usize_with_2() {
    let value = NonZeroUsize::new(2).unwrap();
    assert_eq!(OnceBool::from_usize(value), false);
}

#[test]
fn test_from_usize_with_3() {
    let value = NonZeroUsize::new(3).unwrap();
    assert_eq!(OnceBool::from_usize(value), false);
}

#[test]
fn test_from_usize_with_max_usize() {
    let value = NonZeroUsize::new(usize::MAX).unwrap();
    assert_eq!(OnceBool::from_usize(value), false);
}

