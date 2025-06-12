// Answer 0

#[test]
fn test_chunks_vectored_empty() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            (self.data.as_slice(), &[])
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            if self.is_empty() || dst.is_empty() {
                return 0;
            }

            let (s1, s2) = self.as_slices();
            dst[0] = std::io::IoSlice::new(s1);
            if s2.is_empty() || dst.len() == 1 {
                return 1;
            }

            dst[1] = std::io::IoSlice::new(s2);
            2
        }
    }

    let vec_deque = VecDeque { data: Vec::new() };
    let mut dst = vec![std::io::IoSlice::new(&[])]; 
    assert_eq!(vec_deque.chunks_vectored(&mut dst), 0);
}

#[test]
fn test_chunks_vectored_single_slice() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            (self.data.as_slice(), &[])
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            if self.is_empty() || dst.is_empty() {
                return 0;
            }

            let (s1, s2) = self.as_slices();
            dst[0] = std::io::IoSlice::new(s1);
            if s2.is_empty() || dst.len() == 1 {
                return 1;
            }

            dst[1] = std::io::IoSlice::new(s2);
            2
        }
    }

    let vec_deque = VecDeque { data: vec![1, 2, 3] };
    let mut dst = vec![std::io::IoSlice::new(&[])]; 
    assert_eq!(vec_deque.chunks_vectored(&mut dst), 1);
    assert_eq!(dst[0].as_ref(), &[1, 2, 3]);
}

#[test]
fn test_chunks_vectored_two_slices() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            (self.data.as_slice(), &[4, 5, 6])
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            if self.is_empty() || dst.is_empty() {
                return 0;
            }

            let (s1, s2) = self.as_slices();
            dst[0] = std::io::IoSlice::new(s1);
            if s2.is_empty() || dst.len() == 1 {
                return 1;
            }

            dst[1] = std::io::IoSlice::new(s2);
            2
        }
    }

    let vec_deque = VecDeque { data: vec![1, 2, 3] };
    let mut dst = vec![std::io::IoSlice::new(&[]), std::io::IoSlice::new(&[])]; 
    assert_eq!(vec_deque.chunks_vectored(&mut dst), 2);
    assert_eq!(dst[0].as_ref(), &[1, 2, 3]);
    assert_eq!(dst[1].as_ref(), &[4, 5, 6]);
}

