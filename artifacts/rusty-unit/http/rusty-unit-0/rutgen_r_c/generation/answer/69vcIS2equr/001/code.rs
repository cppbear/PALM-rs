// Answer 0

#[test]
fn test_set_green_transitions_to_green() {
    let mut danger = Danger::Yellow;
    danger.set_green();
    match danger {
        Danger::Green => assert!(true),
        _ => assert!(false, "Expected Danger to be Green after set_green"),
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_set_green_panics_when_not_yellow() {
    let mut danger = Danger::Green;
    danger.set_green(); // This should panic because danger is not yellow
}

