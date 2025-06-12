// Answer 0

#[test]
fn test_swap_greed_with_some_true() {
    let flags = Flags {
        swap_greed: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.swap_greed(), true);
}

#[test]
fn test_swap_greed_with_some_false() {
    let flags = Flags {
        swap_greed: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.swap_greed(), false);
}

#[test]
fn test_swap_greed_with_none() {
    let flags = Flags {
        swap_greed: None,
        ..Flags::default()
    };
    assert_eq!(flags.swap_greed(), false);
}

