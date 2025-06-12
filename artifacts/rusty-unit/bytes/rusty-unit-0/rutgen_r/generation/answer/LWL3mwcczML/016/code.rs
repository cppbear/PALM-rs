// Answer 0

fn test_fmt_with_newline_character() {
    struct TestStruct<'a> {
        data: &'a [u8],
    }

    impl std::fmt::Debug for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.data {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            Ok(())
        }
    }

    let data = TestStruct { data: &[1, 2, b'\n', 4] };
    let mut output = Vec::new();
    let result = std::fmt::Debug::fmt(&data, &mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

fn test_fmt_with_carriage_return() {
    struct TestStruct<'a> {
        data: &'a [u8],
    }

    impl std::fmt::Debug for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.data {
                if b == b'\r' {
                    write!(f, "\\r")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            Ok(())
        }
    }

    let data = TestStruct { data: &[1, 2, b'\r', 4] };
    let mut output = Vec::new();
    let result = std::fmt::Debug::fmt(&data, &mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

fn test_fmt_with_tab_character() {
    struct TestStruct<'a> {
        data: &'a [u8],
    }

    impl std::fmt::Debug for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.data {
                if b == b'\t' {
                    write!(f, "\\t")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            Ok(())
        }
    }

    let data = TestStruct { data: &[1, 2, b'\t', 4] };
    let mut output = Vec::new();
    let result = std::fmt::Debug::fmt(&data, &mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

fn test_fmt_with_escape_character() {
    struct TestStruct<'a> {
        data: &'a [u8],
    }

    impl std::fmt::Debug for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.data {
                if b == b'\\' {
                    write!(f, "\\\\")?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            Ok(())
        }
    }

    let data = TestStruct { data: &[1, 2, b'\\', 4] };
    let mut output = Vec::new();
    let result = std::fmt::Debug::fmt(&data, &mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

fn test_fmt_with_non_printable_character() {
    struct TestStruct<'a> {
        data: &'a [u8],
    }

    impl std::fmt::Debug for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.data {
                if b < 0x20 || b >= 0x7f {
                    write!(f, "\\x{:02x}", b)?;
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            Ok(())
        }
    }

    let data = TestStruct { data: &[0, 1, 2, 0x7f] };
    let mut output = Vec::new();
    let result = std::fmt::Debug::fmt(&data, &mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

