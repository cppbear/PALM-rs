// Answer 0

#[test]
fn test_format_u8() {
    struct Buffer {
        bytes: [std::mem::MaybeUninit<u8>; 4],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer {
                bytes: Default::default(),
            }
        }

        fn format<I: Integer>(&mut self, i: I) -> &str {
            let string = i.write(unsafe {
                &mut *(&mut self.bytes as *mut [std::mem::MaybeUninit<u8>; 4]
                    as *mut <I as private::Sealed>::Buffer)
            });
            if string.len() > I::MAX_STR_LEN {
                unsafe { std::hint::unreachable_unchecked() };
            }
            string
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(255u8);
    assert_eq!(result, "255");
}

#[test]
fn test_format_i32() {
    struct Buffer {
        bytes: [std::mem::MaybeUninit<u8>; 12],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer {
                bytes: Default::default(),
            }
        }

        fn format<I: Integer>(&mut self, i: I) -> &str {
            let string = i.write(unsafe {
                &mut *(&mut self.bytes as *mut [std::mem::MaybeUninit<u8>; 12]
                    as *mut <I as private::Sealed>::Buffer)
            });
            if string.len() > I::MAX_STR_LEN {
                unsafe { std::hint::unreachable_unchecked() };
            }
            string
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(-123456i32);
    assert_eq!(result, "-123456");
}

#[test]
#[should_panic]
fn test_format_exceeding_max_length() {
    struct Buffer {
        bytes: [std::mem::MaybeUninit<u8>; 4],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer {
                bytes: Default::default(),
            }
        }

        fn format<I: Integer>(&mut self, i: I) -> &str {
            let string = i.write(unsafe {
                &mut *(&mut self.bytes as *mut [std::mem::MaybeUninit<u8>; 4]
                    as *mut <I as private::Sealed>::Buffer)
            });
            if string.len() > I::MAX_STR_LEN {
                unsafe { std::hint::unreachable_unchecked() };
            }
            string
        }
    }

    let mut buffer = Buffer::new();
    let _result = buffer.format(12345678901u32); // This value will exceed the max length for u32.
}

