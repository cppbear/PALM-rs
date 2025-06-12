// Answer 0

#[test]
fn test_new_in_with_default_allocator() {
    let bump = bumpalo::Bump::new();
    let map = HashMap::new_in(&bump);
}

#[test]
fn test_new_in_with_large_allocator() {
    let bump = bumpalo::Bump::new();
    let map = HashMap::with_capacity_and_hasher_in(1000000, DefaultHashBuilder::default(), &bump);
}

#[test]
fn test_new_in_with_zero_capacity() {
    let bump = bumpalo::Bump::new();
    let map = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), &bump);
}

#[test]
fn test_new_in_with_random_state() {
    use std::collections::hash_map::RandomState;
    let bump = bumpalo::Bump::new();
    let map = HashMap::with_hasher_in(RandomState::new(), &bump);
}

#[test]
fn test_new_in_with_non_default_hash_builder() {
    struct CustomHashBuilder;
    impl Default for CustomHashBuilder {
        fn default() -> Self {
            CustomHashBuilder
        }
    }

    let bump = bumpalo::Bump::new();
    let map = HashMap::with_hasher_in(CustomHashBuilder::default(), &bump);
}

