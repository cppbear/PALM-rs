// Answer 0

#[derive(Debug)]
struct RandomChooser {
    // Add any fields if necessary, even though this example doesn't use them.
}

impl RandomChooser {
    pub fn usize(&mut self, range: std::ops::Range<usize>) -> usize {
        let mut rng = rand::thread_rng(); // Assuming rand crate is used for randomness.
        rand::Rng::gen_range(&mut rng, range)
    }

    pub fn choice<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        let mut iter = iter.into_iter();
        let len = iter.len();
        if len == 0 {
            return None;
        }
        let index = self.usize(0..len);
        iter.nth(index)
    }
}

#[test]
fn test_choice_non_empty_iterator() {
    let mut chooser = RandomChooser {};
    let items = vec![1, 2, 3, 4, 5];
    let chosen = chooser.choice(items);
    assert!(chosen.is_some());
}

#[test]
fn test_choice_empty_iterator() {
    let mut chooser = RandomChooser {};
    let items: Vec<i32> = Vec::new();
    let chosen = chooser.choice(items);
    assert!(chosen.is_none());
}

#[test]
fn test_choice_single_item_iterator() {
    let mut chooser = RandomChooser {};
    let items = vec![42];
    let chosen = chooser.choice(items);
    assert_eq!(chosen, Some(42));
}

