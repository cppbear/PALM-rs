// Answer 0

#[test]
fn test_danger_is_red_when_red() {
    let danger = Danger::Red(RandomState::new());
    assert!(danger.is_red());
}

#[test]
fn test_danger_is_red_when_yellow() {
    let danger = Danger::Yellow;
    assert!(!danger.is_red());
}

#[test]
fn test_danger_is_red_when_green() {
    let danger = Danger::Green;
    assert!(!danger.is_red());
}

