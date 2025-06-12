#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn test_function0(_param0 :u8) {
    let mut _local0 = itoa::Buffer::new();
    let _local1_param0_helper1 = &mut (_local0);
    let _: &str = itoa::Buffer::format(_local1_param0_helper1, _param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function0(_param0);
    });
}
