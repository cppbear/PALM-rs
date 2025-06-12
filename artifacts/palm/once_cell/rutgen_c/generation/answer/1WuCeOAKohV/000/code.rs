// Answer 0

#[test]
fn test_get_or_init_initializes_value() {
    struct Dummy;

    let once_box = OnceBox::new();
    let value = once_box.get_or_init(|| Box::new(Dummy));

    assert!(!value.is_null());
}

#[test]
fn test_get_or_init_returns_same_value() {
    struct Dummy;

    let once_box = OnceBox::new();
    let first_value = once_box.get_or_init(|| Box::new(Dummy));
    let second_value = once_box.get_or_init(|| Box::new(Dummy));

    assert_eq!(first_value as *const _, second_value as *const _);
}

#[test]
fn test_get_or_init_with_multiple_initializations() {
    struct Dummy;

    let once_box = OnceBox::new();

    let first_value = once_box.get_or_init(|| Box::new(Dummy));
    let second_value = once_box.get_or_init(|| Box::new(Dummy));

    assert_eq!(first_value as *const _, second_value as *const _);
}

#[test]
fn test_get_or_init_with_different_types() {
    struct TypeA;
    struct TypeB;

    let once_box_a = OnceBox::new();
    let once_box_b = OnceBox::new();

    let value_a = once_box_a.get_or_init(|| Box::new(TypeA));
    let value_b = once_box_b.get_or_init(|| Box::new(TypeB));

    assert!(!value_a.is_null());
    assert!(!value_b.is_null());
    assert_ne!(value_a as *const _, value_b as *const _);
}

