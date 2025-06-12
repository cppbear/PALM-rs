// Answer 0

#[test]
fn test_is_yellow_true() {
    struct TestDanger {
        danger: Danger,
    }

    impl TestDanger {
        fn is_yellow(&self) -> bool {
            matches!(*self.danger, Danger::Yellow)
        }
    }

    let danger_instance = TestDanger {
        danger: Danger::Yellow,
    };
    
    assert!(danger_instance.is_yellow());
}

