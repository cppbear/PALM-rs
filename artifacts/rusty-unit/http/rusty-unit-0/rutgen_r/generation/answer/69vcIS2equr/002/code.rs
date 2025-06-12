// Answer 0

fn set_green_test() {
    // Helper struct to use with the set_green function
    struct Danger {
        state: String,
    }

    impl Danger {
        fn is_yellow(&self) -> bool {
            self.state == "Yellow"
        }

        fn set_green(&mut self) {
            debug_assert!(self.is_yellow());
            *self = Danger {
                state: String::from("Green"),
            };
        }
    }

    // Test case where is_yellow is false
    #[should_panic]
    fn test_set_green_when_not_yellow() {
        let mut danger = Danger {
            state: String::from("Red"),
        };
        danger.set_green(); // This should trigger a panic
    }

    // Call the test
    test_set_green_when_not_yellow();
}

