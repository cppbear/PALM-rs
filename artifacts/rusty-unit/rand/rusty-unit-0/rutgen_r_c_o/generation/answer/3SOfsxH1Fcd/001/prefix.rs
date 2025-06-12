// Answer 0

struct MockRng {
    data: Vec<u8>,
    position: usize,
}

impl MockRng {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }
}

impl RngCore for MockRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let len = dest.len();
        for i in 0..len {
            if self.position < self.data.len() {
                dest[i] = self.data[self.position];
                self.position += 1;
            } else {
                dest[i] = 0; // fill with zero if out of data
            }
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[test]
fn test_next_u32_via_fill_normal_case() {
    let mut rng = MockRng::new(vec![1, 2, 3, 4]);
    let result = next_u32_via_fill(&mut rng);
}

#[test]
fn test_next_u32_via_fill_edge_case_zeroes() {
    let mut rng = MockRng::new(vec![0, 0, 0, 0]);
    let result = next_u32_via_fill(&mut rng);
}

#[test]
fn test_next_u32_via_fill_case_large_bytes() {
    let mut rng = MockRng::new(vec![5, 6, 7, 8, 9, 10]);
    let result = next_u32_via_fill(&mut rng);
}

#[test]
fn test_next_u32_via_fill_exceeding_data() {
    let mut rng = MockRng::new(vec![9, 8, 7]);
    let result = next_u32_via_fill(&mut rng);
}

#[test]
fn test_next_u32_via_fill_edge_case_empty() {
    let mut rng = MockRng::new(vec![]);
    let result = next_u32_via_fill(&mut rng);
}

