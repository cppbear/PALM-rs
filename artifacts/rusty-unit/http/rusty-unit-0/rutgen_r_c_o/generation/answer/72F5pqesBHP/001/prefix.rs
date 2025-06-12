// Answer 0

#[test]
fn test_set_red_with_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_red();
}

#[test]
#[should_panic]
fn test_set_red_with_green() {
    let mut danger = Danger::Green;
    danger.set_red();
}

#[test]
#[should_panic]
fn test_set_red_with_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_red();
}

