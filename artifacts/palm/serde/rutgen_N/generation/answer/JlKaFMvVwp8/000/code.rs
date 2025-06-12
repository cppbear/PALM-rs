// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(_: Unsupported) -> ! {
        panic!("Unsupported type")
    }
}

struct Unsupported;

#[test]
#[should_panic(expected = "Unsupported type")]
fn test_serialize_u64() {
    let serializer = TestSerializer;
    let result: Result<_, _> = serializer.serialize_u64(42);
    result.unwrap(); // This will cause a panic since the function always returns an Err.
}

