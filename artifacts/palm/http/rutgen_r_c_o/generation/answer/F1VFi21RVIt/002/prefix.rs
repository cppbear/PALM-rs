// Answer 0

#[test]
fn test_is_yellow_danger_yellow() {
    let danger = Danger::Yellow;
    let result = danger.is_yellow();
}

#[test]
fn test_is_yellow_danger_green() {
    let mut danger = Danger::Green;
    let result = danger.is_yellow();
}

#[test]
fn test_is_yellow_danger_red() {
    let danger = Danger::Red(RandomState::new());
    let result = danger.is_yellow();
}

