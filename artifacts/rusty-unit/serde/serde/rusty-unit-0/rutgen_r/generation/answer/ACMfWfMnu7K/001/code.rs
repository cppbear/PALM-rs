// Answer 0

#[test]
fn test_new_unit_deserializer() {
    use std::marker::PhantomData;

    struct UnitDeserializer<T> {
        marker: PhantomData<T>,
    }

    impl<T> UnitDeserializer<T> {
        pub fn new() -> Self {
            UnitDeserializer {
                marker: PhantomData,
            }
        }
    }

    // Test the creation of the UnitDeserializer
    let deserializer: UnitDeserializer<()> = UnitDeserializer::new();
    let expected_marker = PhantomData::<()>;

    assert_eq!(std::mem::size_of_val(&deserializer.marker), std::mem::size_of_val(&expected_marker));
}

