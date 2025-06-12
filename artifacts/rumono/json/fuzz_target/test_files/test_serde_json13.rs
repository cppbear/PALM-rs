#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

fn test_function13(_param0 :&[u8]) {
    let _local0 = serde_json::de::SliceRead::<'_>::new(_param0);
    let mut _local1: serde_json::Deserializer::<serde_json::de::SliceRead::<'_>> = serde_json::Deserializer::<serde_json::de::SliceRead::<'_>>::new(_local0);
    let _local2_param0_helper1 = &mut (_local1);
    let _: serde_json::Result::<()> = serde_json::Deserializer::<serde_json::de::SliceRead::<'_>>::end(_local2_param0_helper1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function13(_param0);
    });
}
