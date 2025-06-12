// Answer 0

#[test]
fn test_set_red_when_not_yellow_green() {
    let mut danger = Danger::Green;
    danger.set_red();
}

#[test]
fn test_set_red_when_not_yellow_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_red();
}

