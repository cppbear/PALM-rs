// Answer 0

#[test]
fn test_set_yellow_transitions_from_green() {
    let mut danger = Danger::Green;
    danger.set_yellow();
    match danger {
        Danger::Yellow => {},
        _ => panic!("Expected Danger to be Yellow"),
    }
}

#[test]
fn test_set_yellow_no_transition_from_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_yellow();
    match danger {
        Danger::Yellow => {},
        _ => panic!("Expected Danger to remain Yellow"),
    }
}

#[test]
fn test_set_yellow_no_transition_from_red() {
    let mut rng = RandomState::new();
    let mut danger = Danger::Red(rng);
    danger.set_yellow();
    match danger {
        Danger::Red(_) => {},
        _ => panic!("Expected Danger to remain Red"),
    }
}

