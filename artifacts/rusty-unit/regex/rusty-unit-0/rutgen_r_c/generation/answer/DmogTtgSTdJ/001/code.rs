// Answer 0

#[test]
fn test_writer_finish_success() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: MockWriter };

    let result = writer.finish();

    assert_eq!(result, Ok(()));
}

