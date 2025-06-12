#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

fn test_function39(_param0 :&[u8]) {
    let _local0 = bytes::Bytes::from_static(_param0);
    let _local1 = bytes::Bytes::new();
    let _local2 = <std::vec::Vec::<u8> as std::convert::From::<bytes::Bytes>>::from(_local1);
    let _local3_param0_helper1 = &(_local0);
    let _local3_param1_helper1 = &(_local2);
    let _local3_param1_helper2 = &(_local3_param1_helper1);
    let _: bool = <bytes::Bytes as std::cmp::PartialEq::<&std::vec::Vec::<u8>>>::eq(_local3_param0_helper1, _local3_param1_helper2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function39(_param0);
    });
}
