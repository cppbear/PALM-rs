// Answer 0

#[test]
fn test_set_green_transitions_from_yellow_to_green() {
    let mut danger = Danger::Yellow;
    danger.set_green();
    assert!(matches!(danger, Danger::Green));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_set_green_panics_if_not_yellow() {
    let mut danger = Danger::Green;
    danger.set_green(); // This should panic
}

#[test]
fn test_set_green_stays_green_when_called_again() {
    let mut danger = Danger::Yellow;
    danger.set_green();
    danger.set_green(); // Calling again to see if it stays green
    assert!(matches!(danger, Danger::Green));
}

