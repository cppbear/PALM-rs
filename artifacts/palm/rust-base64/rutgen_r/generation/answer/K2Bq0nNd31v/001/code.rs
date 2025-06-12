// Answer 0

#[test]
fn test_encoder_string_writer_from_consumer() {
    struct TestStrConsumer {
        data: String,
    }

    impl TestStrConsumer {
        fn new() -> Self {
            TestStrConsumer {
                data: String::new(),
            }
        }

        fn consume(&mut self, s: &str) {
            self.data.push_str(s);
        }
    }

    struct TestEngine;

    let mut consumer = TestStrConsumer::new();
    let engine = &TestEngine;

    let writer = from_consumer(consumer, engine);

    // Verify the structure is created successfully
    assert!(writer.encoder.is_some());

    // Additional test to check the state of the consumer after using writer
    writer.encoder.write("example").unwrap();
    assert_eq!(consumer.data, "example");
}

#[test]
#[should_panic]
fn test_encoder_string_writer_from_consumer_panic() {
    struct PanicConsumer;

    impl PanicConsumer {
        fn new() -> Self {
            PanicConsumer
        }
    }

    struct PanicEngine;

    let consumer = PanicConsumer::new();
    let engine = &PanicEngine;

    // This test should panic if constructing the EncoderStringWriter fails
    let _writer = from_consumer(consumer, engine);
}

