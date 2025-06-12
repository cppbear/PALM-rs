// Answer 0

#[test]
fn test_chunks_vectored() {
    use std::io::IoSlice;

    struct A {
        data: Vec<u8>,
    }

    impl A {
        fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
            let len = self.data.len();
            for (i, &byte) in self.data.iter().enumerate() {
                if i < dst.len() {
                    dst[i] = IoSlice::new(&self.data[i..i+1]);
                } else {
                    break;
                }
            }
            len.min(dst.len())
        }
    }

    struct B {
        data: Vec<u8>,
    }

    impl B {
        fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
            let len = self.data.len();
            for (i, &byte) in self.data.iter().enumerate() {
                if i < dst.len() {
                    dst[i] = IoSlice::new(&self.data[i..i+1]);
                } else {
                    break;
                }
            }
            len.min(dst.len())
        }
    }

    struct Chain<'a> {
        a: A,
        b: B,
    }

    impl<'a> Chain<'a> {
        fn chunks_vectored(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
            let mut n = self.a.chunks_vectored(dst);
            n += self.b.chunks_vectored(&mut dst[n..]);
            n
        }
    }

    let a = A { data: vec![1, 2, 3] };
    let b = B { data: vec![4, 5, 6] };
    let chain = Chain { a, b };

    let mut dst = vec![IoSlice::new(&[]); 6];  // Maximum size of the destination buffer

    let n = chain.chunks_vectored(&mut dst);
    
    assert_eq!(n, 6);
    assert_eq!(dst[0].as_slice(), &[1]);
    assert_eq!(dst[1].as_slice(), &[2]);
    assert_eq!(dst[2].as_slice(), &[3]);
    assert_eq!(dst[3].as_slice(), &[4]);
    assert_eq!(dst[4].as_slice(), &[5]);
    assert_eq!(dst[5].as_slice(), &[6]);
}

