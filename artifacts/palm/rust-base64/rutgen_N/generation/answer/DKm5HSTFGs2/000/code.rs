// Answer 0

#[derive(Default)]
struct MockWriter {
    buffer: Vec<u8>,
}

impl MockWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
    
    fn into_inner(self) -> Vec<u8> {
        self.buffer
    }
}

struct Encoder<W> {
    delegate: Option<W>,
}

impl<W> Encoder<W> 
where 
    W: std::io::Write,
{
    fn new(delegate: W) -> Self {
        Self {
            delegate: Some(delegate),
        }
    }

    fn write_final_leftovers(&mut self) -> std::io::Result<()> {
        // Simulate writing leftovers
        Ok(())
    }

    pub fn finish(&mut self) -> Result<W, std::io::Error> {
        assert!(
            self.delegate.is_some(),
            "Encoder has already had finish() called"
        );

        self.write_final_leftovers()?;

        let writer = self.delegate.take().expect("Writer must be present");

        Ok(writer)
    }
}

#[test]
fn test_finish_successfully() {
    let mut mock_writer = MockWriter::default();
    let mut encoder = Encoder::new(&mut mock_writer);
    
    let result = encoder.finish().unwrap();
    assert_eq!(result.into_inner(), b"");
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_called_twice() {
    let mut mock_writer = MockWriter::default();
    let mut encoder = Encoder::new(&mut mock_writer);
    
    encoder.finish().unwrap();
    encoder.finish().unwrap();
}

