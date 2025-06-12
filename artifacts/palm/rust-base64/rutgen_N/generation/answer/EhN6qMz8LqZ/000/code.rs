// Answer 0

#[test]
fn test_into_inner_with_complete_data() {
    struct MockEncoder {
        finished: bool,
        str_consumer: String,
    }

    impl MockEncoder {
        fn finish(&mut self) -> Result<&Self, &'static str> {
        self.finished = true;
        Ok(self)
    }
    }

    struct StringWriter {
        encoder: MockEncoder,
    }

    impl StringWriter {
        pub fn new() -> Self {
            StringWriter {
                encoder: MockEncoder {
                    finished: false,
                    str_consumer: String::from("SGVsbG8="), // "Hello" in base64
                },
            }
        }

        pub fn into_inner(mut self) -> String {
            self.encoder
                .finish()
                .expect("Writing to a consumer should never fail")
                .str_consumer
        }
    }

    let writer = StringWriter::new();
    let result = writer.into_inner();
    assert_eq!(result, "SGVsbG8=");
}

#[test]
fn test_into_inner_with_empty_data() {
    struct MockEncoder {
        finished: bool,
        str_consumer: String,
    }

    impl MockEncoder {
        fn finish(&mut self) -> Result<&Self, &'static str> {
            self.finished = true;
            Ok(self)
        }
    }

    struct StringWriter {
        encoder: MockEncoder,
    }

    impl StringWriter {
        pub fn new() -> Self {
            StringWriter {
                encoder: MockEncoder {
                    finished: false,
                    str_consumer: String::from(""), // empty string for base64 encoding
                },
            }
        }

        pub fn into_inner(mut self) -> String {
            self.encoder
                .finish()
                .expect("Writing to a consumer should never fail")
                .str_consumer
        }
    }

    let writer = StringWriter::new();
    let result = writer.into_inner();
    assert_eq!(result, "");
}

