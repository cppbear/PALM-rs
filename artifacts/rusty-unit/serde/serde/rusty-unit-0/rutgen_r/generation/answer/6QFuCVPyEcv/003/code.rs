// Answer 0

#[test]
fn test_serialize_non_human_readable() {
    use serde::ser::{Serializer, Serialize};

    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = (u32, u16);
        type Error = ();
        fn is_human_readable(&self) -> bool {
            false
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::Ok, Self::Error> {
            Ok((1001, 65000)) // simulate a successful serialize
        }
        // Required unimplemented methods can be omitted for brevity
    }

    struct TestStruct {
        ip: u32,
        port: u16,
    }

    impl TestStruct {
        fn ip(&self) -> u32 {
            self.ip
        }
        fn port(&self) -> u16 {
            self.port
        }

        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                const MAX_LEN: usize = 58;
                serialize_display_bounded_length!(self, MAX_LEN, serializer)
            } else {
                (self.ip(), self.port()).serialize(serializer)
            }
        }
    }

    let test_struct = TestStruct { ip: 1001, port: 65000 };
    let mock_serializer = MockSerializer;

    let result = test_struct.serialize(mock_serializer);
    assert_eq!(result, Ok((1001, 65000)));
}

