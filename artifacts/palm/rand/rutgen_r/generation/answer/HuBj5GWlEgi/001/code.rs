// Answer 0

#[test]
fn test_num_choices_non_zero() {
    use std::num::NonZeroUsize;

    struct Distribution {
        num_choices: NonZeroUsize,
    }

    let valid_distribution = Distribution {
        num_choices: NonZeroUsize::new(5).unwrap(),
    };

    assert_eq!(valid_distribution.num_choices(), NonZeroUsize::new(5).unwrap());
}

#[test]
#[should_panic]
fn test_num_choices_zero() {
    use std::num::NonZeroUsize;

    struct Distribution {
        num_choices: NonZeroUsize,
    }

    let invalid_distribution = Distribution {
        num_choices: NonZeroUsize::new(0).unwrap(), // This will panic, as NonZeroUsize cannot be zero.
    };

    // Attempting to use num_choices will panic.
    let _ = invalid_distribution.num_choices();
}

