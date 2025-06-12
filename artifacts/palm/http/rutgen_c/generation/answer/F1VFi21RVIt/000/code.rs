// Answer 0

#[test]
fn test_danger_is_yellow() {
    let danger_green = Danger::Green;
    let danger_yellow = Danger::Yellow;
    let danger_red = Danger::Red(RandomState::new());

    assert!(!danger_green.is_yellow());
    assert!(danger_yellow.is_yellow());
    assert!(!danger_red.is_yellow());
}

#[test]
fn test_danger_is_yellow_boundary() {
    let danger_yellow = Danger::Yellow;
    assert!(danger_yellow.is_yellow());
}

