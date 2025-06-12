// Answer 0

#[test]
fn test_set_yellow_with_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_yellow();
}

#[test]
fn test_set_yellow_with_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_yellow();
}

