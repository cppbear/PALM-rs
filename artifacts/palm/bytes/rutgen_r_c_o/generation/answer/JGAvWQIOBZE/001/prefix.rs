// Answer 0

#[test]
fn test_hash_empty_bytesmut() {
    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(0 as *mut u8).unwrap(),
        len: 0,
        cap: 65,
        data: &shared as *const _ as *mut Shared,
    };
    let mut hasher = crc32fast::Hasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_small_bytesmut() {
    let shared = Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 3,
        ref_count: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: 3,
        cap: 65,
        data: &shared as *const _ as *mut Shared,
    };
    let mut hasher = crc32fast::Hasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_max_capacity_bytesmut() {
    let shared = Shared {
        vec: (0..65).collect(),
        original_capacity_repr: 65,
        ref_count: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: 65,
        cap: 65,
        data: &shared as *const _ as *mut Shared,
    };
    let mut hasher = crc32fast::Hasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_zero_capacity_nonempty() {
    let shared = Shared {
        vec: vec![4, 5, 6, 7],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: 4,
        cap: 0,
        data: &shared as *const _ as *mut Shared,
    };
    let mut hasher = crc32fast::Hasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_ref_count_limit() {
    let shared = Shared {
        vec: vec![8, 9, 10, 11],
        original_capacity_repr: 65,
        ref_count: AtomicUsize::new(99),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: 4,
        cap: 65,
        data: &shared as *const _ as *mut Shared,
    };
    let mut hasher = crc32fast::Hasher::new();
    bytes_mut.hash(&mut hasher);
}

