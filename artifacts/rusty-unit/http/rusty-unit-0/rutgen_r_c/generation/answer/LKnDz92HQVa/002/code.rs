// Answer 0

#[test]
fn test_is_red_true() {
    struct TestDanger {
        danger: Danger,
    }

    let test_danger = TestDanger {
        danger: Danger::Red(RandomState::new()),
    };

    assert!(test_danger.danger.is_red());
}

