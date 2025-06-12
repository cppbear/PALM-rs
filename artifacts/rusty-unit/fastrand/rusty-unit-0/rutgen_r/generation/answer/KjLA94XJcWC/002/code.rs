// Answer 0

#[derive(Default)]
struct RandomChooser {
    // Simulating the random number generator with a simple counter for test consistency
    counter: usize,
}

impl RandomChooser {
    pub fn usize(&mut self, range: std::ops::Range<usize>) -> usize {
        let result = self.counter % (range.end - range.start) + range.start;
        self.counter += 1;
        result
    }

    pub fn choose_multiple<I: IntoIterator>(&mut self, source: I, amount: usize) -> Vec<I::Item> {
        let mut reservoir = Vec::with_capacity(amount);
        let mut iter = source.into_iter();

        reservoir.extend(iter.by_ref().take(amount));

        if reservoir.len() == amount {
            for (i, elem) in iter.enumerate() {
                let end = i + 1 + amount;
                let k = self.usize(0..end);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        } else {
            if reservoir.capacity() > 3 * reservoir.len() {
                reservoir.shrink_to_fit();
            }
        }
        reservoir
    }
}

#[test]
fn test_choose_multiple_exact_amount() {
    let mut chooser = RandomChooser::default();
    let source = vec![1, 2, 3, 4, 5];
    let amount = 3;
    
    let result = chooser.choose_multiple(source, amount);
    
    assert_eq!(result.len(), amount);
    assert!(result.iter().all(|&x| x >= 1 && x <= 5));
}

#[test]
fn test_choose_multiple_fewer_elements() {
    let mut chooser = RandomChooser::default();
    let source = vec![1, 2];
    let amount = 4;
    
    let result = chooser.choose_multiple(source, amount);
    
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|&x| x >= 1 && x <= 2));
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut chooser = RandomChooser::default();
    let source: Vec<i32> = vec![];
    let amount = 3;
    
    let result = chooser.choose_multiple(source, amount);
    
    assert_eq!(result.len(), 0);
}

#[test]
#[should_panic]
fn test_choose_multiple_panic_k_out_of_bounds() {
    let mut chooser = RandomChooser::default();
    let source = vec![1, 2, 3];
    let amount = 3;
    
    // Triggering a panic by using the internal counter manipulation
    chooser.counter = 5; // This will lead to k being out of bounds within the choose_multiple method
    chooser.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_large_amount() {
    let mut chooser = RandomChooser::default();
    let source: Vec<i32> = (1..=100).collect();
    let amount = 100;
    
    let result = chooser.choose_multiple(source, amount);
    
    assert_eq!(result.len(), amount);
    assert!(result.iter().all(|&x| x >= 1 && x <= 100));
}

