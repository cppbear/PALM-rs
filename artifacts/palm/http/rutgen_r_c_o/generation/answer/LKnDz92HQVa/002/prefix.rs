// Answer 0

#[test]
fn test_is_red_with_red_variant() {
    let danger_red = Danger::Red(RandomState::new());
    let result = danger_red.is_red();
}

#[test]
fn test_is_red_with_yellow_variant() {
    let danger_yellow = Danger::Yellow;
    let result = danger_yellow.is_red();
}

#[test]
fn test_is_red_with_green_variant() {
    let danger_green = Danger::Green;
    let result = danger_green.is_red();
}

