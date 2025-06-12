#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn test_function12(_param0 :usize ,_param1 :usize ,_param2 :usize ,_param3 :&[u8]) {
    let _local0 = bytes::BytesMut::with_capacity(_param0);
    let _local1 = <bytes::BytesMut as bytes::buf::BufMut>::limit(_local0, _param1);
    let mut _local2: bytes::buf::Limit::<bytes::buf::Limit::<bytes::BytesMut>> = <bytes::buf::Limit::<bytes::BytesMut> as bytes::buf::BufMut>::limit(_local1, _param2);
    let _local3_param0_helper1 = &mut (_local2);
    let _: () = <bytes::buf::Limit::<bytes::buf::Limit::<bytes::BytesMut>> as bytes::buf::BufMut>::put_slice(_local3_param0_helper1, _param3);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 25 {return;}
        let dynamic_length = (data.len() - 24) / 1;
        let _param0 = _to_usize(data, 0);
        let _param1 = _to_usize(data, 8);
        let _param2 = _to_usize(data, 16);
        let _param3 = _to_slice::<u8>(data, 24 + 0 * dynamic_length, data.len());
        test_function12(_param0 ,_param1 ,_param2 ,_param3);
    });
}
