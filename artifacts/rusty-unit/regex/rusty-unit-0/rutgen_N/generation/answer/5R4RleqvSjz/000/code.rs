// Answer 0

#[test]
fn test_set_word_boundary() {
    struct Dummy {
        ranges: Vec<(u8, u8)>,
    }
    
    impl Dummy {
        fn new() -> Self {
            Dummy { ranges: vec![] }
        }
        
        fn set_range(&mut self, start: u8, end: u8) {
            self.ranges.push((start, end));
        }

        fn get_ranges(&self) -> &Vec<(u8, u8)> {
            &self.ranges
        }

        fn is_word_byte(byte: u8) -> bool {
            byte.is_ascii_alphabetic() || byte.is_ascii_digit() || byte == b'_'
        }

        fn set_word_boundary(&mut self) {
            let iswb = Self::is_word_byte;
            let mut b1: u16 = 0;
            let mut b2: u16;
            while b1 <= 255 {
                b2 = b1 + 1;
                while b2 <= 255 && iswb(b1 as u8) == iswb(b2 as u8) {
                    b2 += 1;
                }
                self.set_range(b1 as u8, (b2 - 1) as u8);
                b1 = b2;
            }
        }
    }

    let mut dummy = Dummy::new();
    dummy.set_word_boundary();
    
    let expected_ranges = vec![
        (0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7),
        (8, 8), (9, 9), (10, 10), (11, 11), (12, 12), (13, 13), (14, 14),
        (15, 15), (16, 16), (17, 17), (18, 18), (19, 19), (20, 20), (21, 21),
        (22, 22), (23, 23), (24, 24), (25, 25), (26, 26), (27, 27), (28, 28),
        (29, 29), (30, 30), (31, 31), (32, 32), (33, 33), (34, 34), (35, 35),
        (36, 36), (37, 37), (38, 38), (39, 39), (40, 40), (41, 41), (42, 42),
        (43, 43), (44, 44), (45, 45), (46, 46), (47, 47), (48, 48), (49, 49),
        (50, 50), (51, 51), (52, 52), (53, 53), (54, 54), (55, 55), (56, 56),
        (57, 57), (58, 58), (59, 59), (60, 60), (61, 61), (62, 62), (63, 63),
        (64, 64), (65, 90), (91, 96), (97, 122), (123, 126), (127, 255)
    ];
    
    assert_eq!(dummy.get_ranges(), &expected_ranges);
}

