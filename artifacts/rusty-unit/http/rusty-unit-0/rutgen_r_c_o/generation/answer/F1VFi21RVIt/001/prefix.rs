// Answer 0

#[test]
fn test_is_yellow_green() {
    let danger = Danger::Green;
    danger.is_yellow();
}

#[test]
fn test_is_yellow_red() {
    let danger = Danger::Red(RandomState::new());
    danger.is_yellow();
}

