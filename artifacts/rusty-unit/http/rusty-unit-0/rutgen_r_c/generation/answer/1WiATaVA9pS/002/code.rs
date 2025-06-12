// Answer 0

#[test]
fn test_set_yellow_already_yellow() {
    let mut danger = Danger::Yellow;
    danger.set_yellow();
    if let Danger::Yellow = danger {
        assert!(true);
    } else {
        assert!(false, "Expected Danger::Yellow");
    }
}

#[test]
fn test_set_yellow_already_red() {
    let mut danger = Danger::Red(RandomState::new());
    danger.set_yellow();
    if let Danger::Red(_) = danger {
        assert!(true);
    } else {
        assert!(false, "Expected Danger::Red");
    }
}

#[test]
fn test_set_yellow_transitions_from_green() {
    let mut danger = Danger::Green;
    danger.set_yellow();
    if let Danger::Yellow = danger {
        assert!(true);
    } else {
        assert!(false, "Expected Danger::Yellow after transition from Green");
    }
}

