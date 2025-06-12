// Answer 0

#[test]
fn test_set_green_when_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_green();
}

#[test]
#[should_panic]
fn test_set_green_when_not_yellow() {
    let mut danger = Danger::Green;
    danger.set_green();
}

#[test]
#[should_panic]
fn test_set_green_when_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_green();
}

