// Answer 0

#[test]
fn test_ignore_str_escape_handling() {
    struct TestContext {
        input: Vec<u8>,
    }

    impl TestContext {
        fn new(input: Vec<u8>) -> Self {
            TestContext { input }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.input.is_empty() {
                Err(ErrorCode::EOF)
            } else {
                Ok(self.input.remove(0))
            }
        }

        fn is_escape(ch: u8, _: bool) -> bool {
            ch == b'\\'
        }

        fn ignore_escape(&mut self) -> Result<()> {
            if self.input.is_empty() {
                return Err(ErrorCode::EOF);
            }
            self.input.remove(0); // Simulating ignoring the escape
            Ok(())
        }
    }

    fn ignore_str(ctx: &mut TestContext) -> Result<()> {
        loop {
            let ch = ctx.next_or_eof()?;
            if !ctx.is_escape(ch, true) {
                continue;
            }
            match ch {
                b'"' => {
                    return Ok(());
                }
                b'\\' => {
                    ctx.ignore_escape()?;
                }
                _ => {
                    return Err(ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }

    // Test case where next_or_eof returns Ok values that are not escapes
    let mut ctx = TestContext::new(vec![b'a', b'b', b'c']);
    let result = ignore_str(&mut ctx);
    assert!(result.is_ok());

    // Test case where next_or_eof returns an escape followed by a control character
    let mut ctx = TestContext::new(vec![b'\\', b'\x01']);
    let result = ignore_str(&mut ctx);
    assert!(result.is_err());

    // Test case where next_or_eof encounters EOF
    let mut ctx = TestContext::new(vec![]);
    let result = ignore_str(&mut ctx);
    assert!(result.is_err());
}

