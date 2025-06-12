// Answer 0

#[test]
#[should_panic]
fn test_danger_set_green_green() {
    let mut danger = Danger::Green;
    danger.set_green();
}

#[test]
#[should_panic]
fn test_danger_set_green_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_green();
}

