// Answer 0

#[test]
fn test_serialize_struct_variant_valid_inputs() {
    let name: &'static str = "test_name";
    let variant_index: u32 = 0;
    let variant: &'static str = "test_variant";
    let len: usize = 0;

    let serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_struct_variant(name, variant_index, variant, len);
}

#[test]
fn test_serialize_struct_variant_non_empty_name() {
    let name: &'static str = "variant_name";
    let variant_index: u32 = 1;
    let variant: &'static str = "another_variant";
    let len: usize = 5;

    let serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_struct_variant(name, variant_index, variant, len);
}

#[test]
fn test_serialize_struct_variant_max_variant_index() {
    let name: &'static str = "max_index_name";
    let variant_index: u32 = std::u32::MAX;
    let variant: &'static str = "max_variant";
    let len: usize = 1;

    let serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_struct_variant(name, variant_index, variant, len);
}

#[test]
fn test_serialize_struct_variant_large_len() {
    let name: &'static str = "large_len_name";
    let variant_index: u32 = 2;
    let variant: &'static str = "large_len_variant";
    let len: usize = std::usize::MAX;

    let serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    map_key_serializer.serialize_struct_variant(name, variant_index, variant, len);
}

