// Answer 0

#[test]
fn test_is_yellow_with_green_danger() {
    let danger = Danger::Green;
    assert!(!danger.is_yellow());
}

#[test]
fn test_is_yellow_with_red_danger() {
    let danger = Danger::Red(RandomState::new());
    assert!(!danger.is_yellow());
}

