// Answer 0

#[test]
fn test_set_yellow_changes_green_to_yellow() {
    let mut danger_state = Danger::Green;

    danger_state.set_yellow();

    match danger_state {
        Danger::Yellow => (),
        _ => panic!("set_yellow did not change Danger::Green to Danger::Yellow"),
    }
}

#[test]
fn test_set_yellow_does_not_change_if_already_yellow() {
    let mut danger_state = Danger::Yellow;

    danger_state.set_yellow();

    match danger_state {
        Danger::Yellow => (),
        _ => panic!("set_yellow incorrectly changed Danger::Yellow"),
    }
}

#[test]
fn test_set_yellow_does_not_change_if_already_red() {
    let random_state = RandomState::new();
    let mut danger_state = Danger::Red(random_state);

    danger_state.set_yellow();

    match danger_state {
        Danger::Red(_) => (),
        _ => panic!("set_yellow incorrectly changed Danger::Red"),
    }
}

