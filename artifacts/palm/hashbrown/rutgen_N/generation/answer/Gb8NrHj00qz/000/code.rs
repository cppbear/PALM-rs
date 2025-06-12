// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;

    #[test]
    fn test_iter_empty() {
        let map: HashMap<i32, i32> = HashMap::new();
        let mut iter = map.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_single() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&1, &10)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_multiple() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&1, &10)));
        assert_eq!(iter.next(), Some((&2, &20)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_boundary() {
        let mut map = HashMap::new();
        for i in 0..100 {
            map.insert(i, i * 10);
        }
        let mut iter = map.iter();
        for i in 0..100 {
            assert_eq!(iter.next(), Some((&i, &(i * 10))));
        }
        assert_eq!(iter.next(), None);
    }
}

