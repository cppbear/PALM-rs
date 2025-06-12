// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    value: T,
}

struct Group {
    index: usize,
}

impl Group {
    const WIDTH: usize = 4;

    fn load_aligned(ptr: *const usize) -> Self {
        Group { index: 0 }
    }

    fn next(&mut self) -> Option<usize> {
        if self.index < Group::WIDTH {
            self.index += 1;
            Some(self.index - 1)
        } else {
            None
        }
    }
}

struct Data<T> {
    index: usize,
    items: Vec<T>,
}

impl<T> Data<T> {
    fn next_n(&mut self, n: usize) -> Bucket<T> {
        let value = self.items[self.index].clone();
        self.index += n;
        Bucket { value }
    }
}

struct Iterator<T> {
    current_group: Group,
    data: Data<T>,
    next_ctrl: *const usize,
    end: *const usize,
}

impl<T> Iterator<T> {
    unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<Bucket<T>> {
        loop {
            if let Some(index) = self.current_group.next() {
                return Some(self.data.next_n(index));
            }

            if DO_CHECK_PTR_RANGE && self.next_ctrl >= self.end {
                return None;
            }

            self.current_group = Group::load_aligned(self.next_ctrl);
            self.data = self.data.next_n(Group::WIDTH);
            self.next_ctrl = (self.next_ctrl as usize + Group::WIDTH) as *const usize;
        }
    }
}

#[test]
fn test_next_impl_with_check() {
    let items = vec![1, 2, 3, 4];
    let data = Data { index: 0, items };
    let group = Group { index: 0 };
    let mut iter = Iterator {
        current_group: group,
        data,
        next_ctrl: &0 as *const _,
        end: &4 as *const _,
    };

    unsafe {
        let result_1 = iter.next_impl::<true>();
        assert_eq!(result_1.unwrap().value, 1);

        let result_2 = iter.next_impl::<true>();
        assert_eq!(result_2.unwrap().value, 2);

        let result_3 = iter.next_impl::<true>();
        assert_eq!(result_3.unwrap().value, 3);

        let result_4 = iter.next_impl::<true>();
        assert_eq!(result_4.unwrap().value, 4);

        let result_5 = iter.next_impl::<true>();
        assert_eq!(result_5, None);
    }
}

#[test]
fn test_next_impl_without_check() {
    let items = vec![1, 2, 3, 4];
    let data = Data { index: 0, items };
    let group = Group { index: 0 };
    let mut iter = Iterator {
        current_group: group,
        data,
        next_ctrl: &0 as *const _,
        end: &4 as *const _,
    };

    unsafe {
        let result_1 = iter.next_impl::<false>();
        assert_eq!(result_1.unwrap().value, 1);

        let result_2 = iter.next_impl::<false>();
        assert_eq!(result_2.unwrap().value, 2);

        let result_3 = iter.next_impl::<false>();
        assert_eq!(result_3.unwrap().value, 3);

        let result_4 = iter.next_impl::<false>();
        assert_eq!(result_4.unwrap().value, 4);

        let result_5 = iter.next_impl::<false>();
        assert_eq!(result_5, None);
    }
}

