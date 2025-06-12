// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    #[test]
    fn test_force_initialization() {
        let lazy = Lazy::new(|| 42);
        let value = force(&lazy);
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_force_multiple_forces() {
        let lazy = Lazy::new(|| 100);
        let value1 = force(&lazy);
        let value2 = force(&lazy);
        assert_eq!(*value1, 100);
        assert_eq!(value1 as *const _ == value2 as *const _, true); // Ensure it returns the same reference
    }

    #[should_panic(expected = "Lazy instance has previously been poisoned")]
    #[test]
    fn test_force_poisoned() {
        struct PoisonedLazy;

        impl PoisonedLazy {
            const fn new() -> Self {
                PoisonedLazy {}
            }
        }

        let lazy = Lazy::new(|| panic!("Poisoned"));
        force(&lazy);
    }
}

