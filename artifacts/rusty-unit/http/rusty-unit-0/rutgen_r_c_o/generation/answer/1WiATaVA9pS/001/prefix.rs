// Answer 0

#[test]
fn test_set_yellow_transitions_from_green() {
    let mut danger = Danger::Green;
    danger.set_yellow();
}

#[test]
#[should_panic]
fn test_set_yellow_does_not_transition_from_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_yellow(); // Should not panic because Danger::Yellow does not match the condition
}

#[test]
#[should_panic]
fn test_set_yellow_does_not_transition_from_red() {
    let mut random_state = RandomState::new();
    let mut danger = Danger::Red(random_state);
    danger.set_yellow(); // Should not panic because Danger::Red does not match the condition
}

