// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "&&");
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter,
    };
    let op = ClassSetBinaryOpKind::Intersection;
    writer.fmt_class_set_binary_op_kind(&op).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "--");
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter,
    };
    let op = ClassSetBinaryOpKind::Difference;
    writer.fmt_class_set_binary_op_kind(&op).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "~~");
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: MockWriter,
    };
    let op = ClassSetBinaryOpKind::SymmetricDifference;
    writer.fmt_class_set_binary_op_kind(&op).unwrap();
}

