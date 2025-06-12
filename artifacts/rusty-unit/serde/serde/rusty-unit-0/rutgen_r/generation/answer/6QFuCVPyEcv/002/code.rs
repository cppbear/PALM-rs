// Answer 0

#[test]
fn test_serialize_human_readable() {
    struct MockSerializer {
        human_readable: bool,
        output: String,
    }

    impl MockSerializer {
        fn new(human_readable: bool) -> Self {
            MockSerializer {
                human_readable,
                output: String::new(),
            }
        }

        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        fn serialize_str(&mut self, value: &str) {
            self.output.push_str(value);
        }

        fn serialize_length_bounded(&mut self, value: &str, max_len: usize) -> Result<(), &'static str> {
            if value.len() > max_len {
                return Err("Length exceeds the maximum allowed");
            }
            self.serialize_str(value);
            Ok(())
        }
    }

    struct Data {
        ip: String,
        port: u16,
    }

    impl Data {
        fn ip(&self) -> &String {
            &self.ip
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

    let mut serializer = MockSerializer::new(true);
    let data = Data {
        ip: String::from("[1001:1002:1003:1004:1005:1006:1007:1008%4294967295]"),
        port: 65000,
    };

    // Triggering the scenario where left_val is not equal to right_val by using different IP and PORTs
    assert_eq!(
        serializer.serialize_length_bounded(&data.ip(), 58),
        Ok(())
    );
    
    // Testing if the result writes correctly to the output
    assert_eq!(serializer.output, "[1001:1002:1003:1004:1005:1006:1007:1008%4294967295]");
}

