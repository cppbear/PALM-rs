// Answer 0

fn next_or_eof(ctx: &mut TestContext) -> Result<u8> {
    // Simulated implementation for example purposes
    if ctx.values.is_empty() {
        Err(ErrorCode::EndOfFile)
    } else {
        Ok(ctx.values.remove(0))
    }
}

fn is_escape(ch: u8, _: bool) -> bool {
    // Simulated implementation for example purposes; we will set escape check to always return false
    false
}

#[derive(Debug)]
enum ErrorCode {
    ControlCharacterWhileParsingString,
    EndOfFile,
}

#[derive(Debug)]
struct TestContext {
    values: Vec<u8>,
}

fn ignore_escape(_: &mut TestContext) -> Result<()> {
    Ok(())
}

fn error(_: &TestContext, err: ErrorCode) -> Result<()> {
    Err(err)
}

fn ignore_str(ctx: &mut TestContext) -> Result<()> {
    loop {
        let ch = next_or_eof(ctx)?;
        if !is_escape(ch, true) {
            continue;
        }
        match ch {
            b'"' => {
                return Ok(());
            }
            b'\\' => {
                ignore_escape(ctx)?;
            }
            _ => {
                return error(ctx, ErrorCode::ControlCharacterWhileParsingString);
            }
        }
    }
}

#[test]
fn test_ignore_str_success() {
    let mut ctx = TestContext { values: vec![b'"'] };
    let result = ignore_str(&mut ctx);
    assert!(result.is_ok());
}

#[test]
fn test_ignore_str_control_character() {
    let mut ctx = TestContext { values: vec![b'\n'] };
    let result = ignore_str(&mut ctx);
    assert!(matches!(result, Err(ErrorCode::ControlCharacterWhileParsingString)));
}

#[test]
fn test_ignore_str_empty_context() {
    let mut ctx = TestContext { values: vec![] };
    let result = ignore_str(&mut ctx);
    assert!(matches!(result, Err(ErrorCode::EndOfFile)));
}

