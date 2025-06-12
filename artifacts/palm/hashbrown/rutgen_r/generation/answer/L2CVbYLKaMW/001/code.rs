// Answer 0

#[test]
fn test_new_uninitialized_valid_buckets() {
    struct Alloc;
    struct Fallibility;

    impl Alloc {
        // You can add any initializations needed for Alloc here
    }

    let buckets = 8; // 2^3, valid power of two
    let fallibility = Fallibility; // Just a placeholder

    let result = unsafe { new_uninitialized(Alloc, buckets, fallibility) };
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_new_uninitialized_invalid_buckets_non_power_of_two() {
    struct Alloc;
    struct Fallibility;

    let buckets = 7; // Not a power of two
    let fallibility = Fallibility;

    unsafe { new_uninitialized(Alloc, buckets, fallibility) };
}

#[test]
fn test_new_uninitialized_fallibility_error() {
    struct Alloc;
    struct Fallibility;

    impl Fallibility {
        // Simulate an error condition in the fallibility check
        fn fail() -> Result<(), TryReserveError> {
            Err(TryReserveError)
        }
    }

    let buckets = 4; // 2^2, valid power of two
    let fallibility = Fallibility;

    let result = unsafe { new_uninitialized(Alloc, buckets, fallibility) };
    assert!(result.is_err());
}

