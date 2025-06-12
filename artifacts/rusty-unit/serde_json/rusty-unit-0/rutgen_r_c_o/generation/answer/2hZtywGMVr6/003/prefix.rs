// Answer 0

#[test]
fn test_deserialize_map_valid() {
    let mut scratch = Vec::new();
    let reader = StrRead::new("{\"key1\": \"value1\", \"key2\": \"value2\"}");
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };

    let visitor = MyVisitor {};
    let _result = deserializer.deserialize_map(visitor);
}

#[test]
fn test_deserialize_map_empty_object() {
    let mut scratch = Vec::new();
    let reader = StrRead::new("{}");
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };

    let visitor = MyVisitor {};
    let _result = deserializer.deserialize_map(visitor);
}

#[test]
fn test_deserialize_map_invalid_peek() {
    let mut scratch = Vec::new();
    let reader = StrRead::new("not_a_json");
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 1,
    };

    let visitor = MyVisitor {};
    let _result = deserializer.deserialize_map(visitor);
}

#[test]
fn test_deserialize_map_recursion_limit_hit() {
    let mut scratch = Vec::new();
    let json_string = "{\"key1\": {\"key2\": {\"key3\": {\"key4\": {\"key5\": \
                       {\"key6\": {\"key7\": {\"key8\": {\"key9\": {\"key10\": \
                       {\"key11\": {\"key12\": {\"key13\": {\"key14\": \
                       {\"key15\": {\"key16\": {\"key17\": {\"key18\": {\"key19\": {\"key20\": \
                       {\"key21\": {\"key22\": {\"key23\": {\"key24\": {\"key25\": \
                       {\"key26\": {\"key27\": {\"key28\": {\"key29\": {\"key30\": \
                       {\"key31\": {\"key32\": {\"key33\": {\"key34\": {\"key35\": \
                       {\"key36\": {\"key37\": {\"key38\": {\"key39\": {\"key40\": \
                       {\"key41\": {\"key42\": {\"key43\": {\"key44\": {\"key45\": \
                       {\"key46\": {\"key47\": {\"key48\": {\"key49\": {\"key50\": \
                       {\"key51\": {\"key52\": {\"key53\": {\"key54\": {\"key55\": \
                       {\"key56\": {\"key57\": {\"key58\": {\"key59\": {\"key60\": \
                       {\"key61\": {\"key62\": {\"key63\": {\"key64\": {\"key65\": \
                       {\"key66\": {\"key67\": {\"key68\": {\"key69\": {\"key70\": \
                       {\"key71\": {\"key72\": {\"key73\": {\"key74\": {\"key75\": \
                       {\"key76\": {\"key77\": {\"key78\": {\"key79\": {\"key80\": \
                       {\"key81\": {\"key82\": {\"key83\": {\"key84\": {\"key85\": \
                       {\"key86\": {\"key87\": {\"key88\": {\"key89\": {\"key90\": \
                       {\"key91\": {\"key92\": {\"key93\": {\"key94\": {\"key95\": \
                       {\"key96\": {\"key97\": {\"key98\": {\"key99\": {\"key100\": \"value\"}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}";
    let reader = StrRead::new(json_string);
    let mut deserializer = Deserializer {
        read: reader,
        scratch,
        remaining_depth: 128,
    };

    let visitor = MyVisitor {};
    let _result = deserializer.deserialize_map(visitor);
}

struct MyVisitor;

impl<'de> de::Visitor<'de> for MyVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<V>(self, _map: V) -> Result<Self::Value>
    where
        V: de::MapAccess<'de>,
    {
        Ok(())
    }
}

