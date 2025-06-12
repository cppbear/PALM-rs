// Answer 0

#[cfg(test)]
struct MockWriter;

impl MockWriter {
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_flush_success() {
    let mut writer = MockWriter;
    let result = writer.flush();
    assert!(result.is_ok());
}

