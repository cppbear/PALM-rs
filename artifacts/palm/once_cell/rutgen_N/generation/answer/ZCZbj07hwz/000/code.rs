// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    #[test]
    fn test_force_mut_initial_value() {
        let mut lazy = Lazy::new(|| 42);
        let result: &mut i32 = force_mut(&mut lazy);
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_force_mut_subsequent_calls() {
        let mut lazy = Lazy::new(|| 100);
        let first_result: &mut i32 = force_mut(&mut lazy);
        *first_result += 1; // Modify the value
        let second_result: &mut i32 = force_mut(&mut lazy);
        assert_eq!(*second_result, 101);
    }

    #[test]
    #[should_panic(expected = "Lazy instance has previously been poisoned")]
    fn test_force_mut_poisoned_lazy() {
        let mut lazy = Lazy::new(|| {
            panic!("This function panicked");
        });
        // Force initialization of Lazy, should panic
        let _ = force_mut(&mut lazy);
    }

    #[test]
    fn test_force_mut_multiple_initializations() {
        let mut lazy = Lazy::new(|| 10);
        let first_result: &mut i32 = force_mut(&mut lazy);
        assert_eq!(*first_result, 10);

        lazy.initialize(|| 20);
        let new_result: &mut i32 = force_mut(&mut lazy);
        assert_eq!(*new_result, 20);
    }
}

