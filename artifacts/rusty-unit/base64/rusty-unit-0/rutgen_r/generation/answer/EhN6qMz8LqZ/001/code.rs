// Answer 0

#[test]
fn test_into_inner_success_case() {
    struct DummyEncoder {
        completed: bool,
        str_consumer: String,
    }

    impl DummyEncoder {
        fn finish(&mut self) -> Result<&Self, &'static str> {
            if self.completed {
                Ok(self)
            } else {
                Err("Encoder not completed")
            }
        }
    }

    struct TestWriter {
        encoder: DummyEncoder,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                encoder: DummyEncoder {
                    completed: true,
                    str_consumer: String::from("encoded_data"),
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

    let writer = TestWriter::new();
    let result = writer.into_inner();
    assert_eq!(result, "encoded_data");
}

#[test]
#[should_panic(expected = "Writing to a consumer should never fail")]
fn test_into_inner_failure_case() {
    struct DummyEncoder {
        completed: bool,
        str_consumer: String,
    }

    impl DummyEncoder {
        fn finish(&mut self) -> Result<&Self, &'static str> {
            if self.completed {
                Ok(self)
            } else {
                Err("Encoder not completed")
            }
        }
    }

    struct TestWriter {
        encoder: DummyEncoder,
    }

    impl TestWriter {
        fn new(completed: bool) -> Self {
            Self {
                encoder: DummyEncoder {
                    completed,
                    str_consumer: String::from("encoded_data"),
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

    let writer = TestWriter::new(false);
    writer.into_inner(); // This should panic
}

