// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_shuffle_with_multiple_elements() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = vec![1, 2, 3, 4, 5];
        data.shuffle(&mut rng);
        assert_eq!(data.len(), 5);
    }

    #[test]
    fn test_shuffle_with_two_elements() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = vec![1, 2];
        data.shuffle(&mut rng);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_shuffle_large_array() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = (1..=100).collect();
        data.shuffle(&mut rng);
        assert_eq!(data.len(), 100);
    }
}

