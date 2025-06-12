// Answer 0

fn next_or_eof(_: &mut Dummy) -> Result<u8> {
    Ok(b'\\')
}

fn is_escape(ch: u8, _: bool) -> bool {
    ch == b'\\' || ch == b'"'
}

fn ignore_escape(_: &mut Dummy) -> Result<()> {
    Err(ErrorCode::ControlCharacterWhileParsingString)
}

struct Dummy;

#[derive(Debug)]
enum ErrorCode {
    ControlCharacterWhileParsingString,
}

type Result<T> = std::result::Result<T, ErrorCode>;

fn error(_: &Dummy, code: ErrorCode) -> Result<()> {
    Err(code)
}

fn ignore_str(_: &mut Dummy) -> Result<()> {
    loop {
        let ch = next_or_eof(&mut Dummy)?;
        if !is_escape(ch, true) {
            continue;
        }
        match ch {
            b'"' => {
                return Ok(());
            }
            b'\\' => {
                ignore_escape(&mut Dummy)?;
            }
            _ => {
                return error(&Dummy, ErrorCode::ControlCharacterWhileParsingString);
            }
        }
    }
}

#[test]
fn test_ignore_str_panic_condition() {
    let mut dummy = Dummy;
    let result = ignore_str(&mut dummy);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, ErrorCode::ControlCharacterWhileParsingString);
    }
}

