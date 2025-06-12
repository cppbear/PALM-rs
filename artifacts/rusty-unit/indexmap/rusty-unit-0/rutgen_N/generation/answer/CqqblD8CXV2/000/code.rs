// Answer 0

#[test]
fn test_fmt_on_non_empty_slice() {
    struct TestSlice<'a> {
        data: &'a [i32],
    }

    impl<'a> core::fmt::Debug for TestSlice<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let slice = TestSlice { data: &[1, 2, 3] };
    let mut buffer = core::fmt::Formatter::default();
    let result = slice.fmt(&mut buffer);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_on_empty_slice() {
    struct TestSlice<'a> {
        data: &'a [i32],
    }

    impl<'a> core::fmt::Debug for TestSlice<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let slice = TestSlice { data: &[] };
    let mut buffer = core::fmt::Formatter::default();
    let result = slice.fmt(&mut buffer);
    assert!(result.is_ok());
}

