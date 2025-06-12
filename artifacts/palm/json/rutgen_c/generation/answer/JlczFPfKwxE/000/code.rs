// Answer 0

#[test]
fn test_deserializer_new() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> read::Read<'de> for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let remaining = self.data.len() - self.position;
            let to_copy = buf.len().min(remaining);
            buf[..to_copy].copy_from_slice(&self.data[self.position..self.position + to_copy]);
            self.position += to_copy;
            Ok(to_copy)
        }
    }

    let mock_data = b"{\"key\":\"value\"}".to_vec();
    let mock_read = MockRead { data: mock_data, position: 0 };
    let deserializer = Deserializer::new(mock_read);

    assert_eq!(deserializer.remaining_depth, 128);
    assert_eq!(deserializer.scratch.len(), 0);
}

