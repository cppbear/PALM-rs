// Answer 0

#[test]
fn test_num_chars_empty_ranges() {
    struct TestStruct {
        ranges: Vec<(char, char)>,
    }
    
    let test_instance = TestStruct { ranges: vec![] };
    assert_eq!(test_instance.num_chars(), 0);
}

#[test]
fn test_num_chars_single_range() {
    struct TestStruct {
        ranges: Vec<(char, char)>,
    }
    
    impl TestStruct {
        pub fn num_chars(&self) -> usize {
            self.ranges.iter()
                .map(|&(s, e)| 1 + (e as u32) - (s as u32))
                .fold(0, |acc, len| acc + len)
                as usize
        }
    }

    let test_instance = TestStruct { ranges: vec![('a', 'a')] };
    assert_eq!(test_instance.num_chars(), 1);
}

#[test]
fn test_num_chars_multiple_ranges() {
    struct TestStruct {
        ranges: Vec<(char, char)>,
    }
    
    impl TestStruct {
        pub fn num_chars(&self) -> usize {
            self.ranges.iter()
                .map(|&(s, e)| 1 + (e as u32) - (s as u32))
                .fold(0, |acc, len| acc + len)
                as usize
        }
    }

    let test_instance = TestStruct { ranges: vec![('a', 'c'), ('e', 'g')] };
    assert_eq!(test_instance.num_chars(), 7);
}

#[test]
fn test_num_chars_overlapping_ranges() {
    struct TestStruct {
        ranges: Vec<(char, char)>,
    }
    
    impl TestStruct {
        pub fn num_chars(&self) -> usize {
            self.ranges.iter()
                .map(|&(s, e)| 1 + (e as u32) - (s as u32))
                .fold(0, |acc, len| acc + len)
                as usize
        }
    }

    let test_instance = TestStruct { ranges: vec![('a', 'd'), ('b', 'c')] };
    assert_eq!(test_instance.num_chars(), 4);
}

#[test]
fn test_num_chars_reverse_order() {
    struct TestStruct {
        ranges: Vec<(char, char)>,
    }
    
    impl TestStruct {
        pub fn num_chars(&self) -> usize {
            self.ranges.iter()
                .map(|&(s, e)| 1 + (e as u32) - (s as u32))
                .fold(0, |acc, len| acc + len)
                as usize
        }
    }

    let test_instance = TestStruct { ranges: vec![('d', 'a')] };
    assert_eq!(test_instance.num_chars(), 0);
}

