// Answer 0

#[derive(Default)]
struct MockSource {
    results: Vec<u8>,
    index: usize,
}

impl MockSource {
    fn new(results: Vec<u8>) -> Self {
        Self { results, index: 0 }
    }

    fn generate_and_set(&mut self, _: u32) {
        self.index = 0; // Reset index to simulate new data generation
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        while read_len < dest.len() {
            if self.index >= self.results.len() {
                self.generate_and_set(0);
            }
            let (consumed_u32, filled_u8) =
                fill_via_chunks(&self.results[self.index..], &mut dest[read_len..]);

            self.index += consumed_u32;
            read_len += filled_u8;
        }
    }
}

fn fill_via_chunks(source: &[u8], dest: &mut [u8]) -> (usize, usize) {
    let to_copy = std::cmp::min(source.len(), dest.len());
    dest[..to_copy].copy_from_slice(&source[..to_copy]);
    (to_copy / 4, to_copy) // For example purposes, simulate chunk consumption
}

#[test]
fn test_fill_bytes_with_enough_data() {
    let mut source = MockSource::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let mut buffer = [0u8; 8];

    source.fill_bytes(&mut buffer);

    assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_fill_bytes_with_exact_data() {
    let mut source = MockSource::new(vec![1, 2, 3, 4]);
    let mut buffer = [0u8; 4];

    source.fill_bytes(&mut buffer);

    assert_eq!(buffer, [1, 2, 3, 4]);
}

#[test]
fn test_fill_bytes_with_more_space_than_data() {
    let mut source = MockSource::new(vec![1, 2, 3]);
    let mut buffer = [0u8; 10];

    source.fill_bytes(&mut buffer);

    assert_eq!(&buffer[..3], [1, 2, 3]);
    assert_eq!(&buffer[3..], [0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_empty_buffer() {
    let mut source = MockSource::new(vec![1, 2, 3, 4]);
    let mut buffer: [u8; 0] = [];

    source.fill_bytes(&mut buffer);

    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_fill_bytes_with_no_data() {
    let mut source = MockSource::new(vec![]);
    let mut buffer = [0u8; 5];

    source.fill_bytes(&mut buffer);

    assert_eq!(&buffer[..], [0, 0, 0, 0, 0]);
}

