// Answer 0

#[derive(Debug)]
struct Group {
    control_bytes: Vec<Tag>,
}

impl Group {
    const WIDTH: usize = 4; // Example width

    fn load_aligned(control_bytes: &[Tag]) -> Self {
        Self {
            control_bytes: control_bytes.to_vec(),
        }
    }

    fn store_aligned(&self, control_bytes: &mut [Tag]) {
        control_bytes.copy_from_slice(&self.control_bytes);
    }

    fn convert_special_to_empty_and_full_to_deleted(self) -> Self {
        let mut new_bytes = self.control_bytes.clone();
        for byte in &mut new_bytes {
            *byte = match *byte {
                Tag::FULL => Tag::DELETED,
                Tag::DELETED => Tag::EMPTY,
                Tag::EMPTY => Tag::EMPTY,
            };
        }
        Self {
            control_bytes: new_bytes,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tag {
    EMPTY,
    DELETED,
    FULL,
}

struct RawTableInner {
    control: Vec<Tag>,
}

impl RawTableInner {
    fn new(buckets: usize) -> Self {
        let control = vec![Tag::EMPTY; buckets];
        Self { control }
    }

    fn buckets(&self) -> usize {
        self.control.len()
    }

    fn ctrl(&mut self, index: usize) -> &mut [Tag] {
        &mut self.control[index..]
    }
}

impl RawTableInner {
    unsafe fn prepare_rehash_in_place(&mut self) {
        for i in (0..self.buckets()).step_by(Group::WIDTH) {
            let group = Group::load_aligned(self.ctrl(i));
            let group = group.convert_special_to_empty_and_full_to_deleted();
            group.store_aligned(self.ctrl(i));
        }

        if self.buckets() < Group::WIDTH {
            self.ctrl(0)
                .copy_to(self.ctrl(Group::WIDTH), self.buckets());
        } else {
            self.ctrl(0)
                .copy_to(self.ctrl(self.buckets()), Group::WIDTH);
        }
    }
}

#[test]
fn test_prepare_rehash_in_place_all_empty() {
    let mut table = RawTableInner::new(8);
    unsafe {
        table.prepare_rehash_in_place();
    }
    assert_eq!(table.control, vec![Tag::EMPTY; 8]);
}

#[test]
fn test_prepare_rehash_in_place_all_full() {
    let mut table = RawTableInner::new(8);
    table.control.fill(Tag::FULL);
    unsafe {
        table.prepare_rehash_in_place();
    }
    assert_eq!(table.control, vec![Tag::DELETED; 8]);
}

#[test]
fn test_prepare_rehash_in_place_mixed() {
    let mut table = RawTableInner::new(8);
    table.control[0] = Tag::FULL;
    table.control[1] = Tag::DELETED;
    table.control[2] = Tag::EMPTY;
    table.control[3] = Tag::FULL;
    unsafe {
        table.prepare_rehash_in_place();
    }
    assert_eq!(table.control, vec![Tag::DELETED, Tag::EMPTY, Tag::EMPTY, Tag::DELETED, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY, Tag::EMPTY]);
}

#[test]
fn test_prepare_rehash_in_place_small_table() {
    let mut table = RawTableInner::new(3);
    table.control[0] = Tag::FULL;
    table.control[1] = Tag::DELETED;
    table.control[2] = Tag::EMPTY;
    unsafe {
        table.prepare_rehash_in_place();
    }
    assert_eq!(table.control, vec![Tag::DELETED, Tag::EMPTY, Tag::EMPTY]);
}

#[should_panic]
fn test_prepare_rehash_in_place_empty_table() {
    let mut table = RawTableInner::new(0);
    unsafe {
        table.prepare_rehash_in_place(); // this should cause undefined behavior due to unallocated table
    }
}

