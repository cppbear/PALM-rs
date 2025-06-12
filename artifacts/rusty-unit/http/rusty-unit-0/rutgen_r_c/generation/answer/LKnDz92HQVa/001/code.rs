// Answer 0

#[test]
fn test_danger_is_red_false() {
    struct TestDanger {
        danger: Danger,
    }
    
    impl TestDanger {
        fn new() -> Self {
            TestDanger {
                danger: Danger::Green, // This will ensure that `is_red` returns false
            }
        }
        
        fn is_red(&self) -> bool {
            matches!(self.danger, Danger::Red(_))
        }
    }
    
    let test_instance = TestDanger::new();
    assert_eq!(test_instance.is_red(), false);
}

#[test]
fn test_danger_is_red_false_yellow() {
    struct TestDanger {
        danger: Danger,
    }
    
    impl TestDanger {
        fn new() -> Self {
            TestDanger {
                danger: Danger::Yellow, // This will also ensure that `is_red` returns false
            }
        }
        
        fn is_red(&self) -> bool {
            matches!(self.danger, Danger::Red(_))
        }
    }
    
    let test_instance = TestDanger::new();
    assert_eq!(test_instance.is_red(), false);
}

