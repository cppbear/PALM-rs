use crate::raw::{
    Allocator, Bucket, Global, RawDrain, RawExtractIf, RawIntoIter, RawIter, RawTable,
};
use crate::{DefaultHashBuilder, Equivalent, TryReserveError};
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ops::Index;

#[cfg(feature = "raw-entry")]
pub use crate::raw_entry::*;

/// A hash map implemented with quadratic probing and SIMD lookup.
///
/// The default hashing algorithm is currently [`foldhash`], though this is
/// subject to change at any point in the future. This hash function is very
/// fast for all types of keys, but this algorithm will typically *not* protect
/// against attacks such as HashDoS.
///
/// The hashing algorithm can be replaced on a per-`HashMap` basis using the
/// [`default`], [`with_hasher`], and [`with_capacity_and_hasher`] methods. Many
/// alternative algorithms are available on crates.io, such as the [`fnv`] crate.
///
/// It is required that the keys implement the [`Eq`] and [`Hash`] traits, although
/// this can frequently be achieved by using `#[derive(PartialEq, Eq, Hash)]`.
/// If you implement these yourself, it is important that the following
/// property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must be equal.
///
/// It is a logic error for a key to be modified in such a way that the key's
/// hash, as determined by the [`Hash`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the map. This is normally only
/// possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
///
/// It is also a logic error for the [`Hash`] implementation of a key to panic.
/// This is generally only possible if the trait is implemented manually. If a
/// panic does occur then the contents of the `HashMap` may become corrupted and
/// some items may be dropped from the table.
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `HashMap<String, String>` in this example).
/// let mut book_reviews = HashMap::new();
///
/// // Review some books.
/// book_reviews.insert(
///     "Adventures of Huckleberry Finn".to_string(),
///     "My favorite book.".to_string(),
/// );
/// book_reviews.insert(
///     "Grimms' Fairy Tales".to_string(),
///     "Masterpiece.".to_string(),
/// );
/// book_reviews.insert(
///     "Pride and Prejudice".to_string(),
///     "Very enjoyable.".to_string(),
/// );
/// book_reviews.insert(
///     "The Adventures of Sherlock Holmes".to_string(),
///     "Eye lyked it alot.".to_string(),
/// );
///
/// // Check for a specific one.
/// // When collections store owned values (String), they can still be
/// // queried using references (&str).
/// if !book_reviews.contains_key("Les Misérables") {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              book_reviews.len());
/// }
///
/// // oops, this review has a lot of spelling mistakes, let's delete it.
/// book_reviews.remove("The Adventures of Sherlock Holmes");
///
/// // Look up the values associated with some keys.
/// let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
/// for &book in &to_find {
///     match book_reviews.get(book) {
///         Some(review) => println!("{}: {}", book, review),
///         None => println!("{} is unreviewed.", book)
///     }
/// }
///
/// // Look up the value for a key (will panic if the key is not found).
/// println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
///
/// // Iterate over everything.
/// for (book, review) in &book_reviews {
///     println!("{}: \"{}\"", book, review);
/// }
/// ```
///
/// `HashMap` also implements an [`Entry API`](#method.entry), which allows
/// for more complex methods of getting, setting, updating and removing keys and
/// their values:
///
/// ```
/// use hashbrown::HashMap;
///
/// // type inference lets us omit an explicit type signature (which
/// // would be `HashMap<&str, u8>` in this example).
/// let mut player_stats = HashMap::new();
///
/// fn random_stat_buff() -> u8 {
///     // could actually return some random value here - let's just return
///     // some fixed value for now
///     42
/// }
///
/// // insert a key only if it doesn't already exist
/// player_stats.entry("health").or_insert(100);
///
/// // insert a key using a function that provides a new value only if it
/// // doesn't already exist
/// player_stats.entry("defence").or_insert_with(random_stat_buff);
///
/// // update a key, guarding against the key possibly not being set
/// let stat = player_stats.entry("attack").or_insert(100);
/// *stat += random_stat_buff();
/// ```
///
/// The easiest way to use `HashMap` with a custom key type is to derive [`Eq`] and [`Hash`].
/// We must also derive [`PartialEq`].
///
/// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
/// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [`default`]: #method.default
/// [`with_hasher`]: #method.with_hasher
/// [`with_capacity_and_hasher`]: #method.with_capacity_and_hasher
/// [`fnv`]: https://crates.io/crates/fnv
/// [`foldhash`]: https://crates.io/crates/foldhash
///
/// ```
/// use hashbrown::HashMap;
///
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     country: String,
/// }
///
/// impl Viking {
///     /// Creates a new Viking.
///     fn new(name: &str, country: &str) -> Viking {
///         Viking { name: name.to_string(), country: country.to_string() }
///     }
/// }
///
/// // Use a HashMap to store the vikings' health points.
/// let mut vikings = HashMap::new();
///
/// vikings.insert(Viking::new("Einar", "Norway"), 25);
/// vikings.insert(Viking::new("Olaf", "Denmark"), 24);
/// vikings.insert(Viking::new("Harald", "Iceland"), 12);
///
/// // Use derived implementation to print the status of the vikings.
/// for (viking, health) in &vikings {
///     println!("{:?} has {} hp", viking, health);
/// }
/// ```
///
/// A `HashMap` with fixed list of elements can be initialized from an array:
///
/// ```
/// use hashbrown::HashMap;
///
/// let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
///     .into_iter().collect();
/// // use the values stored in map
/// ```
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}

impl<K: Clone, V: Clone, S: Clone, A: Allocator + Clone> Clone for HashMap<K, V, S, A> {
    fn clone(&self) -> Self {
        HashMap {
            hash_builder: self.hash_builder.clone(),
            table: self.table.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.table.clone_from(&source.table);

        // Update hash_builder only if we successfully cloned all elements.
        self.hash_builder.clone_from(&source.hash_builder);
    }
}

/// Ensures that a single closure type across uses of this which, in turn prevents multiple
/// instances of any functions like `RawTable::reserve` from being generated
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hasher<Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    Q: Hash,
    S: BuildHasher,
{
    move |val| make_hash::<Q, S>(hash_builder, &val.0)
}

/// Ensures that a single closure type across uses of this which, in turn prevents multiple
/// instances of any functions like `RawTable::reserve` from being generated
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    Q: Equivalent<K> + ?Sized,
{
    move |x| k.equivalent(&x.0)
}

/// Ensures that a single closure type across uses of this which, in turn prevents multiple
/// instances of any functions like `RawTable::reserve` from being generated
#[cfg_attr(feature = "inline-more", inline)]
#[allow(dead_code)]
pub(crate) fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized,
{
    move |x| k.equivalent(x)
}

#[cfg(not(feature = "nightly"))]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

#[cfg(feature = "nightly")]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}

#[cfg(feature = "default-hasher")]
impl<K, V> HashMap<K, V, DefaultHashBuilder> {
    /// Creates an empty `HashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`], for example with
    /// [`with_hasher`](HashMap::with_hasher) method.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    /// assert_eq!(map.len(), 0);
    /// assert_eq!(map.capacity(), 0);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `HashMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`], for example with
    /// [`with_capacity_and_hasher`](HashMap::with_capacity_and_hasher) method.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 10);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, DefaultHashBuilder::default())
    }
}

#[cfg(feature = "default-hasher")]
impl<K, V, A: Allocator> HashMap<K, V, DefaultHashBuilder, A> {
    /// Creates an empty `HashMap` using the given allocator.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`], for example with
    /// [`with_hasher_in`](HashMap::with_hasher_in) method.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use bumpalo::Bump;
    ///
    /// let bump = Bump::new();
    /// let mut map = HashMap::new_in(&bump);
    ///
    /// // The created HashMap holds none elements
    /// assert_eq!(map.len(), 0);
    ///
    /// // The created HashMap also doesn't allocate memory
    /// assert_eq!(map.capacity(), 0);
    ///
    /// // Now we insert element inside created HashMap
    /// map.insert("One", 1);
    /// // We can see that the HashMap holds 1 element
    /// assert_eq!(map.len(), 1);
    /// // And it also allocates some capacity
    /// assert!(map.capacity() > 1);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new_in(alloc: A) -> Self {
        Self::with_hasher_in(DefaultHashBuilder::default(), alloc)
    }

    /// Creates an empty `HashMap` with the specified capacity using the given allocator.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`], for example with
    /// [`with_capacity_and_hasher_in`](HashMap::with_capacity_and_hasher_in) method.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use bumpalo::Bump;
    ///
    /// let bump = Bump::new();
    /// let mut map = HashMap::with_capacity_in(5, &bump);
    ///
    /// // The created HashMap holds none elements
    /// assert_eq!(map.len(), 0);
    /// // But it can hold at least 5 elements without reallocating
    /// let empty_map_capacity = map.capacity();
    /// assert!(empty_map_capacity >= 5);
    ///
    /// // Now we insert some 5 elements inside created HashMap
    /// map.insert("One",   1);
    /// map.insert("Two",   2);
    /// map.insert("Three", 3);
    /// map.insert("Four",  4);
    /// map.insert("Five",  5);
    ///
    /// // We can see that the HashMap holds 5 elements
    /// assert_eq!(map.len(), 5);
    /// // But its capacity isn't changed
    /// assert_eq!(map.capacity(), empty_map_capacity)
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::default(), alloc)
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty `HashMap` which will use the given hash builder to hash
    /// keys.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`].
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the `HashMap` to be useful, see its documentation for details.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::DefaultHashBuilder;
    ///
    /// let s = DefaultHashBuilder::default();
    /// let mut map = HashMap::with_hasher(s);
    /// assert_eq!(map.len(), 0);
    /// assert_eq!(map.capacity(), 0);
    ///
    /// map.insert(1, 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg_attr(feature = "rustc-dep-of-std", rustc_const_stable_indirect)]
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::new(),
        }
    }

    /// Creates an empty `HashMap` with the specified capacity, using `hash_builder`
    /// to hash the keys.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`].
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the `HashMap` to be useful, see its documentation for details.
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::DefaultHashBuilder;
    ///
    /// let s = DefaultHashBuilder::default();
    /// let mut map = HashMap::with_capacity_and_hasher(10, s);
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 10);
    ///
    /// map.insert(1, 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::with_capacity(capacity),
        }
    }
}

impl<K, V, S, A: Allocator> HashMap<K, V, S, A> {
    /// Returns a reference to the underlying allocator.
    #[inline]
    pub fn allocator(&self) -> &A {
        self.table.allocator()
    }

    /// Creates an empty `HashMap` which will use the given hash builder to hash
    /// keys. It will be allocated with the given allocator.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`].
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::DefaultHashBuilder;
    ///
    /// let s = DefaultHashBuilder::default();
    /// let mut map = HashMap::with_hasher(s);
    /// map.insert(1, 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg_attr(feature = "rustc-dep-of-std", rustc_const_stable_indirect)]
    pub const fn with_hasher_in(hash_builder: S, alloc: A) -> Self {
        Self {
            hash_builder,
            table: RawTable::new_in(alloc),
        }
    }

    /// Creates an empty `HashMap` with the specified capacity, using `hash_builder`
    /// to hash the keys. It will be allocated with the given allocator.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # HashDoS resistance
    ///
    /// The `hash_builder` normally use a fixed key by default and that does
    /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
    /// Users who require HashDoS resistance should explicitly use
    /// [`std::collections::hash_map::RandomState`]
    /// as the hasher when creating a [`HashMap`].
    ///
    /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
    /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::DefaultHashBuilder;
    ///
    /// let s = DefaultHashBuilder::default();
    /// let mut map = HashMap::with_capacity_and_hasher(10, s);
    /// map.insert(1, 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self {
        Self {
            hash_builder,
            table: RawTable::with_capacity_in(capacity, alloc),
        }
    }

    /// Returns a reference to the map's [`BuildHasher`].
    ///
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::DefaultHashBuilder;
    ///
    /// let hasher = DefaultHashBuilder::default();
    /// let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
    /// let hasher: &DefaultHashBuilder = map.hasher();
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn hasher(&self) -> &S {
        &self.hash_builder
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `HashMap<K, V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 100);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {
        self.table.capacity()
    }

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    /// assert_eq!(map.len(), 3);
    /// let mut vec: Vec<&str> = Vec::new();
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    ///     vec.push(*key);
    /// }
    ///
    /// // The `Keys` iterator produces keys in arbitrary order, so the
    /// // keys must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, ["a", "b", "c"]);
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { inner: self.iter() }
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    /// assert_eq!(map.len(), 3);
    /// let mut vec: Vec<i32> = Vec::new();
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    ///     vec.push(*val);
    /// }
    ///
    /// // The `Values` iterator produces values in arbitrary order, so the
    /// // values must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn values(&self) -> Values<'_, K, V> {
        Values { inner: self.iter() }
    }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values_mut() {
    ///     *val = *val + 10;
    /// }
    ///
    /// assert_eq!(map.len(), 3);
    /// let mut vec: Vec<i32> = Vec::new();
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    ///     vec.push(*val);
    /// }
    ///
    /// // The `Values` iterator produces values in arbitrary order, so the
    /// // values must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [11, 12, 13]);
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    /// assert_eq!(map.len(), 3);
    /// let mut vec: Vec<(&str, i32)> = Vec::new();
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    ///     vec.push((*key, *val));
    /// }
    ///
    /// // The `Iter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        // Here we tie the lifetime of self to the iter.
        unsafe {
            Iter {
                inner: self.table.iter(),
                marker: PhantomData,
            }
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// // Update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val *= 2;
    /// }
    ///
    /// assert_eq!(map.len(), 3);
    /// let mut vec: Vec<(&str, i32)> = Vec::new();
    ///
    /// for (key, val) in &map {
    ///     println!("key: {} val: {}", key, val);
    ///     vec.push((*key, *val));
    /// }
    ///
    /// // The `Iter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [("a", 2), ("b", 4), ("c", 6)]);
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        // Here we tie the lifetime of self to the iter.
        unsafe {
            IterMut {
                inner: self.table.iter(),
                marker: PhantomData,
            }
        }
    }

    #[cfg(test)]
    #[cfg_attr(feature = "inline-more", inline)]
    fn raw_capacity(&self) -> usize {
        self.table.buckets()
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it
    /// drops the remaining key-value pairs. The returned iterator keeps a
    /// mutable borrow on the vector to optimize its implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// let capacity_before_drain = a.capacity();
    ///
    /// for (k, v) in a.drain().take(1) {
    ///     assert!(k == 1 || k == 2);
    ///     assert!(v == "a" || v == "b");
    /// }
    ///
    /// // As we can see, the map is empty and contains no element.
    /// assert!(a.is_empty() && a.len() == 0);
    /// // But map capacity is equal to old one.
    /// assert_eq!(a.capacity(), capacity_before_drain);
    ///
    /// let mut a = HashMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    ///
    /// {   // Iterator is dropped without being consumed.
    ///     let d = a.drain();
    /// }
    ///
    /// // But the map is empty even if we do not use Drain iterator.
    /// assert!(a.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn drain(&mut self) -> Drain<'_, K, V, A> {
        Drain {
            inner: self.table.drain(),
        }
    }

    /// Retains only the elements specified by the predicate. Keeps the
    /// allocated memory for reuse.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<i32, i32> = (0..8).map(|x|(x, x*10)).collect();
    /// assert_eq!(map.len(), 8);
    ///
    /// map.retain(|&k, _| k % 2 == 0);
    ///
    /// // We can see, that the number of elements inside map is changed.
    /// assert_eq!(map.len(), 4);
    ///
    /// let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    /// vec.sort_unstable();
    /// assert_eq!(vec, [(0, 0), (2, 20), (4, 40), (6, 60)]);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        // Here we only use `iter` as a temporary, preventing use-after-free
        unsafe {
            for item in self.table.iter() {
                let &mut (ref key, ref mut value) = item.as_mut();
                if !f(key, value) {
                    self.table.erase(item);
                }
            }
        }
    }

    /// Drains elements which are true under the given predicate,
    /// and returns an iterator over the removed items.
    ///
    /// In other words, move all pairs `(k, v)` such that `f(&k, &mut v)` returns `true` out
    /// into another iterator.
    ///
    /// Note that `extract_if` lets you mutate every value in the filter closure, regardless of
    /// whether you choose to keep or remove it.
    ///
    /// If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating
    /// or the iteration short-circuits, then the remaining elements will be retained.
    /// Use [`retain()`] with a negated predicate if you do not need the returned iterator.
    ///
    /// Keeps the allocated memory for reuse.
    ///
    /// [`retain()`]: HashMap::retain
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    ///
    /// let drained: HashMap<i32, i32> = map.extract_if(|k, _v| k % 2 == 0).collect();
    ///
    /// let mut evens = drained.keys().cloned().collect::<Vec<_>>();
    /// let mut odds = map.keys().cloned().collect::<Vec<_>>();
    /// evens.sort();
    /// odds.sort();
    ///
    /// assert_eq!(evens, vec![0, 2, 4, 6]);
    /// assert_eq!(odds, vec![1, 3, 5, 7]);
    ///
    /// let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    ///
    /// {   // Iterator is dropped without being consumed.
    ///     let d = map.extract_if(|k, _v| k % 2 != 0);
    /// }
    ///
    /// // ExtractIf was not exhausted, therefore no elements were drained.
    /// assert_eq!(map.len(), 8);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, K, V, F, A>
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        ExtractIf {
            f,
            inner: RawExtractIf {
                iter: unsafe { self.table.iter() },
                table: &mut self.table,
            },
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// a.insert(1, "a");
    /// let capacity_before_clear = a.capacity();
    ///
    /// a.clear();
    ///
    /// // Map is empty.
    /// assert!(a.is_empty());
    /// // But map capacity is equal to old one.
    /// assert_eq!(a.capacity(), capacity_before_clear);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {
        self.table.clear();
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// let mut vec: Vec<&str> = map.into_keys().collect();
    ///
    /// // The `IntoKeys` iterator produces keys in arbitrary order, so the
    /// // keys must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, ["a", "b", "c"]);
    /// ```
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        IntoKeys {
            inner: self.into_iter(),
        }
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// let mut vec: Vec<i32> = map.into_values().collect();
    ///
    /// // The `IntoValues` iterator produces values in arbitrary order, so
    /// // the values must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V, A> {
        IntoValues {
            inner: self.into_iter(),
        }
    }
}

impl<K, V, S, A> HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the `HashMap`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes and [`abort`] the program
    /// in case of allocation error. Use [`try_reserve`](HashMap::try_reserve) instead
    /// if you want to handle memory allocation failure.
    ///
    /// [`isize::MAX`]: https://doc.rust-lang.org/std/primitive.isize.html
    /// [`abort`]: https://doc.rust-lang.org/alloc/alloc/fn.handle_alloc_error.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    /// // Map is empty and doesn't allocate memory
    /// assert_eq!(map.capacity(), 0);
    ///
    /// map.reserve(10);
    ///
    /// // And now map can hold at least 10 elements
    /// assert!(map.capacity() >= 10);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn reserve(&mut self, additional: usize) {
        self.table
            .reserve(additional, make_hasher::<_, V, S>(&self.hash_builder));
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `HashMap<K,V>`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, isize> = HashMap::new();
    /// // Map is empty and doesn't allocate memory
    /// assert_eq!(map.capacity(), 0);
    ///
    /// map.try_reserve(10).expect("why is the test harness OOMing on 10 bytes?");
    ///
    /// // And now map can hold at least 10 elements
    /// assert!(map.capacity() >= 10);
    /// ```
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned:
    /// ```
    /// # fn test() {
    /// use hashbrown::HashMap;
    /// use hashbrown::TryReserveError;
    /// let mut map: HashMap<i32, i32> = HashMap::new();
    ///
    /// match map.try_reserve(usize::MAX) {
    ///     Err(error) => match error {
    ///         TryReserveError::CapacityOverflow => {}
    ///         _ => panic!("TryReserveError::AllocError ?"),
    ///     },
    ///     _ => panic!(),
    /// }
    /// # }
    /// # fn main() {
    /// #     #[cfg(not(miri))]
    /// #     test()
    /// # }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.table
            .try_reserve(additional, make_hasher::<_, V, S>(&self.hash_builder))
    }

    /// Shrinks the capacity of the map as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    /// map.insert(1, 2);
    /// map.insert(3, 4);
    /// assert!(map.capacity() >= 100);
    /// map.shrink_to_fit();
    /// assert!(map.capacity() >= 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to_fit(&mut self) {
        self.table
            .shrink_to(0, make_hasher::<_, V, S>(&self.hash_builder));
    }

    /// Shrinks the capacity of the map with a lower limit. It will drop
    /// down no lower than the supplied limit while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// This function does nothing if the current capacity is smaller than the
    /// supplied minimum capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    /// map.insert(1, 2);
    /// map.insert(3, 4);
    /// assert!(map.capacity() >= 100);
    /// map.shrink_to(10);
    /// assert!(map.capacity() >= 10);
    /// map.shrink_to(0);
    /// assert!(map.capacity() >= 2);
    /// map.shrink_to(10);
    /// assert!(map.capacity() >= 2);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.table
            .shrink_to(min_capacity, make_hasher::<_, V, S>(&self.hash_builder));
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut letters = HashMap::new();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     let counter = letters.entry(ch).or_insert(0);
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(letters[&'s'], 2);
    /// assert_eq!(letters[&'t'], 3);
    /// assert_eq!(letters[&'u'], 1);
    /// assert_eq!(letters.get(&'y'), None);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S, A> {
        let hash = make_hash::<K, S>(&self.hash_builder, &key);
        if let Some(elem) = self.table.find(hash, equivalent_key(&key)) {
            Entry::Occupied(OccupiedEntry {
                hash,
                elem,
                table: self,
            })
        } else {
            Entry::Vacant(VacantEntry {
                hash,
                key,
                table: self,
            })
        }
    }

    /// Gets the given key's corresponding entry by reference in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut words: HashMap<String, usize> = HashMap::new();
    /// let source = ["poneyland", "horseyland", "poneyland", "poneyland"];
    /// for (i, &s) in source.iter().enumerate() {
    ///     let counter = words.entry_ref(s).or_insert(0);
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(words["poneyland"], 3);
    /// assert_eq!(words["horseyland"], 1);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry_ref<'a, 'b, Q>(&'a mut self, key: &'b Q) -> EntryRef<'a, 'b, K, Q, V, S, A>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.hash_builder, key);
        if let Some(elem) = self.table.find(hash, equivalent_key(key)) {
            EntryRef::Occupied(OccupiedEntry {
                hash,
                elem,
                table: self,
            })
        } else {
            EntryRef::Vacant(VacantEntryRef {
                hash,
                key,
                table: self,
            })
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.get_inner(k) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.get_inner(k) {
            Some((key, value)) => Some((key, value)),
            None => None,
        }
    }

    #[inline]
    fn get_inner<Q>(&self, k: &Q) -> Option<&(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.table.is_empty() {
            None
        } else {
            let hash = make_hash::<Q, S>(&self.hash_builder, k);
            self.table.get(hash, equivalent_key(k))
        }
    }

    /// Returns the key-value pair corresponding to the supplied key, with a mutable reference to value.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// let (k, v) = map.get_key_value_mut(&1).unwrap();
    /// assert_eq!(k, &1);
    /// assert_eq!(v, &mut "a");
    /// *v = "b";
    /// assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "b")));
    /// assert_eq!(map.get_key_value_mut(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value_mut<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.get_inner_mut(k) {
            Some(&mut (ref key, ref mut value)) => Some((key, value)),
            None => None,
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_inner(k).is_some()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    ///
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.get_inner_mut(k) {
            Some(&mut (_, ref mut v)) => Some(v),
            None => None,
        }
    }

    #[inline]
    fn get_inner_mut<Q>(&mut self, k: &Q) -> Option<&mut (K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.table.is_empty() {
            None
        } else {
            let hash = make_hash::<Q, S>(&self.hash_builder, k);
            self.table.get_mut(hash, equivalent_key(k))
        }
    }

    /// Attempts to get mutable references to `N` values in the map at once.
    ///
    /// Returns an array of length `N` with the results of each query. For soundness, at most one
    /// mutable reference will be returned to any value. `None` will be used if the key is missing.
    ///
    /// # Panics
    ///
    /// Panics if any keys are overlapping.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Bodleian Library".to_string(), 1602);
    /// libraries.insert("Athenæum".to_string(), 1807);
    /// libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    /// libraries.insert("Library of Congress".to_string(), 1800);
    ///
    /// // Get Athenæum and Bodleian Library
    /// let [Some(a), Some(b)] = libraries.get_many_mut([
    ///     "Athenæum",
    ///     "Bodleian Library",
    /// ]) else { panic!() };
    ///
    /// // Assert values of Athenæum and Library of Congress
    /// let got = libraries.get_many_mut([
    ///     "Athenæum",
    ///     "Library of Congress",
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some(&mut 1807),
    ///         Some(&mut 1800),
    ///     ],
    /// );
    ///
    /// // Missing keys result in None
    /// let got = libraries.get_many_mut([
    ///     "Athenæum",
    ///     "New York Public Library",
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some(&mut 1807),
    ///         None
    ///     ]
    /// );
    /// ```
    ///
    /// ```should_panic
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Athenæum".to_string(), 1807);
    ///
    /// // Duplicate keys panic!
    /// let got = libraries.get_many_mut([
    ///     "Athenæum",
    ///     "Athenæum",
    /// ]);
    /// ```
    pub fn get_many_mut<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut V>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_many_mut_inner(ks).map(|res| res.map(|(_, v)| v))
    }

    /// Attempts to get mutable references to `N` values in the map at once, without validating that
    /// the values are unique.
    ///
    /// Returns an array of length `N` with the results of each query. `None` will be used if
    /// the key is missing.
    ///
    /// For a safe alternative see [`get_many_mut`](`HashMap::get_many_mut`).
    ///
    /// # Safety
    ///
    /// Calling this method with overlapping keys is *[undefined behavior]* even if the resulting
    /// references are not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Bodleian Library".to_string(), 1602);
    /// libraries.insert("Athenæum".to_string(), 1807);
    /// libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    /// libraries.insert("Library of Congress".to_string(), 1800);
    ///
    /// // SAFETY: The keys do not overlap.
    /// let [Some(a), Some(b)] = (unsafe { libraries.get_many_unchecked_mut([
    ///     "Athenæum",
    ///     "Bodleian Library",
    /// ]) }) else { panic!() };
    ///
    /// // SAFETY: The keys do not overlap.
    /// let got = unsafe { libraries.get_many_unchecked_mut([
    ///     "Athenæum",
    ///     "Library of Congress",
    /// ]) };
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some(&mut 1807),
    ///         Some(&mut 1800),
    ///     ],
    /// );
    ///
    /// // SAFETY: The keys do not overlap.
    /// let got = unsafe { libraries.get_many_unchecked_mut([
    ///     "Athenæum",
    ///     "New York Public Library",
    /// ]) };
    /// // Missing keys result in None
    /// assert_eq!(got, [Some(&mut 1807), None]);
    /// ```
    pub unsafe fn get_many_unchecked_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut V>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_many_unchecked_mut_inner(ks)
            .map(|res| res.map(|(_, v)| v))
    }

    /// Attempts to get mutable references to `N` values in the map at once, with immutable
    /// references to the corresponding keys.
    ///
    /// Returns an array of length `N` with the results of each query. For soundness, at most one
    /// mutable reference will be returned to any value. `None` will be used if the key is missing.
    ///
    /// # Panics
    ///
    /// Panics if any keys are overlapping.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Bodleian Library".to_string(), 1602);
    /// libraries.insert("Athenæum".to_string(), 1807);
    /// libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    /// libraries.insert("Library of Congress".to_string(), 1800);
    ///
    /// let got = libraries.get_many_key_value_mut([
    ///     "Bodleian Library",
    ///     "Herzogin-Anna-Amalia-Bibliothek",
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some((&"Bodleian Library".to_string(), &mut 1602)),
    ///         Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
    ///     ],
    /// );
    /// // Missing keys result in None
    /// let got = libraries.get_many_key_value_mut([
    ///     "Bodleian Library",
    ///     "Gewandhaus",
    /// ]);
    /// assert_eq!(got, [Some((&"Bodleian Library".to_string(), &mut 1602)), None]);
    /// ```
    ///
    /// ```should_panic
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Bodleian Library".to_string(), 1602);
    /// libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    ///
    /// // Duplicate keys result in panic!
    /// let got = libraries.get_many_key_value_mut([
    ///     "Bodleian Library",
    ///     "Herzogin-Anna-Amalia-Bibliothek",
    ///     "Herzogin-Anna-Amalia-Bibliothek",
    /// ]);
    /// ```
    pub fn get_many_key_value_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<(&'_ K, &'_ mut V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_many_mut_inner(ks)
            .map(|res| res.map(|(k, v)| (&*k, v)))
    }

    /// Attempts to get mutable references to `N` values in the map at once, with immutable
    /// references to the corresponding keys, without validating that the values are unique.
    ///
    /// Returns an array of length `N` with the results of each query. `None` will be returned if
    /// any of the keys are missing.
    ///
    /// For a safe alternative see [`get_many_key_value_mut`](`HashMap::get_many_key_value_mut`).
    ///
    /// # Safety
    ///
    /// Calling this method with overlapping keys is *[undefined behavior]* even if the resulting
    /// references are not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut libraries = HashMap::new();
    /// libraries.insert("Bodleian Library".to_string(), 1602);
    /// libraries.insert("Athenæum".to_string(), 1807);
    /// libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    /// libraries.insert("Library of Congress".to_string(), 1800);
    ///
    /// let got = libraries.get_many_key_value_mut([
    ///     "Bodleian Library",
    ///     "Herzogin-Anna-Amalia-Bibliothek",
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some((&"Bodleian Library".to_string(), &mut 1602)),
    ///         Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
    ///     ],
    /// );
    /// // Missing keys result in None
    /// let got = libraries.get_many_key_value_mut([
    ///     "Bodleian Library",
    ///     "Gewandhaus",
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     [
    ///         Some((&"Bodleian Library".to_string(), &mut 1602)),
    ///         None,
    ///     ],
    /// );
    /// ```
    pub unsafe fn get_many_key_value_unchecked_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<(&'_ K, &'_ mut V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_many_unchecked_mut_inner(ks)
            .map(|res| res.map(|(k, v)| (&*k, v)))
    }

    fn get_many_mut_inner<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut (K, V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hashes = self.build_hashes_inner(ks);
        self.table
            .get_many_mut(hashes, |i, (k, _)| ks[i].equivalent(k))
    }

    unsafe fn get_many_unchecked_mut_inner<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut (K, V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hashes = self.build_hashes_inner(ks);
        self.table
            .get_many_unchecked_mut(hashes, |i, (k, _)| ks[i].equivalent(k))
    }

    fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let mut hashes = [0_u64; N];
        for i in 0..N {
            hashes[i] = make_hash::<Q, S>(&self.hash_builder, ks[i]);
        }
        hashes
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical. See the [`std::collections`]
    /// [module-level documentation] for more.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`std::collections`]: https://doc.rust-lang.org/std/collections/index.html
    /// [module-level documentation]: https://doc.rust-lang.org/std/collections/index.html#insert-and-complex-keys
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        match self.find_or_find_insert_slot(hash, &k) {
            Ok(bucket) => Some(mem::replace(unsafe { &mut bucket.as_mut().1 }, v)),
            Err(slot) => {
                unsafe {
                    self.table.insert_in_slot(hash, slot, (k, v));
                }
                None
            }
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn find_or_find_insert_slot<Q>(
        &mut self,
        hash: u64,
        key: &Q,
    ) -> Result<Bucket<(K, V)>, crate::raw::InsertSlot>
    where
        Q: Equivalent<K> + ?Sized,
    {
        self.table.find_or_find_insert_slot(
            hash,
            equivalent_key(key),
            make_hasher(&self.hash_builder),
        )
    }

    /// Insert a key-value pair into the map without checking
    /// if the key already exists in the map.
    ///
    /// This operation is faster than regular insert, because it does not perform
    /// lookup before insertion.
    ///
    /// This operation is useful during initial population of the map.
    /// For example, when constructing a map from another map, we know
    /// that keys are unique.
    ///
    /// Returns a reference to the key and value just inserted.
    ///
    /// # Safety
    ///
    /// This operation is safe if a key does not exist in the map.
    ///
    /// However, if a key exists in the map already, the behavior is unspecified:
    /// this operation may panic, loop forever, or any following operation with the map
    /// may panic, loop forever or return arbitrary result.
    ///
    /// That said, this operation (and following operations) are guaranteed to
    /// not violate memory safety.
    ///
    /// However this operation is still unsafe because the resulting `HashMap`
    /// may be passed to unsafe code which does expect the map to behave
    /// correctly, and would cause unsoundness as a result.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map1 = HashMap::new();
    /// assert_eq!(map1.insert(1, "a"), None);
    /// assert_eq!(map1.insert(2, "b"), None);
    /// assert_eq!(map1.insert(3, "c"), None);
    /// assert_eq!(map1.len(), 3);
    ///
    /// let mut map2 = HashMap::new();
    ///
    /// for (key, value) in map1.into_iter() {
    ///     unsafe {
    ///         map2.insert_unique_unchecked(key, value);
    ///     }
    /// }
    ///
    /// let (key, value) = unsafe { map2.insert_unique_unchecked(4, "d") };
    /// assert_eq!(key, &4);
    /// assert_eq!(value, &mut "d");
    /// *value = "e";
    ///
    /// assert_eq!(map2[&1], "a");
    /// assert_eq!(map2[&2], "b");
    /// assert_eq!(map2[&3], "c");
    /// assert_eq!(map2[&4], "e");
    /// assert_eq!(map2.len(), 4);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn insert_unique_unchecked(&mut self, k: K, v: V) -> (&K, &mut V) {
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        let bucket = self
            .table
            .insert(hash, (k, v), make_hasher::<_, V, S>(&self.hash_builder));
        let (k_ref, v_ref) = unsafe { bucket.as_mut() };
        (k_ref, v_ref)
    }

    /// Tries to insert a key-value pair into the map, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Errors
    ///
    /// If the map already had this key present, nothing is updated, and
    /// an error containing the occupied entry and the value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::OccupiedError;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.try_insert(37, "a").unwrap(), &"a");
    ///
    /// match map.try_insert(37, "b") {
    ///     Err(OccupiedError { entry, value }) => {
    ///         assert_eq!(entry.key(), &37);
    ///         assert_eq!(entry.get(), &"a");
    ///         assert_eq!(value, "b");
    ///     }
    ///     _ => panic!()
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<&mut V, OccupiedError<'_, K, V, S, A>> {
        match self.entry(key) {
            Entry::Occupied(entry) => Err(OccupiedError { entry, value }),
            Entry::Vacant(entry) => Ok(entry.insert(value)),
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map. Keeps the allocated memory for reuse.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// // The map is empty
    /// assert!(map.is_empty() && map.capacity() == 0);
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    ///
    /// // Now map holds none elements
    /// assert!(map.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.remove_entry(k) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map. Keeps the allocated memory for reuse.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// // The map is empty
    /// assert!(map.is_empty() && map.capacity() == 0);
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.remove_entry(&1), Some((1, "a")));
    /// assert_eq!(map.remove(&1), None);
    ///
    /// // Now map hold none elements
    /// assert!(map.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.hash_builder, k);
        self.table.remove_entry(hash, equivalent_key(k))
    }

    /// Returns the total amount of memory allocated internally by the hash
    /// set, in bytes.
    ///
    /// The returned number is informational only. It is intended to be
    /// primarily used for memory profiling.
    #[inline]
    pub fn allocation_size(&self) -> usize {
        self.table.allocation_size()
    }
}

impl<K, V, S, A> PartialEq for HashMap<K, V, S, A>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, S, A> Eq for HashMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
    A: Allocator,
{
}

impl<K, V, S, A> Debug for HashMap<K, V, S, A>
where
    K: Debug,
    V: Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S, A> Default for HashMap<K, V, S, A>
where
    S: Default,
    A: Default + Allocator,
{
    /// Creates an empty `HashMap<K, V, S, A>`, with the `Default` value for the hasher and allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// // You can specify all types of HashMap, including hasher and allocator.
    /// // Created map is empty and don't allocate memory
    /// let map: HashMap<u32, String> = Default::default();
    /// assert_eq!(map.capacity(), 0);
    /// let map: HashMap<u32, String, RandomState> = HashMap::default();
    /// assert_eq!(map.capacity(), 0);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::with_hasher_in(Default::default(), Default::default())
    }
}

impl<K, Q, V, S, A> Index<&Q> for HashMap<K, V, S, A>
where
    K: Eq + Hash,
    Q: Hash + Equivalent<K> + ?Sized,
    S: BuildHasher,
    A: Allocator,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let map: HashMap<_, _> = [("a", "One"), ("b", "Two")].into();
    ///
    /// assert_eq!(map[&"a"], "One");
    /// assert_eq!(map[&"b"], "Two");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

// The default hasher is used to match the std implementation signature
#[cfg(feature = "default-hasher")]
impl<K, V, A, const N: usize> From<[(K, V); N]> for HashMap<K, V, DefaultHashBuilder, A>
where
    K: Eq + Hash,
    A: Default + Allocator,
{
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let map1 = HashMap::from([(1, 2), (3, 4)]);
    /// let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

/// An iterator over the entries of a `HashMap` in arbitrary order.
/// The iterator element type is `(&'a K, &'a V)`.
///
/// This `struct` is created by the [`iter`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`iter`]: struct.HashMap.html#method.iter
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut iter = map.iter();
/// let mut vec = vec![iter.next(), iter.next(), iter.next()];
///
/// // The `Iter` iterator produces items in arbitrary order, so the
/// // items must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some((&1, &"a")), Some((&2, &"b")), Some((&3, &"c"))]);
///
/// // It is fused iterator
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.next(), None);
/// ```
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<K, V> Clone for Iter<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<K: Debug, V: Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A mutable iterator over the entries of a `HashMap` in arbitrary order.
/// The iterator element type is `(&'a K, &'a mut V)`.
///
/// This `struct` is created by the [`iter_mut`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`iter_mut`]: struct.HashMap.html#method.iter_mut
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let mut map: HashMap<_, _> = [(1, "One".to_owned()), (2, "Two".into())].into();
///
/// let mut iter = map.iter_mut();
/// iter.next().map(|(_, v)| v.push_str(" Mississippi"));
/// iter.next().map(|(_, v)| v.push_str(" Mississippi"));
///
/// // It is fused iterator
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.next(), None);
///
/// assert_eq!(map.get(&1).unwrap(), &"One Mississippi".to_owned());
/// assert_eq!(map.get(&2).unwrap(), &"Two Mississippi".to_owned());
/// ```
pub struct IterMut<'a, K, V> {
    inner: RawIter<(K, V)>,
    // To ensure invariance with respect to V
    marker: PhantomData<(&'a K, &'a mut V)>,
}

// We override the default Send impl which has K: Sync instead of K: Send. Both
// are correct, but this one is more general since it allows keys which
// implement Send but not Sync.
unsafe impl<K: Send, V: Send> Send for IterMut<'_, K, V> {}

impl<K, V> IterMut<'_, K, V> {
    /// Returns a iterator of references over the remaining items.
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

/// An owning iterator over the entries of a `HashMap` in arbitrary order.
/// The iterator element type is `(K, V)`.
///
/// This `struct` is created by the [`into_iter`] method on [`HashMap`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
/// The map cannot be used after calling that method.
///
/// [`into_iter`]: struct.HashMap.html#method.into_iter
/// [`HashMap`]: struct.HashMap.html
/// [`IntoIterator`]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut iter = map.into_iter();
/// let mut vec = vec![iter.next(), iter.next(), iter.next()];
///
/// // The `IntoIter` iterator produces items in arbitrary order, so the
/// // items must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some((1, "a")), Some((2, "b")), Some((3, "c"))]);
///
/// // It is fused iterator
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.next(), None);
/// ```
pub struct IntoIter<K, V, A: Allocator = Global> {
    inner: RawIntoIter<(K, V), A>,
}

impl<K, V, A: Allocator> IntoIter<K, V, A> {
    /// Returns a iterator of references over the remaining items.
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
            marker: PhantomData,
        }
    }
}

/// An owning iterator over the keys of a `HashMap` in arbitrary order.
/// The iterator element type is `K`.
///
/// This `struct` is created by the [`into_keys`] method on [`HashMap`].
/// See its documentation for more.
/// The map cannot be used after calling that method.
///
/// [`into_keys`]: struct.HashMap.html#method.into_keys
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut keys = map.into_keys();
/// let mut vec = vec![keys.next(), keys.next(), keys.next()];
///
/// // The `IntoKeys` iterator produces keys in arbitrary order, so the
/// // keys must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some(1), Some(2), Some(3)]);
///
/// // It is fused iterator
/// assert_eq!(keys.next(), None);
/// assert_eq!(keys.next(), None);
/// ```
pub struct IntoKeys<K, V, A: Allocator = Global> {
    inner: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator> Default for IntoKeys<K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, _)| f(acc, k))
    }
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoKeys<K, V, A> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V, A: Allocator> FusedIterator for IntoKeys<K, V, A> {}

impl<K: Debug, V: Debug, A: Allocator> fmt::Debug for IntoKeys<K, V, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.inner.iter().map(|(k, _)| k))
            .finish()
    }
}

/// An owning iterator over the values of a `HashMap` in arbitrary order.
/// The iterator element type is `V`.
///
/// This `struct` is created by the [`into_values`] method on [`HashMap`].
/// See its documentation for more. The map cannot be used after calling that method.
///
/// [`into_values`]: struct.HashMap.html#method.into_values
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut values = map.into_values();
/// let mut vec = vec![values.next(), values.next(), values.next()];
///
/// // The `IntoValues` iterator produces values in arbitrary order, so
/// // the values must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some("a"), Some("b"), Some("c")]);
///
/// // It is fused iterator
/// assert_eq!(values.next(), None);
/// assert_eq!(values.next(), None);
/// ```
pub struct IntoValues<K, V, A: Allocator = Global> {
    inner: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator> Default for IntoValues<K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoValues<K, V, A> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V, A: Allocator> FusedIterator for IntoValues<K, V, A> {}

impl<K, V: Debug, A: Allocator> fmt::Debug for IntoValues<K, V, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.inner.iter().map(|(_, v)| v))
            .finish()
    }
}

/// An iterator over the keys of a `HashMap` in arbitrary order.
/// The iterator element type is `&'a K`.
///
/// This `struct` is created by the [`keys`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`keys`]: struct.HashMap.html#method.keys
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut keys = map.keys();
/// let mut vec = vec![keys.next(), keys.next(), keys.next()];
///
/// // The `Keys` iterator produces keys in arbitrary order, so the
/// // keys must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some(&1), Some(&2), Some(&3)]);
///
/// // It is fused iterator
/// assert_eq!(keys.next(), None);
/// assert_eq!(keys.next(), None);
/// ```
pub struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<K, V> Clone for Keys<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Debug, V> fmt::Debug for Keys<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// An iterator over the values of a `HashMap` in arbitrary order.
/// The iterator element type is `&'a V`.
///
/// This `struct` is created by the [`values`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`values`]: struct.HashMap.html#method.values
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut values = map.values();
/// let mut vec = vec![values.next(), values.next(), values.next()];
///
/// // The `Values` iterator produces values in arbitrary order, so the
/// // values must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some(&"a"), Some(&"b"), Some(&"c")]);
///
/// // It is fused iterator
/// assert_eq!(values.next(), None);
/// assert_eq!(values.next(), None);
/// ```
pub struct Values<'a, K, V> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<K, V> Clone for Values<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V: Debug> fmt::Debug for Values<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A draining iterator over the entries of a `HashMap` in arbitrary
/// order. The iterator element type is `(K, V)`.
///
/// This `struct` is created by the [`drain`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`drain`]: struct.HashMap.html#method.drain
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let mut map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut drain_iter = map.drain();
/// let mut vec = vec![drain_iter.next(), drain_iter.next(), drain_iter.next()];
///
/// // The `Drain` iterator produces items in arbitrary order, so the
/// // items must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some((1, "a")), Some((2, "b")), Some((3, "c"))]);
///
/// // It is fused iterator
/// assert_eq!(drain_iter.next(), None);
/// assert_eq!(drain_iter.next(), None);
/// ```
pub struct Drain<'a, K, V, A: Allocator = Global> {
    inner: RawDrain<'a, (K, V), A>,
}

impl<K, V, A: Allocator> Drain<'_, K, V, A> {
    /// Returns a iterator of references over the remaining items.
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
            marker: PhantomData,
        }
    }
}

/// A draining iterator over entries of a `HashMap` which don't satisfy the predicate
/// `f(&k, &mut v)` in arbitrary order. The iterator element type is `(K, V)`.
///
/// This `struct` is created by the [`extract_if`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`extract_if`]: struct.HashMap.html#method.extract_if
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
///
/// let mut extract_if = map.extract_if(|k, _v| k % 2 != 0);
/// let mut vec = vec![extract_if.next(), extract_if.next()];
///
/// // The `ExtractIf` iterator produces items in arbitrary order, so the
/// // items must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [Some((1, "a")),Some((3, "c"))]);
///
/// // It is fused iterator
/// assert_eq!(extract_if.next(), None);
/// assert_eq!(extract_if.next(), None);
/// drop(extract_if);
///
/// assert_eq!(map.len(), 1);
/// ```
#[must_use = "Iterators are lazy unless consumed"]
pub struct ExtractIf<'a, K, V, F, A: Allocator = Global> {
    f: F,
    inner: RawExtractIf<'a, (K, V), A>,
}

impl<K, V, F, A> Iterator for ExtractIf<'_, K, V, F, A>
where
    F: FnMut(&K, &mut V) -> bool,
    A: Allocator,
{
    type Item = (K, V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|&mut (ref k, ref mut v)| (self.f)(k, v))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.iter.size_hint().1)
    }
}

impl<K, V, F> FusedIterator for ExtractIf<'_, K, V, F> where F: FnMut(&K, &mut V) -> bool {}

/// A mutable iterator over the values of a `HashMap` in arbitrary order.
/// The iterator element type is `&'a mut V`.
///
/// This `struct` is created by the [`values_mut`] method on [`HashMap`]. See its
/// documentation for more.
///
/// [`values_mut`]: struct.HashMap.html#method.values_mut
/// [`HashMap`]: struct.HashMap.html
///
/// # Examples
///
/// ```
/// use hashbrown::HashMap;
///
/// let mut map: HashMap<_, _> = [(1, "One".to_owned()), (2, "Two".into())].into();
///
/// let mut values = map.values_mut();
/// values.next().map(|v| v.push_str(" Mississippi"));
/// values.next().map(|v| v.push_str(" Mississippi"));
///
/// // It is fused iterator
/// assert_eq!(values.next(), None);
/// assert_eq!(values.next(), None);
///
/// assert_eq!(map.get(&1).unwrap(), &"One Mississippi".to_owned());
/// assert_eq!(map.get(&2).unwrap(), &"Two Mississippi".to_owned());
/// ```
pub struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>,
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`HashMap`].
///
/// [`HashMap`]: struct.HashMap.html
/// [`entry`]: struct.HashMap.html#method.entry
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{Entry, HashMap, OccupiedEntry};
///
/// let mut map = HashMap::new();
/// map.extend([("a", 10), ("b", 20), ("c", 30)]);
/// assert_eq!(map.len(), 3);
///
/// // Existing key (insert)
/// let entry: Entry<_, _, _> = map.entry("a");
/// let _raw_o: OccupiedEntry<_, _, _> = entry.insert(1);
/// assert_eq!(map.len(), 3);
/// // Nonexistent key (insert)
/// map.entry("d").insert(4);
///
/// // Existing key (or_insert)
/// let v = map.entry("b").or_insert(2);
/// assert_eq!(std::mem::replace(v, 2), 20);
/// // Nonexistent key (or_insert)
/// map.entry("e").or_insert(5);
///
/// // Existing key (or_insert_with)
/// let v = map.entry("c").or_insert_with(|| 3);
/// assert_eq!(std::mem::replace(v, 3), 30);
/// // Nonexistent key (or_insert_with)
/// map.entry("f").or_insert_with(|| 6);
///
/// println!("Our HashMap: {:?}", map);
///
/// let mut vec: Vec<_> = map.iter().map(|(&k, &v)| (k, v)).collect();
/// // The `Iter` iterator produces items in arbitrary order, so the
/// // items must be sorted to test them against a sorted array.
/// vec.sort_unstable();
/// assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5), ("f", 6)]);
/// ```
pub enum Entry<'a, K, V, S, A = Global>
where
    A: Allocator,
{
    /// An occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
    ///
    /// match map.entry("a") {
    ///     Entry::Vacant(_) => unreachable!(),
    ///     Entry::Occupied(_) => { }
    /// }
    /// ```
    Occupied(OccupiedEntry<'a, K, V, S, A>),

    /// A vacant entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    ///
    /// match map.entry("a") {
    ///     Entry::Occupied(_) => unreachable!(),
    ///     Entry::Vacant(_) => { }
    /// }
    /// ```
    Vacant(VacantEntry<'a, K, V, S, A>),
}

impl<K: Debug, V: Debug, S, A: Allocator> Debug for Entry<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Entry::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
            Entry::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
        }
    }
}

/// A view into an occupied entry in a [`HashMap`].
/// It is part of the [`Entry`] and [`EntryRef`] enums.
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{Entry, HashMap, OccupiedEntry};
///
/// let mut map = HashMap::new();
/// map.extend([("a", 10), ("b", 20), ("c", 30)]);
///
/// let _entry_o: OccupiedEntry<_, _, _> = map.entry("a").insert(100);
/// assert_eq!(map.len(), 3);
///
/// // Existing key (insert and update)
/// match map.entry("a") {
///     Entry::Vacant(_) => unreachable!(),
///     Entry::Occupied(mut view) => {
///         assert_eq!(view.get(), &100);
///         let v = view.get_mut();
///         *v *= 10;
///         assert_eq!(view.insert(1111), 1000);
///     }
/// }
///
/// assert_eq!(map[&"a"], 1111);
/// assert_eq!(map.len(), 3);
///
/// // Existing key (take)
/// match map.entry("c") {
///     Entry::Vacant(_) => unreachable!(),
///     Entry::Occupied(view) => {
///         assert_eq!(view.remove_entry(), ("c", 30));
///     }
/// }
/// assert_eq!(map.get(&"c"), None);
/// assert_eq!(map.len(), 2);
/// ```
pub struct OccupiedEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    elem: Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}

unsafe impl<K, V, S, A> Send for OccupiedEntry<'_, K, V, S, A>
where
    K: Send,
    V: Send,
    S: Send,
    A: Send + Allocator,
{
}
unsafe impl<K, V, S, A> Sync for OccupiedEntry<'_, K, V, S, A>
where
    K: Sync,
    V: Sync,
    S: Sync,
    A: Sync + Allocator,
{
}

impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedEntry<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

/// A view into a vacant entry in a `HashMap`.
/// It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{Entry, HashMap, VacantEntry};
///
/// let mut map = HashMap::<&str, i32>::new();
///
/// let entry_v: VacantEntry<_, _, _> = match map.entry("a") {
///     Entry::Vacant(view) => view,
///     Entry::Occupied(_) => unreachable!(),
/// };
/// entry_v.insert(10);
/// assert!(map[&"a"] == 10 && map.len() == 1);
///
/// // Nonexistent key (insert and update)
/// match map.entry("b") {
///     Entry::Occupied(_) => unreachable!(),
///     Entry::Vacant(view) => {
///         let value = view.insert(2);
///         assert_eq!(*value, 2);
///         *value = 20;
///     }
/// }
/// assert!(map[&"b"] == 20 && map.len() == 2);
/// ```
pub struct VacantEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}

impl<K: Debug, V, S, A: Allocator> Debug for VacantEntry<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied,
/// with any borrowed form of the map's key type.
///
///
/// This `enum` is constructed from the [`entry_ref`] method on [`HashMap`].
///
/// [`Hash`] and [`Eq`] on the borrowed form of the map's key type *must* match those
/// for the key type. It also require that key may be constructed from the borrowed
/// form through the [`From`] trait.
///
/// [`HashMap`]: struct.HashMap.html
/// [`entry_ref`]: struct.HashMap.html#method.entry_ref
/// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
/// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{EntryRef, HashMap, OccupiedEntry};
///
/// let mut map = HashMap::new();
/// map.extend([("a".to_owned(), 10), ("b".into(), 20), ("c".into(), 30)]);
/// assert_eq!(map.len(), 3);
///
/// // Existing key (insert)
/// let key = String::from("a");
/// let entry: EntryRef<_, _, _, _> = map.entry_ref(&key);
/// let _raw_o: OccupiedEntry<_, _, _, _> = entry.insert(1);
/// assert_eq!(map.len(), 3);
/// // Nonexistent key (insert)
/// map.entry_ref("d").insert(4);
///
/// // Existing key (or_insert)
/// let v = map.entry_ref("b").or_insert(2);
/// assert_eq!(std::mem::replace(v, 2), 20);
/// // Nonexistent key (or_insert)
/// map.entry_ref("e").or_insert(5);
///
/// // Existing key (or_insert_with)
/// let v = map.entry_ref("c").or_insert_with(|| 3);
/// assert_eq!(std::mem::replace(v, 3), 30);
/// // Nonexistent key (or_insert_with)
/// map.entry_ref("f").or_insert_with(|| 6);
///
/// println!("Our HashMap: {:?}", map);
///
/// for (key, value) in ["a", "b", "c", "d", "e", "f"].into_iter().zip(1..=6) {
///     assert_eq!(map[key], value)
/// }
/// assert_eq!(map.len(), 6);
/// ```
pub enum EntryRef<'a, 'b, K, Q: ?Sized, V, S, A = Global>
where
    A: Allocator,
{
    /// An occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{EntryRef, HashMap};
    /// let mut map: HashMap<_, _> = [("a".to_owned(), 100), ("b".into(), 200)].into();
    ///
    /// match map.entry_ref("a") {
    ///     EntryRef::Vacant(_) => unreachable!(),
    ///     EntryRef::Occupied(_) => { }
    /// }
    /// ```
    Occupied(OccupiedEntry<'a, K, V, S, A>),

    /// A vacant entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{EntryRef, HashMap};
    /// let mut map: HashMap<String, i32> = HashMap::new();
    ///
    /// match map.entry_ref("a") {
    ///     EntryRef::Occupied(_) => unreachable!(),
    ///     EntryRef::Vacant(_) => { }
    /// }
    /// ```
    Vacant(VacantEntryRef<'a, 'b, K, Q, V, S, A>),
}

impl<K, Q, V, S, A> Debug for EntryRef<'_, '_, K, Q, V, S, A>
where
    K: Debug + Borrow<Q>,
    Q: Debug + ?Sized,
    V: Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EntryRef::Vacant(ref v) => f.debug_tuple("EntryRef").field(v).finish(),
            EntryRef::Occupied(ref o) => f.debug_tuple("EntryRef").field(o).finish(),
        }
    }
}

/// A view into a vacant entry in a `HashMap`.
/// It is part of the [`EntryRef`] enum.
///
/// [`EntryRef`]: enum.EntryRef.html
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{EntryRef, HashMap, VacantEntryRef};
///
/// let mut map = HashMap::<String, i32>::new();
///
/// let entry_v: VacantEntryRef<_, _, _, _> = match map.entry_ref("a") {
///     EntryRef::Vacant(view) => view,
///     EntryRef::Occupied(_) => unreachable!(),
/// };
/// entry_v.insert(10);
/// assert!(map["a"] == 10 && map.len() == 1);
///
/// // Nonexistent key (insert and update)
/// match map.entry_ref("b") {
///     EntryRef::Occupied(_) => unreachable!(),
///     EntryRef::Vacant(view) => {
///         let value = view.insert(2);
///         assert_eq!(*value, 2);
///         *value = 20;
///     }
/// }
/// assert!(map["b"] == 20 && map.len() == 2);
/// ```
pub struct VacantEntryRef<'a, 'b, K, Q: ?Sized, V, S, A: Allocator = Global> {
    hash: u64,
    key: &'b Q,
    table: &'a mut HashMap<K, V, S, A>,
}

impl<K, Q, V, S, A> Debug for VacantEntryRef<'_, '_, K, Q, V, S, A>
where
    K: Borrow<Q>,
    Q: Debug + ?Sized,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntryRef").field(&self.key()).finish()
    }
}

/// The error returned by [`try_insert`](HashMap::try_insert) when the key already exists.
///
/// Contains the occupied entry, and the value that was not inserted.
///
/// # Examples
///
/// ```
/// use hashbrown::hash_map::{HashMap, OccupiedError};
///
/// let mut map: HashMap<_, _> = [("a", 10), ("b", 20)].into();
///
/// // try_insert method returns mutable reference to the value if keys are vacant,
/// // but if the map did have key present, nothing is updated, and the provided
/// // value is returned inside `Err(_)` variant
/// match map.try_insert("a", 100) {
///     Err(OccupiedError { mut entry, value }) => {
///         assert_eq!(entry.key(), &"a");
///         assert_eq!(value, 100);
///         assert_eq!(entry.insert(100), 10)
///     }
///     _ => unreachable!(),
/// }
/// assert_eq!(map[&"a"], 100);
/// ```
pub struct OccupiedError<'a, K, V, S, A: Allocator = Global> {
    /// The entry in the map that was already occupied.
    pub entry: OccupiedEntry<'a, K, V, S, A>,
    /// The value which was not inserted, because the entry was already occupied.
    pub value: V,
}

impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedError<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedError")
            .field("key", self.entry.key())
            .field("old_value", self.entry.get())
            .field("new_value", &self.value)
            .finish()
    }
}

impl<K: Debug, V: Debug, S, A: Allocator> fmt::Display for OccupiedError<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to insert {:?}, key {:?} already exists with value {:?}",
            self.value,
            self.entry.key(),
            self.entry.get(),
        )
    }
}

impl<'a, K, V, S, A: Allocator> IntoIterator for &'a HashMap<K, V, S, A> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    /// Creates an iterator over the entries of a `HashMap` in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// Return the same `Iter` struct as by the [`iter`] method on [`HashMap`].
    ///
    /// [`iter`]: struct.HashMap.html#method.iter
    /// [`HashMap`]: struct.HashMap.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let map_one: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
    /// let mut map_two = HashMap::new();
    ///
    /// for (key, value) in &map_one {
    ///     println!("Key: {}, Value: {}", key, value);
    ///     map_two.insert(*key, *value);
    /// }
    ///
    /// assert_eq!(map_one, map_two);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K, V, S, A: Allocator> IntoIterator for &'a mut HashMap<K, V, S, A> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    /// Creates an iterator over the entries of a `HashMap` in arbitrary order
    /// with mutable references to the values. The iterator element type is
    /// `(&'a K, &'a mut V)`.
    ///
    /// Return the same `IterMut` struct as by the [`iter_mut`] method on
    /// [`HashMap`].
    ///
    /// [`iter_mut`]: struct.HashMap.html#method.iter_mut
    /// [`HashMap`]: struct.HashMap.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// let mut map: HashMap<_, _> = [("a", 1), ("b", 2), ("c", 3)].into();
    ///
    /// for (key, value) in &mut map {
    ///     println!("Key: {}, Value: {}", key, value);
    ///     *value *= 2;
    /// }
    ///
    /// let mut vec = map.iter().collect::<Vec<_>>();
    /// // The `Iter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [(&"a", &2), (&"b", &4), (&"c", &6)]);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

impl<K, V, S, A: Allocator> IntoIterator for HashMap<K, V, S, A> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, A>;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the map in arbitrary order. The map cannot be used after
    /// calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let map: HashMap<_, _> = [("a", 1), ("b", 2), ("c", 3)].into();
    ///
    /// // Not possible with .iter()
    /// let mut vec: Vec<(&str, i32)> = map.into_iter().collect();
    /// // The `IntoIter` iterator produces items in arbitrary order, so
    /// // the items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> IntoIter<K, V, A> {
        IntoIter {
            inner: self.table.into_iter(),
        }
    }
}

impl<K, V> Default for Iter<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
            marker: PhantomData,
        }
    }
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some(x) => unsafe {
                let r = x.as_ref();
                Some((&r.0, &r.1))
            },
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, x| unsafe {
            let (k, v) = x.as_ref();
            f(acc, (k, v))
        })
    }
}
impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for Iter<'_, K, V> {}

impl<K, V> Default for IterMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
            marker: PhantomData,
        }
    }
}
impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some(x) => unsafe {
                let r = x.as_mut();
                Some((&r.0, &mut r.1))
            },
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, x| unsafe {
            let (k, v) = x.as_mut();
            f(acc, (k, v))
        })
    }
}
impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V> FusedIterator for IterMut<'_, K, V> {}

impl<K, V> fmt::Debug for IterMut<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V, A: Allocator> Default for IntoIter<K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A> {
    type Item = (K, V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }
}
impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A> {}

impl<K: Debug, V: Debug, A: Allocator> fmt::Debug for IntoIter<K, V, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V> Default for Keys<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a K> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, _)| f(acc, k))
    }
}
impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V> FusedIterator for Keys<'_, K, V> {}

impl<K, V> Default for Values<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a V> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}
impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V> FusedIterator for Values<'_, K, V> {}

impl<K, V> Default for ValuesMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a mut V> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}
impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}

impl<K, V: Debug> fmt::Debug for ValuesMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.inner.iter().map(|(_, val)| val))
            .finish()
    }
}

impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A> {
    type Item = (K, V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }
}
impl<K, V, A: Allocator> ExactSizeIterator for Drain<'_, K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<K, V, A: Allocator> FusedIterator for Drain<'_, K, V, A> {}

impl<K, V, A> fmt::Debug for Drain<'_, K, V, A>
where
    K: fmt::Debug,
    V: fmt::Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, K, V, S, A: Allocator> Entry<'a, K, V, S, A> {
    /// Sets the value of the entry, and returns an `OccupiedEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// let entry = map.entry("horseyland").insert(37);
    ///
    /// assert_eq!(entry.key(), &"horseyland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry("poneyland").or_insert(3);
    /// assert_eq!(map["poneyland"], 3);
    ///
    /// // existing key
    /// *map.entry("poneyland").or_insert(10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert(self, default: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry("poneyland").or_insert_with(|| 3);
    /// assert_eq!(map["poneyland"], 3);
    ///
    /// // existing key
    /// *map.entry("poneyland").or_insert_with(|| 10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, usize> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    /// assert_eq!(map["poneyland"], 9);
    ///
    /// // existing key
    /// *map.entry("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;
    /// assert_eq!(map["poneyland"], 18);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(3);
    /// // existing key
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// // nonexistent key
    /// assert_eq!(map.entry("horseland").key(), &"horseland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }

    /// Provides shared access to the key and owned access to the value of
    /// an occupied entry and allows to replace or remove it based on the
    /// value of the returned option.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// let entry = map
    ///     .entry("poneyland")
    ///     .and_replace_entry_with(|_k, _v| panic!());
    ///
    /// match entry {
    ///     Entry::Vacant(e) => {
    ///         assert_eq!(e.key(), &"poneyland");
    ///     }
    ///     Entry::Occupied(_) => panic!(),
    /// }
    ///
    /// map.insert("poneyland", 42);
    ///
    /// let entry = map
    ///     .entry("poneyland")
    ///     .and_replace_entry_with(|k, v| {
    ///         assert_eq!(k, &"poneyland");
    ///         assert_eq!(v, 42);
    ///         Some(v + 1)
    ///     });
    ///
    /// match entry {
    ///     Entry::Occupied(e) => {
    ///         assert_eq!(e.key(), &"poneyland");
    ///         assert_eq!(e.get(), &43);
    ///     }
    ///     Entry::Vacant(_) => panic!(),
    /// }
    ///
    /// assert_eq!(map["poneyland"], 43);
    ///
    /// let entry = map
    ///     .entry("poneyland")
    ///     .and_replace_entry_with(|_k, _v| None);
    ///
    /// match entry {
    ///     Entry::Vacant(e) => assert_eq!(e.key(), &"poneyland"),
    ///     Entry::Occupied(_) => panic!(),
    /// }
    ///
    /// assert!(!map.contains_key("poneyland"));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn and_replace_entry_with<F>(self, f: F) -> Self
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        match self {
            Entry::Occupied(entry) => entry.replace_entry_with(f),
            Entry::Vacant(_) => self,
        }
    }
}

impl<'a, K, V: Default, S, A: Allocator> Entry<'a, K, V, S, A> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry("poneyland").or_default();
    /// assert_eq!(map["poneyland"], None);
    ///
    /// map.insert("horseland", Some(3));
    ///
    /// // existing key
    /// assert_eq!(map.entry("horseland").or_default(), &mut Some(3));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_default(self) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

impl<'a, K, V, S, A: Allocator> OccupiedEntry<'a, K, V, S, A> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// match map.entry("poneyland") {
    ///     Entry::Vacant(_) => panic!(),
    ///     Entry::Occupied(entry) => assert_eq!(entry.key(), &"poneyland"),
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {
        unsafe { &self.elem.as_ref().0 }
    }

    /// Take the ownership of the key and value from the map.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// // The map is empty
    /// assert!(map.is_empty() && map.capacity() == 0);
    ///
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     assert_eq!(o.remove_entry(), ("poneyland", 12));
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// // Now map hold none elements
    /// assert!(map.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry(self) -> (K, V) {
        unsafe { self.table.table.remove(self.elem).0 }
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// match map.entry("poneyland") {
    ///     Entry::Vacant(_) => panic!(),
    ///     Entry::Occupied(entry) => assert_eq!(entry.get(), &12),
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get(&self) -> &V {
        unsafe { &self.elem.as_ref().1 }
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: #method.into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    ///
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut(&mut self) -> &mut V {
        unsafe { &mut self.elem.as_mut().1 }
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: #method.get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    ///
    /// let value: &mut u32;
    /// match map.entry("poneyland") {
    ///     Entry::Occupied(entry) => value = entry.into_mut(),
    ///     Entry::Vacant(_) => panic!(),
    /// }
    /// *value += 10;
    ///
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_mut(self) -> &'a mut V {
        unsafe { &mut self.elem.as_mut().1 }
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    ///
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    /// Takes the value out of the entry, and returns it.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// // The map is empty
    /// assert!(map.is_empty() && map.capacity() == 0);
    ///
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// // Now map hold none elements
    /// assert!(map.is_empty());
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> V {
        self.remove_entry().1
    }

    /// Provides shared access to the key and owned access to the value of
    /// the entry and allows to replace or remove it based on the
    /// value of the returned option.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.insert("poneyland", 42);
    ///
    /// let entry = match map.entry("poneyland") {
    ///     Entry::Occupied(e) => {
    ///         e.replace_entry_with(|k, v| {
    ///             assert_eq!(k, &"poneyland");
    ///             assert_eq!(v, 42);
    ///             Some(v + 1)
    ///         })
    ///     }
    ///     Entry::Vacant(_) => panic!(),
    /// };
    ///
    /// match entry {
    ///     Entry::Occupied(e) => {
    ///         assert_eq!(e.key(), &"poneyland");
    ///         assert_eq!(e.get(), &43);
    ///     }
    ///     Entry::Vacant(_) => panic!(),
    /// }
    ///
    /// assert_eq!(map["poneyland"], 43);
    ///
    /// let entry = match map.entry("poneyland") {
    ///     Entry::Occupied(e) => e.replace_entry_with(|_k, _v| None),
    ///     Entry::Vacant(_) => panic!(),
    /// };
    ///
    /// match entry {
    ///     Entry::Vacant(e) => {
    ///         assert_eq!(e.key(), &"poneyland");
    ///     }
    ///     Entry::Occupied(_) => panic!(),
    /// }
    ///
    /// assert!(!map.contains_key("poneyland"));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn replace_entry_with<F>(self, f: F) -> Entry<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        unsafe {
            let mut spare_key = None;

            self.table
                .table
                .replace_bucket_with(self.elem.clone(), |(key, value)| {
                    if let Some(new_value) = f(&key, value) {
                        Some((key, new_value))
                    } else {
                        spare_key = Some(key);
                        None
                    }
                });

            if let Some(key) = spare_key {
                Entry::Vacant(VacantEntry {
                    hash: self.hash,
                    key,
                    table: self.table,
                })
            } else {
                Entry::Occupied(self)
            }
        }
    }
}

impl<'a, K, V, S, A: Allocator> VacantEntry<'a, K, V, S, A> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// match map.entry("poneyland") {
    ///     Entry::Occupied(_) => panic!(),
    ///     Entry::Vacant(v) => assert_eq!(v.into_key(), "poneyland"),
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with the [`VacantEntry`]'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        let table = &mut self.table.table;
        let entry = table.insert_entry(
            self.hash,
            (self.key, value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        &mut entry.1
    }

    /// Sets the value of the entry with the [`VacantEntry`]'s key,
    /// and returns an [`OccupiedEntry`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     let o = v.insert_entry(37);
    ///     assert_eq!(o.get(), &37);
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {
        let elem = self.table.table.insert(
            self.hash,
            (self.key, value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        OccupiedEntry {
            hash: self.hash,
            elem,
            table: self.table,
        }
    }
}

impl<'a, 'b, K, Q: ?Sized, V, S, A: Allocator> EntryRef<'a, 'b, K, Q, V, S, A> {
    /// Sets the value of the entry, and returns an `OccupiedEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    /// let entry = map.entry_ref("horseyland").insert(37);
    ///
    /// assert_eq!(entry.key(), "horseyland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            EntryRef::Vacant(entry) => entry.insert_entry(value),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry_ref("poneyland").or_insert(3);
    /// assert_eq!(map["poneyland"], 3);
    ///
    /// // existing key
    /// *map.entry_ref("poneyland").or_insert(10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert(self, default: V) -> &'a mut V
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry_ref("poneyland").or_insert_with(|| 3);
    /// assert_eq!(map["poneyland"], 3);
    ///
    /// // existing key
    /// *map.entry_ref("poneyland").or_insert_with(|| 10) *= 2;
    /// assert_eq!(map["poneyland"], 6);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function an access to the borrower form of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, usize> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());
    /// assert_eq!(map["poneyland"], 9);
    ///
    /// // existing key
    /// *map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;
    /// assert_eq!(map["poneyland"], 18);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert_with_key<F: FnOnce(&Q) -> V>(self, default: F) -> &'a mut V
    where
        K: Hash + Borrow<Q>,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => {
                let value = default(entry.key);
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    /// map.entry_ref("poneyland").or_insert(3);
    /// // existing key
    /// assert_eq!(map.entry_ref("poneyland").key(), "poneyland");
    /// // nonexistent key
    /// assert_eq!(map.entry_ref("horseland").key(), "horseland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &Q
    where
        K: Borrow<Q>,
    {
        match *self {
            EntryRef::Occupied(ref entry) => entry.key().borrow(),
            EntryRef::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    ///
    /// map.entry_ref("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry_ref("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            EntryRef::Occupied(mut entry) => {
                f(entry.get_mut());
                EntryRef::Occupied(entry)
            }
            EntryRef::Vacant(entry) => EntryRef::Vacant(entry),
        }
    }
}

impl<'a, 'b, K, Q: ?Sized, V: Default, S, A: Allocator> EntryRef<'a, 'b, K, Q, V, S, A> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, Option<u32>> = HashMap::new();
    ///
    /// // nonexistent key
    /// map.entry_ref("poneyland").or_default();
    /// assert_eq!(map["poneyland"], None);
    ///
    /// map.insert("horseland".to_string(), Some(3));
    ///
    /// // existing key
    /// assert_eq!(map.entry_ref("horseland").or_default(), &mut Some(3));
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_default(self) -> &'a mut V
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

impl<'a, 'b, K, Q: ?Sized, V, S, A: Allocator> VacantEntryRef<'a, 'b, K, Q, V, S, A> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntryRef`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    /// let key: &str = "poneyland";
    /// assert_eq!(map.entry_ref(key).key(), "poneyland");
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &'b Q {
        self.key
    }

    /// Sets the value of the entry with the `VacantEntryRef`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::EntryRef;
    ///
    /// let mut map: HashMap<String, u32> = HashMap::new();
    /// let key: &str = "poneyland";
    ///
    /// if let EntryRef::Vacant(o) = map.entry_ref(key) {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        let table = &mut self.table.table;
        let entry = table.insert_entry(
            self.hash,
            (self.key.into(), value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        &mut entry.1
    }

    /// Sets the value of the entry with the [`VacantEntryRef`]'s key,
    /// and returns an [`OccupiedEntry`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::HashMap;
    /// use hashbrown::hash_map::EntryRef;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let EntryRef::Vacant(v) = map.entry_ref("poneyland") {
    ///     let o = v.insert_entry(37);
    ///     assert_eq!(o.get(), &37);
    /// }
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash,
        &'b Q: Into<K>,
        S: BuildHasher,
    {
        let elem = self.table.table.insert(
            self.hash,
            (self.key.into(), value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        OccupiedEntry {
            hash: self.hash,
            elem,
            table: self.table,
        }
    }
}

impl<K, V, S, A> FromIterator<(K, V)> for HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
    A: Default + Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut map =
            Self::with_capacity_and_hasher_in(iter.size_hint().0, S::default(), A::default());
        iter.for_each(|(k, v)| {
            map.insert(k, v);
        });
        map
    }
}

/// Inserts all new key-values from the iterator and replaces values with existing
/// keys with new values returned from the iterator.
impl<K, V, S, A> Extend<(K, V)> for HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    /// Inserts all new key-values from the iterator to existing `HashMap<K, V, S, A>`.
    /// Replace values with existing keys with new values returned from the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, 100);
    ///
    /// let some_iter = [(1, 1), (2, 2)].into_iter();
    /// map.extend(some_iter);
    /// // Replace values with existing keys with new values returned from the iterator.
    /// // So that the map.get(&1) doesn't return Some(&100).
    /// assert_eq!(map.get(&1), Some(&1));
    ///
    /// let some_vec: Vec<_> = vec![(3, 3), (4, 4)];
    /// map.extend(some_vec);
    ///
    /// let some_arr = [(5, 5), (6, 6)];
    /// map.extend(some_arr);
    /// let old_map_len = map.len();
    ///
    /// // You can also extend from another HashMap
    /// let mut new_map = HashMap::new();
    /// new_map.extend(map);
    /// assert_eq!(new_map.len(), old_map_len);
    ///
    /// let mut vec: Vec<_> = new_map.into_iter().collect();
    /// // The `IntoIter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty.
        // Otherwise reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case.
        let iter = iter.into_iter();
        let reserve = if self.is_empty() {
            iter.size_hint().0
        } else {
            (iter.size_hint().0 + 1) / 2
        };
        self.reserve(reserve);
        iter.for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (K, V)) {
        self.insert(k, v);
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_reserve(&mut self, additional: usize) {
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty.
        // Otherwise reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case.
        let reserve = if self.is_empty() {
            additional
        } else {
            (additional + 1) / 2
        };
        self.reserve(reserve);
    }
}

/// Inserts all new key-values from the iterator and replaces values with existing
/// keys with new values returned from the iterator.
impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for HashMap<K, V, S, A>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
    A: Allocator,
{
    /// Inserts all new key-values from the iterator to existing `HashMap<K, V, S, A>`.
    /// Replace values with existing keys with new values returned from the iterator.
    /// The keys and values must implement [`Copy`] trait.
    ///
    /// [`Copy`]: https://doc.rust-lang.org/core/marker/trait.Copy.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, 100);
    ///
    /// let arr = [(1, 1), (2, 2)];
    /// let some_iter = arr.iter().map(|(k, v)| (k, v));
    /// map.extend(some_iter);
    /// // Replace values with existing keys with new values returned from the iterator.
    /// // So that the map.get(&1) doesn't return Some(&100).
    /// assert_eq!(map.get(&1), Some(&1));
    ///
    /// let some_vec: Vec<_> = vec![(3, 3), (4, 4)];
    /// map.extend(some_vec.iter().map(|(k, v)| (k, v)));
    ///
    /// let some_arr = [(5, 5), (6, 6)];
    /// map.extend(some_arr.iter().map(|(k, v)| (k, v)));
    ///
    /// // You can also extend from another HashMap
    /// let mut new_map = HashMap::new();
    /// new_map.extend(&map);
    /// assert_eq!(new_map, map);
    ///
    /// let mut vec: Vec<_> = new_map.into_iter().collect();
    /// // The `IntoIter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (&'a K, &'a V)) {
        self.insert(*k, *v);
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_reserve(&mut self, additional: usize) {
        Extend::<(K, V)>::extend_reserve(self, additional);
    }
}

/// Inserts all new key-values from the iterator and replaces values with existing
/// keys with new values returned from the iterator.
impl<'a, K, V, S, A> Extend<&'a (K, V)> for HashMap<K, V, S, A>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
    A: Allocator,
{
    /// Inserts all new key-values from the iterator to existing `HashMap<K, V, S, A>`.
    /// Replace values with existing keys with new values returned from the iterator.
    /// The keys and values must implement [`Copy`] trait.
    ///
    /// [`Copy`]: https://doc.rust-lang.org/core/marker/trait.Copy.html
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, 100);
    ///
    /// let arr = [(1, 1), (2, 2)];
    /// let some_iter = arr.iter();
    /// map.extend(some_iter);
    /// // Replace values with existing keys with new values returned from the iterator.
    /// // So that the map.get(&1) doesn't return Some(&100).
    /// assert_eq!(map.get(&1), Some(&1));
    ///
    /// let some_vec: Vec<_> = vec![(3, 3), (4, 4)];
    /// map.extend(&some_vec);
    ///
    /// let some_arr = [(5, 5), (6, 6)];
    /// map.extend(&some_arr);
    ///
    /// let mut vec: Vec<_> = map.into_iter().collect();
    /// // The `IntoIter` iterator produces items in arbitrary order, so the
    /// // items must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);
    /// ```
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<T: IntoIterator<Item = &'a (K, V)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|&(key, value)| (key, value)));
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, &(k, v): &'a (K, V)) {
        self.insert(k, v);
    }

    #[inline]
    #[cfg(feature = "nightly")]
    fn extend_reserve(&mut self, additional: usize) {
        Extend::<(K, V)>::extend_reserve(self, additional);
    }
}

#[allow(dead_code)]
fn assert_covariance() {
    fn map_key<'new>(v: HashMap<&'static str, u8>) -> HashMap<&'new str, u8> {
        v
    }
    fn map_val<'new>(v: HashMap<u8, &'static str>) -> HashMap<u8, &'new str> {
        v
    }
    fn iter_key<'a, 'new>(v: Iter<'a, &'static str, u8>) -> Iter<'a, &'new str, u8> {
        v
    }
    fn iter_val<'a, 'new>(v: Iter<'a, u8, &'static str>) -> Iter<'a, u8, &'new str> {
        v
    }
    fn into_iter_key<'new, A: Allocator>(
        v: IntoIter<&'static str, u8, A>,
    ) -> IntoIter<&'new str, u8, A> {
        v
    }
    fn into_iter_val<'new, A: Allocator>(
        v: IntoIter<u8, &'static str, A>,
    ) -> IntoIter<u8, &'new str, A> {
        v
    }
    fn keys_key<'a, 'new>(v: Keys<'a, &'static str, u8>) -> Keys<'a, &'new str, u8> {
        v
    }
    fn keys_val<'a, 'new>(v: Keys<'a, u8, &'static str>) -> Keys<'a, u8, &'new str> {
        v
    }
    fn values_key<'a, 'new>(v: Values<'a, &'static str, u8>) -> Values<'a, &'new str, u8> {
        v
    }
    fn values_val<'a, 'new>(v: Values<'a, u8, &'static str>) -> Values<'a, u8, &'new str> {
        v
    }
    fn drain<'new>(
        d: Drain<'static, &'static str, &'static str>,
    ) -> Drain<'new, &'new str, &'new str> {
        d
    }
}

#[cfg(test)]
mod test_map {
    use super::DefaultHashBuilder;
    use super::Entry::{Occupied, Vacant};
    use super::EntryRef;
    use super::HashMap;
    use crate::raw::{AllocError, Allocator, Global};
    use alloc::string::{String, ToString};
    use alloc::sync::Arc;
    use core::alloc::Layout;
    use core::ptr::NonNull;
    use core::sync::atomic::{AtomicI8, Ordering};
    use rand::{rngs::SmallRng, Rng, SeedableRng};
    use std::borrow::ToOwned;
    use std::cell::RefCell;
    use std::vec::Vec;

    #[test]
    fn test_zero_capacities() {
        type HM = HashMap<i32, i32>;

        let m = HM::new();
        assert_eq!(m.capacity(), 0);

        let m = HM::default();
        assert_eq!(m.capacity(), 0);

        let m = HM::with_hasher(DefaultHashBuilder::default());
        assert_eq!(m.capacity(), 0);

        let m = HM::with_capacity(0);
        assert_eq!(m.capacity(), 0);

        let m = HM::with_capacity_and_hasher(0, DefaultHashBuilder::default());
        assert_eq!(m.capacity(), 0);

        let mut m = HM::new();
        m.insert(1, 1);
        m.insert(2, 2);
        m.remove(&1);
        m.remove(&2);
        m.shrink_to_fit();
        assert_eq!(m.capacity(), 0);

        let mut m = HM::new();
        m.reserve(0);
        assert_eq!(m.capacity(), 0);
    }

    #[test]
    fn test_create_capacity_zero() {
        let mut m = HashMap::with_capacity(0);

        assert!(m.insert(1, 1).is_none());

        assert!(m.contains_key(&1));
        assert!(!m.contains_key(&0));
    }

    #[test]
    fn test_insert() {
        let mut m = HashMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert(2, 4).is_none());
        assert_eq!(m.len(), 2);
        assert_eq!(*m.get(&1).unwrap(), 2);
        assert_eq!(*m.get(&2).unwrap(), 4);
    }

    #[test]
    fn test_clone() {
        let mut m = HashMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert(2, 4).is_none());
        assert_eq!(m.len(), 2);
        #[allow(clippy::redundant_clone)]
        let m2 = m.clone();
        assert_eq!(*m2.get(&1).unwrap(), 2);
        assert_eq!(*m2.get(&2).unwrap(), 4);
        assert_eq!(m2.len(), 2);
    }

    #[test]
    fn test_clone_from() {
        let mut m = HashMap::new();
        let mut m2 = HashMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert(2, 4).is_none());
        assert_eq!(m.len(), 2);
        m2.clone_from(&m);
        assert_eq!(*m2.get(&1).unwrap(), 2);
        assert_eq!(*m2.get(&2).unwrap(), 4);
        assert_eq!(m2.len(), 2);
    }

    thread_local! { static DROP_VECTOR: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) } }

    #[derive(Hash, PartialEq, Eq)]
    struct Droppable {
        k: usize,
    }

    impl Droppable {
        fn new(k: usize) -> Droppable {
            DROP_VECTOR.with(|slot| {
                slot.borrow_mut()[k] += 1;
            });

            Droppable { k }
        }
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            DROP_VECTOR.with(|slot| {
                slot.borrow_mut()[self.k] -= 1;
            });
        }
    }

    impl Clone for Droppable {
        fn clone(&self) -> Self {
            Droppable::new(self.k)
        }
    }

    #[test]
    fn test_drops() {
        DROP_VECTOR.with(|slot| {
            *slot.borrow_mut() = vec![0; 200];
        });

        {
            let mut m = HashMap::new();

            DROP_VECTOR.with(|v| {
                for i in 0..200 {
                    assert_eq!(v.borrow()[i], 0);
                }
            });

            for i in 0..100 {
                let d1 = Droppable::new(i);
                let d2 = Droppable::new(i + 100);
                m.insert(d1, d2);
            }

            DROP_VECTOR.with(|v| {
                for i in 0..200 {
                    assert_eq!(v.borrow()[i], 1);
                }
            });

            for i in 0..50 {
                let k = Droppable::new(i);
                let v = m.remove(&k);

                assert!(v.is_some());

                DROP_VECTOR.with(|v| {
                    assert_eq!(v.borrow()[i], 1);
                    assert_eq!(v.borrow()[i + 100], 1);
                });
            }

            DROP_VECTOR.with(|v| {
                for i in 0..50 {
                    assert_eq!(v.borrow()[i], 0);
                    assert_eq!(v.borrow()[i + 100], 0);
                }

                for i in 50..100 {
                    assert_eq!(v.borrow()[i], 1);
                    assert_eq!(v.borrow()[i + 100], 1);
                }
            });
        }

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 0);
            }
        });
    }

    #[test]
    fn test_into_iter_drops() {
        DROP_VECTOR.with(|v| {
            *v.borrow_mut() = vec![0; 200];
        });

        let hm = {
            let mut hm = HashMap::new();

            DROP_VECTOR.with(|v| {
                for i in 0..200 {
                    assert_eq!(v.borrow()[i], 0);
                }
            });

            for i in 0..100 {
                let d1 = Droppable::new(i);
                let d2 = Droppable::new(i + 100);
                hm.insert(d1, d2);
            }

            DROP_VECTOR.with(|v| {
                for i in 0..200 {
                    assert_eq!(v.borrow()[i], 1);
                }
            });

            hm
        };

        // By the way, ensure that cloning doesn't screw up the dropping.
        drop(hm.clone());

        {
            let mut half = hm.into_iter().take(50);

            DROP_VECTOR.with(|v| {
                for i in 0..200 {
                    assert_eq!(v.borrow()[i], 1);
                }
            });

            for _ in half.by_ref() {}

            DROP_VECTOR.with(|v| {
                let nk = (0..100).filter(|&i| v.borrow()[i] == 1).count();

                let nv = (0..100).filter(|&i| v.borrow()[i + 100] == 1).count();

                assert_eq!(nk, 50);
                assert_eq!(nv, 50);
            });
        };

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 0);
            }
        });
    }

    #[test]
    fn test_empty_remove() {
        let mut m: HashMap<i32, bool> = HashMap::new();
        assert_eq!(m.remove(&0), None);
    }

    #[test]
    fn test_empty_entry() {
        let mut m: HashMap<i32, bool> = HashMap::new();
        match m.entry(0) {
            Occupied(_) => panic!(),
            Vacant(_) => {}
        }
        assert!(*m.entry(0).or_insert(true));
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_empty_entry_ref() {
        let mut m: HashMap<std::string::String, bool> = HashMap::new();
        match m.entry_ref("poneyland") {
            EntryRef::Occupied(_) => panic!(),
            EntryRef::Vacant(_) => {}
        }
        assert!(*m.entry_ref("poneyland").or_insert(true));
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_empty_iter() {
        let mut m: HashMap<i32, bool> = HashMap::new();
        assert_eq!(m.drain().next(), None);
        assert_eq!(m.keys().next(), None);
        assert_eq!(m.values().next(), None);
        assert_eq!(m.values_mut().next(), None);
        assert_eq!(m.iter().next(), None);
        assert_eq!(m.iter_mut().next(), None);
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
        assert_eq!(m.into_iter().next(), None);
    }

    #[test]
    #[cfg_attr(miri, ignore)] // FIXME: takes too long
    fn test_lots_of_insertions() {
        let mut m = HashMap::new();

        // Try this a few times to make sure we never screw up the hashmap's
        // internal state.
        for _ in 0..10 {
            assert!(m.is_empty());

            for i in 1..1001 {
                assert!(m.insert(i, i).is_none());

                for j in 1..=i {
                    let r = m.get(&j);
                    assert_eq!(r, Some(&j));
                }

                for j in i + 1..1001 {
                    let r = m.get(&j);
                    assert_eq!(r, None);
                }
            }

            for i in 1001..2001 {
                assert!(!m.contains_key(&i));
            }

            // remove forwards
            for i in 1..1001 {
                assert!(m.remove(&i).is_some());

                for j in 1..=i {
                    assert!(!m.contains_key(&j));
                }

                for j in i + 1..1001 {
                    assert!(m.contains_key(&j));
                }
            }

            for i in 1..1001 {
                assert!(!m.contains_key(&i));
            }

            for i in 1..1001 {
                assert!(m.insert(i, i).is_none());
            }

            // remove backwards
            for i in (1..1001).rev() {
                assert!(m.remove(&i).is_some());

                for j in i..1001 {
                    assert!(!m.contains_key(&j));
                }

                for j in 1..i {
                    assert!(m.contains_key(&j));
                }
            }
        }
    }

    #[test]
    fn test_find_mut() {
        let mut m = HashMap::new();
        assert!(m.insert(1, 12).is_none());
        assert!(m.insert(2, 8).is_none());
        assert!(m.insert(5, 14).is_none());
        let new = 100;
        match m.get_mut(&5) {
            None => panic!(),
            Some(x) => *x = new,
        }
        assert_eq!(m.get(&5), Some(&new));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut m = HashMap::new();
        assert!(m.insert(1, 2).is_none());
        assert_eq!(*m.get(&1).unwrap(), 2);
        assert!(m.insert(1, 3).is_some());
        assert_eq!(*m.get(&1).unwrap(), 3);
    }

    #[test]
    fn test_insert_conflicts() {
        let mut m = HashMap::with_capacity(4);
        assert!(m.insert(1, 2).is_none());
        assert!(m.insert(5, 3).is_none());
        assert!(m.insert(9, 4).is_none());
        assert_eq!(*m.get(&9).unwrap(), 4);
        assert_eq!(*m.get(&5).unwrap(), 3);
        assert_eq!(*m.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_conflict_remove() {
        let mut m = HashMap::with_capacity(4);
        assert!(m.insert(1, 2).is_none());
        assert_eq!(*m.get(&1).unwrap(), 2);
        assert!(m.insert(5, 3).is_none());
        assert_eq!(*m.get(&1).unwrap(), 2);
        assert_eq!(*m.get(&5).unwrap(), 3);
        assert!(m.insert(9, 4).is_none());
        assert_eq!(*m.get(&1).unwrap(), 2);
        assert_eq!(*m.get(&5).unwrap(), 3);
        assert_eq!(*m.get(&9).unwrap(), 4);
        assert!(m.remove(&1).is_some());
        assert_eq!(*m.get(&9).unwrap(), 4);
        assert_eq!(*m.get(&5).unwrap(), 3);
    }

    #[test]
    fn test_insert_unique_unchecked() {
        let mut map = HashMap::new();
        let (k1, v1) = unsafe { map.insert_unique_unchecked(10, 11) };
        assert_eq!((&10, &mut 11), (k1, v1));
        let (k2, v2) = unsafe { map.insert_unique_unchecked(20, 21) };
        assert_eq!((&20, &mut 21), (k2, v2));
        assert_eq!(Some(&11), map.get(&10));
        assert_eq!(Some(&21), map.get(&20));
        assert_eq!(None, map.get(&30));
    }

    #[test]
    fn test_is_empty() {
        let mut m = HashMap::with_capacity(4);
        assert!(m.insert(1, 2).is_none());
        assert!(!m.is_empty());
        assert!(m.remove(&1).is_some());
        assert!(m.is_empty());
    }

    #[test]
    fn test_remove() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        assert_eq!(m.remove(&1), Some(2));
        assert_eq!(m.remove(&1), None);
    }

    #[test]
    fn test_remove_entry() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        assert_eq!(m.remove_entry(&1), Some((1, 2)));
        assert_eq!(m.remove(&1), None);
    }

    #[test]
    fn test_iterate() {
        let mut m = HashMap::with_capacity(4);
        for i in 0..32 {
            assert!(m.insert(i, i * 2).is_none());
        }
        assert_eq!(m.len(), 32);

        let mut observed: u32 = 0;

        for (k, v) in &m {
            assert_eq!(*v, *k * 2);
            observed |= 1 << *k;
        }
        assert_eq!(observed, 0xFFFF_FFFF);
    }

    #[test]
    fn test_keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_iter().collect();
        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_iter().collect();
        let values: Vec<_> = map.values().copied().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_values_mut() {
        let vec = vec![(1, 1), (2, 2), (3, 3)];
        let mut map: HashMap<_, _> = vec.into_iter().collect();
        for value in map.values_mut() {
            *value *= 2;
        }
        let values: Vec<_> = map.values().copied().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&2));
        assert!(values.contains(&4));
        assert!(values.contains(&6));
    }

    #[test]
    fn test_into_keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_iter().collect();
        let keys: Vec<_> = map.into_keys().collect();

        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_into_values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: HashMap<_, _> = vec.into_iter().collect();
        let values: Vec<_> = map.into_values().collect();

        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_find() {
        let mut m = HashMap::new();
        assert!(m.get(&1).is_none());
        m.insert(1, 2);
        match m.get(&1) {
            None => panic!(),
            Some(v) => assert_eq!(*v, 2),
        }
    }

    #[test]
    fn test_eq() {
        let mut m1 = HashMap::new();
        m1.insert(1, 2);
        m1.insert(2, 3);
        m1.insert(3, 4);

        let mut m2 = HashMap::new();
        m2.insert(1, 2);
        m2.insert(2, 3);

        assert!(m1 != m2);

        m2.insert(3, 4);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_show() {
        let mut map = HashMap::new();
        let empty: HashMap<i32, i32> = HashMap::new();

        map.insert(1, 2);
        map.insert(3, 4);

        let map_str = format!("{map:?}");

        assert!(map_str == "{1: 2, 3: 4}" || map_str == "{3: 4, 1: 2}");
        assert_eq!(format!("{empty:?}"), "{}");
    }

    #[test]
    fn test_expand() {
        let mut m = HashMap::new();

        assert_eq!(m.len(), 0);
        assert!(m.is_empty());

        let mut i = 0;
        let old_raw_cap = m.raw_capacity();
        while old_raw_cap == m.raw_capacity() {
            m.insert(i, i);
            i += 1;
        }

        assert_eq!(m.len(), i);
        assert!(!m.is_empty());
    }

    #[test]
    fn test_behavior_resize_policy() {
        let mut m = HashMap::new();

        assert_eq!(m.len(), 0);
        assert_eq!(m.raw_capacity(), 1);
        assert!(m.is_empty());

        m.insert(0, 0);
        m.remove(&0);
        assert!(m.is_empty());
        let initial_raw_cap = m.raw_capacity();
        m.reserve(initial_raw_cap);
        let raw_cap = m.raw_capacity();

        assert_eq!(raw_cap, initial_raw_cap * 2);

        let mut i = 0;
        for _ in 0..raw_cap * 3 / 4 {
            m.insert(i, i);
            i += 1;
        }
        // three quarters full

        assert_eq!(m.len(), i);
        assert_eq!(m.raw_capacity(), raw_cap);

        for _ in 0..raw_cap / 4 {
            m.insert(i, i);
            i += 1;
        }
        // half full

        let new_raw_cap = m.raw_capacity();
        assert_eq!(new_raw_cap, raw_cap * 2);

        for _ in 0..raw_cap / 2 - 1 {
            i -= 1;
            m.remove(&i);
            assert_eq!(m.raw_capacity(), new_raw_cap);
        }
        // A little more than one quarter full.
        m.shrink_to_fit();
        assert_eq!(m.raw_capacity(), raw_cap);
        // again, a little more than half full
        for _ in 0..raw_cap / 2 {
            i -= 1;
            m.remove(&i);
        }
        m.shrink_to_fit();

        assert_eq!(m.len(), i);
        assert!(!m.is_empty());
        assert_eq!(m.raw_capacity(), initial_raw_cap);
    }

    #[test]
    fn test_reserve_shrink_to_fit() {
        let mut m = HashMap::new();
        m.insert(0, 0);
        m.remove(&0);
        assert!(m.capacity() >= m.len());
        for i in 0..128 {
            m.insert(i, i);
        }
        m.reserve(256);

        let usable_cap = m.capacity();
        for i in 128..(128 + 256) {
            m.insert(i, i);
            assert_eq!(m.capacity(), usable_cap);
        }

        for i in 100..(128 + 256) {
            assert_eq!(m.remove(&i), Some(i));
        }
        m.shrink_to_fit();

        assert_eq!(m.len(), 100);
        assert!(!m.is_empty());
        assert!(m.capacity() >= m.len());

        for i in 0..100 {
            assert_eq!(m.remove(&i), Some(i));
        }
        m.shrink_to_fit();
        m.insert(0, 0);

        assert_eq!(m.len(), 1);
        assert!(m.capacity() >= m.len());
        assert_eq!(m.remove(&0), Some(0));
    }

    #[test]
    fn test_from_iter() {
        let xs = [(1, 1), (2, 2), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: HashMap<_, _> = xs.iter().copied().collect();

        for &(k, v) in &xs {
            assert_eq!(map.get(&k), Some(&v));
        }

        assert_eq!(map.iter().len(), xs.len() - 1);
    }

    #[test]
    fn test_size_hint() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: HashMap<_, _> = xs.iter().copied().collect();

        let mut iter = map.iter();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_iter_len() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: HashMap<_, _> = xs.iter().copied().collect();

        let mut iter = map.iter();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn test_mut_size_hint() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let mut map: HashMap<_, _> = xs.iter().copied().collect();

        let mut iter = map.iter_mut();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_iter_mut_len() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let mut map: HashMap<_, _> = xs.iter().copied().collect();

        let mut iter = map.iter_mut();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn test_index() {
        let mut map = HashMap::new();

        map.insert(1, 2);
        map.insert(2, 1);
        map.insert(3, 4);

        assert_eq!(map[&2], 1);
    }

    #[test]
    #[should_panic]
    fn test_index_nonexistent() {
        let mut map = HashMap::new();

        map.insert(1, 2);
        map.insert(2, 1);
        map.insert(3, 4);

        #[allow(clippy::no_effect)] // false positive lint
        map[&4];
    }

    #[test]
    fn test_entry() {
        let xs = [(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)];

        let mut map: HashMap<_, _> = xs.iter().copied().collect();

        // Existing key (insert)
        match map.entry(1) {
            Vacant(_) => unreachable!(),
            Occupied(mut view) => {
                assert_eq!(view.get(), &10);
                assert_eq!(view.insert(100), 10);
            }
        }
        assert_eq!(map.get(&1).unwrap(), &100);
        assert_eq!(map.len(), 6);

        // Existing key (update)
        match map.entry(2) {
            Vacant(_) => unreachable!(),
            Occupied(mut view) => {
                let v = view.get_mut();
                let new_v = (*v) * 10;
                *v = new_v;
            }
        }
        assert_eq!(map.get(&2).unwrap(), &200);
        assert_eq!(map.len(), 6);

        // Existing key (take)
        match map.entry(3) {
            Vacant(_) => unreachable!(),
            Occupied(view) => {
                assert_eq!(view.remove(), 30);
            }
        }
        assert_eq!(map.get(&3), None);
        assert_eq!(map.len(), 5);

        // Inexistent key (insert)
        match map.entry(10) {
            Occupied(_) => unreachable!(),
            Vacant(view) => {
                assert_eq!(*view.insert(1000), 1000);
            }
        }
        assert_eq!(map.get(&10).unwrap(), &1000);
        assert_eq!(map.len(), 6);
    }

    #[test]
    fn test_entry_ref() {
        let xs = [
            ("One".to_owned(), 10),
            ("Two".to_owned(), 20),
            ("Three".to_owned(), 30),
            ("Four".to_owned(), 40),
            ("Five".to_owned(), 50),
            ("Six".to_owned(), 60),
        ];

        let mut map: HashMap<_, _> = xs.iter().cloned().collect();

        // Existing key (insert)
        match map.entry_ref("One") {
            EntryRef::Vacant(_) => unreachable!(),
            EntryRef::Occupied(mut view) => {
                assert_eq!(view.get(), &10);
                assert_eq!(view.insert(100), 10);
            }
        }
        assert_eq!(map.get("One").unwrap(), &100);
        assert_eq!(map.len(), 6);

        // Existing key (update)
        match map.entry_ref("Two") {
            EntryRef::Vacant(_) => unreachable!(),
            EntryRef::Occupied(mut view) => {
                let v = view.get_mut();
                let new_v = (*v) * 10;
                *v = new_v;
            }
        }
        assert_eq!(map.get("Two").unwrap(), &200);
        assert_eq!(map.len(), 6);

        // Existing key (take)
        match map.entry_ref("Three") {
            EntryRef::Vacant(_) => unreachable!(),
            EntryRef::Occupied(view) => {
                assert_eq!(view.remove(), 30);
            }
        }
        assert_eq!(map.get("Three"), None);
        assert_eq!(map.len(), 5);

        // Inexistent key (insert)
        match map.entry_ref("Ten") {
            EntryRef::Occupied(_) => unreachable!(),
            EntryRef::Vacant(view) => {
                assert_eq!(*view.insert(1000), 1000);
            }
        }
        assert_eq!(map.get("Ten").unwrap(), &1000);
        assert_eq!(map.len(), 6);
    }

    #[test]
    fn test_entry_take_doesnt_corrupt() {
        #![allow(deprecated)] //rand
                              // Test for #19292
        fn check(m: &HashMap<i32, ()>) {
            for k in m.keys() {
                assert!(m.contains_key(k), "{k} is in keys() but not in the map?");
            }
        }

        let mut m = HashMap::new();

        let mut rng = {
            let seed = u64::from_le_bytes(*b"testseed");
            SmallRng::seed_from_u64(seed)
        };

        // Populate the map with some items.
        for _ in 0..50 {
            let x = rng.gen_range(-10..10);
            m.insert(x, ());
        }

        for _ in 0..1000 {
            let x = rng.gen_range(-10..10);
            match m.entry(x) {
                Vacant(_) => {}
                Occupied(e) => {
                    e.remove();
                }
            }

            check(&m);
        }
    }

    #[test]
    fn test_entry_ref_take_doesnt_corrupt() {
        #![allow(deprecated)] //rand
                              // Test for #19292
        fn check(m: &HashMap<std::string::String, ()>) {
            for k in m.keys() {
                assert!(m.contains_key(k), "{k} is in keys() but not in the map?");
            }
        }

        let mut m = HashMap::new();

        let mut rng = {
            let seed = u64::from_le_bytes(*b"testseed");
            SmallRng::seed_from_u64(seed)
        };

        // Populate the map with some items.
        for _ in 0..50 {
            let mut x = std::string::String::with_capacity(1);
            x.push(rng.gen_range('a'..='z'));
            m.insert(x, ());
        }

        for _ in 0..1000 {
            let mut x = std::string::String::with_capacity(1);
            x.push(rng.gen_range('a'..='z'));
            match m.entry_ref(x.as_str()) {
                EntryRef::Vacant(_) => {}
                EntryRef::Occupied(e) => {
                    e.remove();
                }
            }

            check(&m);
        }
    }

    #[test]
    fn test_extend_ref_k_ref_v() {
        let mut a = HashMap::new();
        a.insert(1, "one");
        let mut b = HashMap::new();
        b.insert(2, "two");
        b.insert(3, "three");

        a.extend(&b);

        assert_eq!(a.len(), 3);
        assert_eq!(a[&1], "one");
        assert_eq!(a[&2], "two");
        assert_eq!(a[&3], "three");
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn test_extend_ref_kv_tuple() {
        use std::ops::AddAssign;
        let mut a = HashMap::new();
        a.insert(0, 0);

        fn create_arr<T: AddAssign<T> + Copy, const N: usize>(start: T, step: T) -> [(T, T); N] {
            let mut outs: [(T, T); N] = [(start, start); N];
            let mut element = step;
            outs.iter_mut().skip(1).for_each(|(k, v)| {
                *k += element;
                *v += element;
                element += step;
            });
            outs
        }

        let for_iter: Vec<_> = (0..100).map(|i| (i, i)).collect();
        let iter = for_iter.iter();
        let vec: Vec<_> = (100..200).map(|i| (i, i)).collect();
        a.extend(iter);
        a.extend(&vec);
        a.extend(create_arr::<i32, 100>(200, 1));

        assert_eq!(a.len(), 300);

        for item in 0..300 {
            assert_eq!(a[&item], item);
        }
    }

    #[test]
    fn test_capacity_not_less_than_len() {
        let mut a = HashMap::new();
        let mut item = 0;

        for _ in 0..116 {
            a.insert(item, 0);
            item += 1;
        }

        assert!(a.capacity() > a.len());

        let free = a.capacity() - a.len();
        for _ in 0..free {
            a.insert(item, 0);
            item += 1;
        }

        assert_eq!(a.len(), a.capacity());

        // Insert at capacity should cause allocation.
        a.insert(item, 0);
        assert!(a.capacity() > a.len());
    }

    #[test]
    fn test_occupied_entry_key() {
        let mut a = HashMap::new();
        let key = "hello there";
        let value = "value goes here";
        assert!(a.is_empty());
        a.insert(key, value);
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);

        match a.entry(key) {
            Vacant(_) => panic!(),
            Occupied(e) => assert_eq!(key, *e.key()),
        }
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);
    }

    #[test]
    fn test_occupied_entry_ref_key() {
        let mut a = HashMap::new();
        let key = "hello there";
        let value = "value goes here";
        assert!(a.is_empty());
        a.insert(key.to_owned(), value);
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);

        match a.entry_ref(key) {
            EntryRef::Vacant(_) => panic!(),
            EntryRef::Occupied(e) => assert_eq!(key, e.key()),
        }
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);
    }

    #[test]
    fn test_vacant_entry_key() {
        let mut a = HashMap::new();
        let key = "hello there";
        let value = "value goes here";

        assert!(a.is_empty());
        match a.entry(key) {
            Occupied(_) => panic!(),
            Vacant(e) => {
                assert_eq!(key, *e.key());
                e.insert(value);
            }
        }
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);
    }

    #[test]
    fn test_vacant_entry_ref_key() {
        let mut a: HashMap<std::string::String, &str> = HashMap::new();
        let key = "hello there";
        let value = "value goes here";

        assert!(a.is_empty());
        match a.entry_ref(key) {
            EntryRef::Occupied(_) => panic!(),
            EntryRef::Vacant(e) => {
                assert_eq!(key, e.key());
                e.insert(value);
            }
        }
        assert_eq!(a.len(), 1);
        assert_eq!(a[key], value);
    }

    #[test]
    fn test_occupied_entry_replace_entry_with() {
        let mut a = HashMap::new();

        let key = "a key";
        let value = "an initial value";
        let new_value = "a new value";

        let entry = a.entry(key).insert(value).replace_entry_with(|k, v| {
            assert_eq!(k, &key);
            assert_eq!(v, value);
            Some(new_value)
        });

        match entry {
            Occupied(e) => {
                assert_eq!(e.key(), &key);
                assert_eq!(e.get(), &new_value);
            }
            Vacant(_) => panic!(),
        }

        assert_eq!(a[key], new_value);
        assert_eq!(a.len(), 1);

        let entry = match a.entry(key) {
            Occupied(e) => e.replace_entry_with(|k, v| {
                assert_eq!(k, &key);
                assert_eq!(v, new_value);
                None
            }),
            Vacant(_) => panic!(),
        };

        match entry {
            Vacant(e) => assert_eq!(e.key(), &key),
            Occupied(_) => panic!(),
        }

        assert!(!a.contains_key(key));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn test_entry_and_replace_entry_with() {
        let mut a = HashMap::new();

        let key = "a key";
        let value = "an initial value";
        let new_value = "a new value";

        let entry = a.entry(key).and_replace_entry_with(|_, _| panic!());

        match entry {
            Vacant(e) => assert_eq!(e.key(), &key),
            Occupied(_) => panic!(),
        }

        a.insert(key, value);

        let entry = a.entry(key).and_replace_entry_with(|k, v| {
            assert_eq!(k, &key);
            assert_eq!(v, value);
            Some(new_value)
        });

        match entry {
            Occupied(e) => {
                assert_eq!(e.key(), &key);
                assert_eq!(e.get(), &new_value);
            }
            Vacant(_) => panic!(),
        }

        assert_eq!(a[key], new_value);
        assert_eq!(a.len(), 1);

        let entry = a.entry(key).and_replace_entry_with(|k, v| {
            assert_eq!(k, &key);
            assert_eq!(v, new_value);
            None
        });

        match entry {
            Vacant(e) => assert_eq!(e.key(), &key),
            Occupied(_) => panic!(),
        }

        assert!(!a.contains_key(key));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn test_replace_entry_with_doesnt_corrupt() {
        #![allow(deprecated)] //rand
                              // Test for #19292
        fn check(m: &HashMap<i32, ()>) {
            for k in m.keys() {
                assert!(m.contains_key(k), "{k} is in keys() but not in the map?");
            }
        }

        let mut m = HashMap::new();

        let mut rng = {
            let seed = u64::from_le_bytes(*b"testseed");
            SmallRng::seed_from_u64(seed)
        };

        // Populate the map with some items.
        for _ in 0..50 {
            let x = rng.gen_range(-10..10);
            m.insert(x, ());
        }

        for _ in 0..1000 {
            let x = rng.gen_range(-10..10);
            m.entry(x).and_replace_entry_with(|_, _| None);
            check(&m);
        }
    }

    #[test]
    fn test_retain() {
        let mut map: HashMap<i32, i32> = (0..100).map(|x| (x, x * 10)).collect();

        map.retain(|&k, _| k % 2 == 0);
        assert_eq!(map.len(), 50);
        assert_eq!(map[&2], 20);
        assert_eq!(map[&4], 40);
        assert_eq!(map[&6], 60);
    }

    #[test]
    fn test_extract_if() {
        {
            let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
            let drained = map.extract_if(|&k, _| k % 2 == 0);
            let mut out = drained.collect::<Vec<_>>();
            out.sort_unstable();
            assert_eq!(vec![(0, 0), (2, 20), (4, 40), (6, 60)], out);
            assert_eq!(map.len(), 4);
        }
        {
            let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
            map.extract_if(|&k, _| k % 2 == 0).for_each(drop);
            assert_eq!(map.len(), 4);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)] // FIXME: no OOM signalling (https://github.com/rust-lang/miri/issues/613)
    fn test_try_reserve() {
        use crate::TryReserveError::{AllocError, CapacityOverflow};

        const MAX_ISIZE: usize = isize::MAX as usize;

        let mut empty_bytes: HashMap<u8, u8> = HashMap::new();

        if let Err(CapacityOverflow) = empty_bytes.try_reserve(usize::MAX) {
        } else {
            panic!("usize::MAX should trigger an overflow!");
        }

        if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_ISIZE) {
        } else {
            panic!("isize::MAX should trigger an overflow!");
        }

        if let Err(AllocError { .. }) = empty_bytes.try_reserve(MAX_ISIZE / 5) {
        } else {
            // This may succeed if there is enough free memory. Attempt to
            // allocate a few more hashmaps to ensure the allocation will fail.
            let mut empty_bytes2: HashMap<u8, u8> = HashMap::new();
            let _ = empty_bytes2.try_reserve(MAX_ISIZE / 5);
            let mut empty_bytes3: HashMap<u8, u8> = HashMap::new();
            let _ = empty_bytes3.try_reserve(MAX_ISIZE / 5);
            let mut empty_bytes4: HashMap<u8, u8> = HashMap::new();
            if let Err(AllocError { .. }) = empty_bytes4.try_reserve(MAX_ISIZE / 5) {
            } else {
                panic!("isize::MAX / 5 should trigger an OOM!");
            }
        }
    }

    #[test]
    fn test_const_with_hasher() {
        use core::hash::BuildHasher;
        use std::collections::hash_map::DefaultHasher;

        #[derive(Clone)]
        struct MyHasher;
        impl BuildHasher for MyHasher {
            type Hasher = DefaultHasher;

            fn build_hasher(&self) -> DefaultHasher {
                DefaultHasher::new()
            }
        }

        const EMPTY_MAP: HashMap<u32, std::string::String, MyHasher> =
            HashMap::with_hasher(MyHasher);

        let mut map = EMPTY_MAP;
        map.insert(17, "seventeen".to_owned());
        assert_eq!("seventeen", map[&17]);
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = HashMap::new();
        map.insert("foo".to_owned(), 0);
        map.insert("bar".to_owned(), 10);
        map.insert("baz".to_owned(), 20);
        map.insert("qux".to_owned(), 30);

        let xs = map.get_many_mut(["foo", "qux"]);
        assert_eq!(xs, [Some(&mut 0), Some(&mut 30)]);

        let xs = map.get_many_mut(["foo", "dud"]);
        assert_eq!(xs, [Some(&mut 0), None]);

        let ys = map.get_many_key_value_mut(["bar", "baz"]);
        assert_eq!(
            ys,
            [
                Some((&"bar".to_owned(), &mut 10)),
                Some((&"baz".to_owned(), &mut 20))
            ],
        );

        let ys = map.get_many_key_value_mut(["bar", "dip"]);
        assert_eq!(ys, [Some((&"bar".to_string(), &mut 10)), None]);
    }

    #[test]
    #[should_panic = "duplicate keys found"]
    fn test_get_many_mut_duplicate() {
        let mut map = HashMap::new();
        map.insert("foo".to_owned(), 0);

        let _xs = map.get_many_mut(["foo", "foo"]);
    }

    #[test]
    #[should_panic = "panic in drop"]
    fn test_clone_from_double_drop() {
        #[derive(Clone)]
        struct CheckedDrop {
            panic_in_drop: bool,
            dropped: bool,
        }
        impl Drop for CheckedDrop {
            fn drop(&mut self) {
                if self.panic_in_drop {
                    self.dropped = true;
                    panic!("panic in drop");
                }
                if self.dropped {
                    panic!("double drop");
                }
                self.dropped = true;
            }
        }
        const DISARMED: CheckedDrop = CheckedDrop {
            panic_in_drop: false,
            dropped: false,
        };
        const ARMED: CheckedDrop = CheckedDrop {
            panic_in_drop: true,
            dropped: false,
        };

        let mut map1 = HashMap::new();
        map1.insert(1, DISARMED);
        map1.insert(2, DISARMED);
        map1.insert(3, DISARMED);
        map1.insert(4, DISARMED);

        let mut map2 = HashMap::new();
        map2.insert(1, DISARMED);
        map2.insert(2, ARMED);
        map2.insert(3, DISARMED);
        map2.insert(4, DISARMED);

        map2.clone_from(&map1);
    }

    #[test]
    #[should_panic = "panic in clone"]
    fn test_clone_from_memory_leaks() {
        use alloc::vec::Vec;

        struct CheckedClone {
            panic_in_clone: bool,
            need_drop: Vec<i32>,
        }
        impl Clone for CheckedClone {
            fn clone(&self) -> Self {
                if self.panic_in_clone {
                    panic!("panic in clone")
                }
                Self {
                    panic_in_clone: self.panic_in_clone,
                    need_drop: self.need_drop.clone(),
                }
            }
        }
        let mut map1 = HashMap::new();
        map1.insert(
            1,
            CheckedClone {
                panic_in_clone: false,
                need_drop: vec![0, 1, 2],
            },
        );
        map1.insert(
            2,
            CheckedClone {
                panic_in_clone: false,
                need_drop: vec![3, 4, 5],
            },
        );
        map1.insert(
            3,
            CheckedClone {
                panic_in_clone: true,
                need_drop: vec![6, 7, 8],
            },
        );
        let _map2 = map1.clone();
    }

    struct MyAllocInner {
        drop_count: Arc<AtomicI8>,
    }

    #[derive(Clone)]
    struct MyAlloc {
        _inner: Arc<MyAllocInner>,
    }

    impl MyAlloc {
        fn new(drop_count: Arc<AtomicI8>) -> Self {
            MyAlloc {
                _inner: Arc::new(MyAllocInner { drop_count }),
            }
        }
    }

    impl Drop for MyAllocInner {
        fn drop(&mut self) {
            println!("MyAlloc freed.");
            self.drop_count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    unsafe impl Allocator for MyAlloc {
        fn allocate(&self, layout: Layout) -> std::result::Result<NonNull<[u8]>, AllocError> {
            let g = Global;
            g.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            let g = Global;
            g.deallocate(ptr, layout)
        }
    }

    #[test]
    fn test_hashmap_into_iter_bug() {
        let dropped: Arc<AtomicI8> = Arc::new(AtomicI8::new(1));

        {
            let mut map = HashMap::with_capacity_in(10, MyAlloc::new(dropped.clone()));
            for i in 0..10 {
                map.entry(i).or_insert_with(|| "i".to_string());
            }

            for (k, v) in map {
                println!("{}, {}", k, v);
            }
        }

        // All allocator clones should already be dropped.
        assert_eq!(dropped.load(Ordering::SeqCst), 0);
    }

    #[derive(Debug)]
    struct CheckedCloneDrop<T> {
        panic_in_clone: bool,
        panic_in_drop: bool,
        dropped: bool,
        data: T,
    }

    impl<T> CheckedCloneDrop<T> {
        fn new(panic_in_clone: bool, panic_in_drop: bool, data: T) -> Self {
            CheckedCloneDrop {
                panic_in_clone,
                panic_in_drop,
                dropped: false,
                data,
            }
        }
    }

    impl<T: Clone> Clone for CheckedCloneDrop<T> {
        fn clone(&self) -> Self {
            if self.panic_in_clone {
                panic!("panic in clone")
            }
            Self {
                panic_in_clone: self.panic_in_clone,
                panic_in_drop: self.panic_in_drop,
                dropped: self.dropped,
                data: self.data.clone(),
            }
        }
    }

    impl<T> Drop for CheckedCloneDrop<T> {
        fn drop(&mut self) {
            if self.panic_in_drop {
                self.dropped = true;
                panic!("panic in drop");
            }
            if self.dropped {
                panic!("double drop");
            }
            self.dropped = true;
        }
    }

    /// Return hashmap with predefined distribution of elements.
    /// All elements will be located in the same order as elements
    /// returned by iterator.
    ///
    /// This function does not panic, but returns an error as a `String`
    /// to distinguish between a test panic and an error in the input data.
    fn get_test_map<I, T, A>(
        iter: I,
        mut fun: impl FnMut(u64) -> T,
        alloc: A,
    ) -> Result<HashMap<u64, CheckedCloneDrop<T>, DefaultHashBuilder, A>, String>
    where
        I: Iterator<Item = (bool, bool)> + Clone + ExactSizeIterator,
        A: Allocator,
        T: PartialEq + core::fmt::Debug,
    {
        use crate::scopeguard::guard;

        let mut map: HashMap<u64, CheckedCloneDrop<T>, _, A> =
            HashMap::with_capacity_in(iter.size_hint().0, alloc);
        {
            let mut guard = guard(&mut map, |map| {
                for (_, value) in map.iter_mut() {
                    value.panic_in_drop = false
                }
            });

            let mut count = 0;
            // Hash and Key must be equal to each other for controlling the elements placement.
            for (panic_in_clone, panic_in_drop) in iter.clone() {
                if core::mem::needs_drop::<T>() && panic_in_drop {
                    return Err(String::from(
                        "panic_in_drop can be set with a type that doesn't need to be dropped",
                    ));
                }
                guard.table.insert(
                    count,
                    (
                        count,
                        CheckedCloneDrop::new(panic_in_clone, panic_in_drop, fun(count)),
                    ),
                    |(k, _)| *k,
                );
                count += 1;
            }

            // Let's check that all elements are located as we wanted
            let mut check_count = 0;
            for ((key, value), (panic_in_clone, panic_in_drop)) in guard.iter().zip(iter) {
                if *key != check_count {
                    return Err(format!(
                        "key != check_count,\nkey: `{}`,\ncheck_count: `{}`",
                        key, check_count
                    ));
                }
                if value.dropped
                    || value.panic_in_clone != panic_in_clone
                    || value.panic_in_drop != panic_in_drop
                    || value.data != fun(check_count)
                {
                    return Err(format!(
                        "Value is not equal to expected,\nvalue: `{:?}`,\nexpected: \
                        `CheckedCloneDrop {{ panic_in_clone: {}, panic_in_drop: {}, dropped: {}, data: {:?} }}`",
                        value, panic_in_clone, panic_in_drop, false, fun(check_count)
                    ));
                }
                check_count += 1;
            }

            if guard.len() != check_count as usize {
                return Err(format!(
                    "map.len() != check_count,\nmap.len(): `{}`,\ncheck_count: `{}`",
                    guard.len(),
                    check_count
                ));
            }

            if count != check_count {
                return Err(format!(
                    "count != check_count,\ncount: `{}`,\ncheck_count: `{}`",
                    count, check_count
                ));
            }
            core::mem::forget(guard);
        }
        Ok(map)
    }

    const DISARMED: bool = false;
    const ARMED: bool = true;

    const ARMED_FLAGS: [bool; 8] = [
        DISARMED, DISARMED, DISARMED, ARMED, DISARMED, DISARMED, DISARMED, DISARMED,
    ];

    const DISARMED_FLAGS: [bool; 8] = [
        DISARMED, DISARMED, DISARMED, DISARMED, DISARMED, DISARMED, DISARMED, DISARMED,
    ];

    #[test]
    #[should_panic = "panic in clone"]
    fn test_clone_memory_leaks_and_double_drop_one() {
        let dropped: Arc<AtomicI8> = Arc::new(AtomicI8::new(2));

        {
            assert_eq!(ARMED_FLAGS.len(), DISARMED_FLAGS.len());

            let map: HashMap<u64, CheckedCloneDrop<Vec<u64>>, DefaultHashBuilder, MyAlloc> =
                match get_test_map(
                    ARMED_FLAGS.into_iter().zip(DISARMED_FLAGS),
                    |n| vec![n],
                    MyAlloc::new(dropped.clone()),
                ) {
                    Ok(map) => map,
                    Err(msg) => panic!("{msg}"),
                };

            // Clone should normally clone a few elements, and then (when the
            // clone function panics), deallocate both its own memory, memory
            // of `dropped: Arc<AtomicI8>` and the memory of already cloned
            // elements (Vec<i32> memory inside CheckedCloneDrop).
            let _map2 = map.clone();
        }
    }

    #[test]
    #[should_panic = "panic in drop"]
    fn test_clone_memory_leaks_and_double_drop_two() {
        let dropped: Arc<AtomicI8> = Arc::new(AtomicI8::new(2));

        {
            assert_eq!(ARMED_FLAGS.len(), DISARMED_FLAGS.len());

            let map: HashMap<u64, CheckedCloneDrop<u64>, DefaultHashBuilder, _> = match get_test_map(
                DISARMED_FLAGS.into_iter().zip(DISARMED_FLAGS),
                |n| n,
                MyAlloc::new(dropped.clone()),
            ) {
                Ok(map) => map,
                Err(msg) => panic!("{msg}"),
            };

            let mut map2 = match get_test_map(
                DISARMED_FLAGS.into_iter().zip(ARMED_FLAGS),
                |n| n,
                MyAlloc::new(dropped.clone()),
            ) {
                Ok(map) => map,
                Err(msg) => panic!("{msg}"),
            };

            // The `clone_from` should try to drop the elements of `map2` without
            // double drop and leaking the allocator. Elements that have not been
            // dropped leak their memory.
            map2.clone_from(&map);
        }
    }

    /// We check that we have a working table if the clone operation from another
    /// thread ended in a panic (when buckets of maps are equal to each other).
    #[test]
    fn test_catch_panic_clone_from_when_len_is_equal() {
        use std::thread;

        let dropped: Arc<AtomicI8> = Arc::new(AtomicI8::new(2));

        {
            assert_eq!(ARMED_FLAGS.len(), DISARMED_FLAGS.len());

            let mut map = match get_test_map(
                DISARMED_FLAGS.into_iter().zip(DISARMED_FLAGS),
                |n| vec![n],
                MyAlloc::new(dropped.clone()),
            ) {
                Ok(map) => map,
                Err(msg) => panic!("{msg}"),
            };

            thread::scope(|s| {
                let result: thread::ScopedJoinHandle<'_, String> = s.spawn(|| {
                    let scope_map =
                        match get_test_map(ARMED_FLAGS.into_iter().zip(DISARMED_FLAGS), |n| vec![n * 2], MyAlloc::new(dropped.clone())) {
                            Ok(map) => map,
                            Err(msg) => return msg,
                        };
                    if map.table.buckets() != scope_map.table.buckets() {
                        return format!(
                            "map.table.buckets() != scope_map.table.buckets(),\nleft: `{}`,\nright: `{}`",
                            map.table.buckets(), scope_map.table.buckets()
                        );
                    }
                    map.clone_from(&scope_map);
                    "We must fail the cloning!!!".to_owned()
                });
                if let Ok(msg) = result.join() {
                    panic!("{msg}")
                }
            });

            // Let's check that all iterators work fine and do not return elements
            // (especially `RawIterRange`, which does not depend on the number of
            // elements in the table, but looks directly at the control bytes)
            //
            // SAFETY: We know for sure that `RawTable` will outlive
            // the returned `RawIter / RawIterRange` iterator.
            assert_eq!(map.len(), 0);
            assert_eq!(map.iter().count(), 0);
            assert_eq!(unsafe { map.table.iter().count() }, 0);
            assert_eq!(unsafe { map.table.iter().iter.count() }, 0);

            for idx in 0..map.table.buckets() {
                let idx = idx as u64;
                assert!(
                    map.table.find(idx, |(k, _)| *k == idx).is_none(),
                    "Index: {idx}"
                );
            }
        }

        // All allocator clones should already be dropped.
        assert_eq!(dropped.load(Ordering::SeqCst), 0);
    }

    /// We check that we have a working table if the clone operation from another
    /// thread ended in a panic (when buckets of maps are not equal to each other).
    #[test]
    fn test_catch_panic_clone_from_when_len_is_not_equal() {
        use std::thread;

        let dropped: Arc<AtomicI8> = Arc::new(AtomicI8::new(2));

        {
            assert_eq!(ARMED_FLAGS.len(), DISARMED_FLAGS.len());

            let mut map = match get_test_map(
                [DISARMED].into_iter().zip([DISARMED]),
                |n| vec![n],
                MyAlloc::new(dropped.clone()),
            ) {
                Ok(map) => map,
                Err(msg) => panic!("{msg}"),
            };

            thread::scope(|s| {
                let result: thread::ScopedJoinHandle<'_, String> = s.spawn(|| {
                    let scope_map = match get_test_map(
                        ARMED_FLAGS.into_iter().zip(DISARMED_FLAGS),
                        |n| vec![n * 2],
                        MyAlloc::new(dropped.clone()),
                    ) {
                        Ok(map) => map,
                        Err(msg) => return msg,
                    };
                    if map.table.buckets() == scope_map.table.buckets() {
                        return format!(
                            "map.table.buckets() == scope_map.table.buckets(): `{}`",
                            map.table.buckets()
                        );
                    }
                    map.clone_from(&scope_map);
                    "We must fail the cloning!!!".to_owned()
                });
                if let Ok(msg) = result.join() {
                    panic!("{msg}")
                }
            });

            // Let's check that all iterators work fine and do not return elements
            // (especially `RawIterRange`, which does not depend on the number of
            // elements in the table, but looks directly at the control bytes)
            //
            // SAFETY: We know for sure that `RawTable` will outlive
            // the returned `RawIter / RawIterRange` iterator.
            assert_eq!(map.len(), 0);
            assert_eq!(map.iter().count(), 0);
            assert_eq!(unsafe { map.table.iter().count() }, 0);
            assert_eq!(unsafe { map.table.iter().iter.count() }, 0);

            for idx in 0..map.table.buckets() {
                let idx = idx as u64;
                assert!(
                    map.table.find(idx, |(k, _)| *k == idx).is_none(),
                    "Index: {idx}"
                );
            }
        }

        // All allocator clones should already be dropped.
        assert_eq!(dropped.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_allocation_info() {
        assert_eq!(HashMap::<(), ()>::new().allocation_size(), 0);
        assert_eq!(HashMap::<u32, u32>::new().allocation_size(), 0);
        assert!(
            HashMap::<u32, u32>::with_capacity(1).allocation_size() > core::mem::size_of::<u32>()
        );
    }
}

#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");
        
        let mut drain = map.drain();
        
        // Since the map has 3 elements, the size hint should be (3, Some(3)).
        let (lower, upper) = drain.size_hint();
        assert_eq!(lower, 3);
        assert_eq!(upper, Some(3));

        drain.next(); // Draining one element

        // After draining one element, the size hint should be (2, Some(2)).
        let (lower, upper) = drain.size_hint();
        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2));

        drain.next(); // Draining another element

        // After draining another element, the size hint should be (1, Some(1)).
        let (lower, upper) = drain.size_hint();
        assert_eq!(lower, 1);
        assert_eq!(upper, Some(1));

        drain.next(); // Draining the last element

        // After draining the last element, the size hint should be (0, Some(0)).
        let (lower, upper) = drain.size_hint();
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0));

        // Check that the drain is fused
        assert_eq!(drain.next(), None);
        assert_eq!(drain.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_21 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_extract_if_next() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        
        {
            let mut extract_if = map.extract_if(|&k, _| k % 2 != 0);
            let mut results = vec![extract_if.next(), extract_if.next()];

            results.sort_unstable();
            assert_eq!(results, vec![Some((1, "a")), Some((3, "c"))]);
        }
        
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_extract_if_next_none() {
        let mut map: HashMap<i32, &str> = [(2, "b")].into();
        
        {
            let mut extract_if = map.extract_if(|&k, _| k % 2 != 0);
            assert_eq!(extract_if.next(), None);
        }
        
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_extract_if_fused() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b")].into();
        
        let mut extract_if = map.extract_if(|&k, _| k % 2 != 0);
        assert_eq!(extract_if.next(), Some((1, "a")));
        assert_eq!(extract_if.next(), None);
        assert_eq!(extract_if.next(), None); // should still return None
    }
}

#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint_empty() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        let extract_if = map.extract_if(|_, _| false);
        assert_eq!(extract_if.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_some_elements() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let extract_if = map.extract_if(|_, _| false);
        assert_eq!(extract_if.size_hint(), (0, Some(3))); // 3 elements exist
    }

    #[test]
    fn test_size_hint_after_extraction() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let mut extract_if = map.extract_if(|k, _| k % 2 != 0); // extract odd keys
        extract_if.next(); // extract one
        assert_eq!(extract_if.size_hint(), (0, Some(2))); // 2 elements remain
    }

    #[test]
    fn test_size_hint_after_extraction_empty() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let mut extract_if = map.extract_if(|k, _| k % 2 != 0); // extract odd keys
        extract_if.next(); // extract one
        extract_if.next(); // extract another
        let _ = extract_if.next(); // extract last
        assert_eq!(extract_if.size_hint(), (0, Some(0))); // no elements remain
    }
}

#[cfg(test)]
mod tests_llm_16_23 {
    use crate::HashMap;

    #[test]
    fn test_clone_empty() {
        let map: HashMap<i32, i32> = HashMap::new();
        let cloned_map = map.clone();
        assert_eq!(map.len(), 0);
        assert_eq!(cloned_map.len(), 0);
        assert_eq!(cloned_map, map);
    }

    #[test]
    fn test_clone_single_entry() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        let cloned_map = map.clone();

        assert_eq!(map.len(), 1);
        assert_eq!(cloned_map.len(), 1);
        assert_eq!(cloned_map, map);
        assert_eq!(cloned_map.get(&1), Some(&2));
    }

    #[test]
    fn test_clone_multiple_entries() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        map.insert(5, 6);
        let cloned_map = map.clone();

        assert_eq!(map.len(), 3);
        assert_eq!(cloned_map.len(), 3);
        assert_eq!(cloned_map, map);
        assert_eq!(cloned_map.get(&3), Some(&4));
    }

    #[test]
    fn test_clone_non_empty_map() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");
        map.insert("baz", "qux");
        
        let cloned_map = map.clone();

        assert_eq!(map.len(), 2);
        assert_eq!(cloned_map.len(), 2);
        assert_eq!(cloned_map.get("foo"), Some(&"bar"));
        assert_eq!(cloned_map.get("baz"), Some(&"qux"));
        assert_eq!(cloned_map, map);
    }

    #[test]
    fn test_clone_and_modify() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        let cloned_map = map.clone();

        map.insert(2, "b");
        
        assert_eq!(map.len(), 2);
        assert_eq!(cloned_map.len(), 1);
        assert_eq!(cloned_map.get(&1), Some(&"a"));
        assert!(cloned_map.get(&2).is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_eq_different_length() {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(1, "a");
        map2.insert(1, "a");
        map2.insert(2, "b");
        assert!(!map1.eq(&map2));
    }

    #[test]
    fn test_eq_different_keys() {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(1, "a");
        map2.insert(2, "a");
        assert!(!map1.eq(&map2));
    }

    #[test]
    fn test_eq_different_values() {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(1, "a");
        map2.insert(1, "b");
        assert!(!map1.eq(&map2));
    }

    #[test]
    fn test_eq_same() {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(1, "a");
        map1.insert(2, "b");
        map2.insert(1, "a");
        map2.insert(2, "b");
        assert!(map1.eq(&map2));
    }

    #[test]
    fn test_eq_with_different_order() {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(1, "a");
        map1.insert(2, "b");
        map2.insert(2, "b");
        map2.insert(1, "a");
        assert!(map1.eq(&map2));
    }
}

#[cfg(test)]
mod tests_llm_16_30 {
    use super::*;

use crate::*;
    use crate::hash_map::HashMap;

    #[test]
    fn test_extend_with_array() {
        let mut map = HashMap::new();
        map.insert(1, 100);

        let some_arr = [(1, 1), (2, 2)];
        map.extend(some_arr.iter().cloned());
        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&2), Some(&2));
    }

    #[test]
    fn test_extend_with_vec() {
        let mut map = HashMap::new();
        map.insert(1, 100);

        let some_vec = vec![(1, 1), (2, 2)];
        map.extend(some_vec.iter().cloned());
        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&2), Some(&2));
    }

    #[test]
    fn test_extend_from_another_hashmap() {
        let mut map1 = HashMap::new();
        map1.insert(1, 100);
        map1.insert(2, 200);

        let mut map2 = HashMap::new();
        map2.extend(map1);
        assert_eq!(map2.get(&1), Some(&100));
        assert_eq!(map2.get(&2), Some(&200));
    }

    #[test]
    fn test_extend_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert(1, 100);
        map.insert(2, 200);

        let mixed_iter = vec![(1, 150), (3, 300)];
        map.extend(mixed_iter.iter().cloned());
        assert_eq!(map.get(&1), Some(&150));
        assert_eq!(map.get(&2), Some(&200));
        assert_eq!(map.get(&3), Some(&300));
    }

    #[test]
    fn test_extend_does_not_change_unrelated_keys() {
        let mut map = HashMap::new();
        map.insert(1, 100);
        map.insert(2, 200);

        let some_iter = [(3, 300), (4, 400)];
        map.extend(some_iter.iter().cloned());
        
        assert_eq!(map.get(&1), Some(&100));
        assert_eq!(map.get(&2), Some(&200));
        assert_eq!(map.get(&3), Some(&300));
        assert_eq!(map.get(&4), Some(&400));
    }

    #[test]
    fn test_extend_and_check_final_length() {
        let mut map = HashMap::new();
        let initial_length = map.len();

        let some_vec = vec![(1, 1), (2, 2), (3, 3)];
        map.extend(some_vec.iter().cloned());
        
        assert_eq!(map.len(), initial_length + 3);
    }
}

#[cfg(test)]
mod tests_llm_16_33 {
    use crate::HashMap;

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_key_not_present() {
        let map: HashMap<&str, &str> = HashMap::new();
        let _ = map["key_not_present"];
    }

    #[test]
    fn test_index_key_present() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert_eq!(map["key1"], "value1");
        assert_eq!(map["key2"], "value2");
    }
    
    #[test]
    fn test_index_with_uninitialized_key() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("initialized_key", "initialized_value");

        assert_eq!(map["initialized_key"], "initialized_value");
    }
}

#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_hashmap_from_array() {
        // Test with a simple integer to integer mapping
        let map: HashMap<i32, i32> = HashMap::from([(1, 2), (3, 4)]);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));

        // Test with a different set of integers
        let map2: HashMap<i32, i32> = HashMap::from([(5, 6), (7, 8)]);
        assert_eq!(map2.len(), 2);
        assert_eq!(map2.get(&5), Some(&6));
        assert_eq!(map2.get(&7), Some(&8));

        // Test with empty array
        let empty_map: HashMap<i32, i32> = HashMap::from([]);
        assert!(empty_map.is_empty());

        // Test with strings
        let string_map: HashMap<&str, &str> = HashMap::from([("a", "apple"), ("b", "banana")]);
        assert_eq!(string_map.get(&"a"), Some(&"apple"));
        assert_eq!(string_map.get(&"b"), Some(&"banana"));
    }
}

#[cfg(test)]
mod tests_llm_16_37 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_into_iter_len() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");
        
        let iter = map.into_iter();
        let len = iter.len();
        
        assert_eq!(len, 3);
    }

    #[test]
    fn test_into_iter_len_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        
        let iter = map.into_iter();
        let len = iter.len();
        
        assert_eq!(len, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint() {
        let map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into_iter().collect();
        let iter = map.into_iter();

        // Check the size_hint when the iterator is created
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 3); // We expect at least 3 elements
        assert_eq!(upper, Some(3)); // We expect at most 3 elements

        let mut iter = iter; // Re-assign for further tests

        // Check size_hint after consuming some elements
        iter.next();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 2); // Now we expect at least 2 elements
        assert_eq!(upper, Some(2)); // Now we expect at most 2 elements

        iter.next();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 1); // Now we expect at least 1 element
        assert_eq!(upper, Some(1)); // Now we expect at most 1 element

        iter.next();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 0); // Now we expect at least 0 elements
        assert_eq!(upper, Some(0)); // Now we expect at most 0 elements
    }
}

#[cfg(test)]
mod tests_llm_16_43 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_into_keys_len() {
        // Create a HashMap and insert some key-value pairs
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        // Create an IntoKeys iterator from the HashMap
        let keys: IntoKeys<i32, &str> = map.into_keys();

        // Check the length using the len method
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_into_keys_empty_len() {
        // Create an empty HashMap
        let map: HashMap<i32, &str> = HashMap::new();

        // Create an IntoKeys iterator from the empty HashMap
        let keys: IntoKeys<i32, &str> = map.into_keys();

        // Check the length using the len method
        assert_eq!(keys.len(), 0);
    }

    #[test]
    fn test_into_keys_after_consuming() {
        // Create a HashMap and insert some key-value pairs
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        // Create an IntoKeys iterator
        let mut keys: IntoKeys<i32, &str> = map.into_keys();

        // Consume some keys
        keys.next();
        keys.next();

        // Check the length after consuming
        assert_eq!(keys.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_into_keys_next() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut into_keys = map.into_keys();
        let mut keys = HashSet::new();

        while let Some(key) = into_keys.next() {
            keys.insert(key);
        }

        let expected_keys: HashSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(keys, expected_keys);
    }

    #[test]
    fn test_into_keys_next_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        let mut into_keys = map.into_keys();

        assert_eq!(into_keys.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_50 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_fold_with_empty_map() {
        let map: HashMap<i32, &str> = HashMap::new();
        let result = map.into_values().fold(0, |acc, _| acc + 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fold_with_non_empty_map() {
        let map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let result = map.into_values().fold(0, |acc, _| acc + 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_fold_with_initial_value() {
        let map: HashMap<i32, i32> = [(1, 10), (2, 20), (3, 30)].into();
        let result = map.into_values().fold(100, |acc, val| acc + val);
        assert_eq!(result, 100 + 10 + 20 + 30);
    }

    #[test]
    fn test_fold_with_different_operation() {
        let map: HashMap<i32, i32> = [(1, 1), (2, 2), (3, 3)].into();
        let result = map.into_values().fold(1, |acc, val| acc * val);
        assert_eq!(result, 1 * 1 * 2 * 3);
    }

    #[test]
    fn test_fold_with_identity_function() {
        let map: HashMap<i32, i32> = [(1, 5), (2, 10)].into();
        let result = map.into_values().fold(0, |acc, val| acc + val);
        assert_eq!(result, 0 + 5 + 10);
    }
}

#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_into_values_next() {
        let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
        let mut values = map.into_values();

        let mut vec = vec![values.next(), values.next(), values.next()];

        // The `IntoValues` iterator produces values in arbitrary order, so
        // the values must be sorted to test them against a sorted array.
        vec.sort_unstable();
        assert_eq!(vec, [Some("a"), Some("b"), Some("c")]);

        // It is a fused iterator
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_52 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut into_values = map.into_values();
        let size_hint = into_values.size_hint();

        // The size hint should reflect the number of elements in the HashMap
        assert_eq!(size_hint, (3, Some(3)));

        // Consume all elements
        let _ = into_values.next();
        let size_hint_after_one = into_values.size_hint();

        // After consuming one element, the size hint should reflect 2 remaining
        assert_eq!(size_hint_after_one, (2, Some(2)));

        // Consume remaining elements
        let _ = into_values.next();
        let _ = into_values.next();
        let size_hint_after_all = into_values.size_hint();

        // After consuming all elements, the size hint should indicate no remaining elements
        assert_eq!(size_hint_after_all, (0, Some(0)));
    }
}

#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_len() {
        let map: HashMap<u32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let iter = map.iter();

        assert_eq!(iter.len(), 3);

        let mut iter_clone = iter.clone();
        assert_eq!(iter_clone.len(), 3);

        // Consume some elements
        assert!(iter_clone.next().is_some());
        assert_eq!(iter_clone.len(), 2);

        assert!(iter_clone.next().is_some());
        assert_eq!(iter_clone.len(), 1);

        // Consume the last element
        assert!(iter_clone.next().is_some());
        assert_eq!(iter_clone.len(), 0);

        // No elements left
        assert!(iter_clone.next().is_none());
        assert_eq!(iter_clone.len(), 0);
    }

    #[test]
    fn test_len_empty() {
        let map: HashMap<u32, &str> = HashMap::new();
        let iter = map.iter();

        assert_eq!(iter.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_59 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint_empty() {
        let map: HashMap<i32, i32> = HashMap::new();
        let iter = map.iter();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_single_element() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 10);
        let iter = map.iter();
        assert_eq!(iter.size_hint(), (1, Some(1)));
    }

    #[test]
    fn test_size_hint_multiple_elements() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let iter = map.iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_size_hint_after_consuming_elements() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let mut iter = map.iter();
        iter.next(); // consume one element
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next(); // consume another element
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}

#[cfg(test)]
mod tests_llm_16_69 {
    use crate::HashMap;

    #[test]
    fn test_keys_len() {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let keys = map.keys();
        assert_eq!(keys.len(), 3);

        let empty_map: HashMap<u32, &str> = HashMap::new();
        let empty_keys = empty_map.keys();
        assert_eq!(empty_keys.len(), 0);
    }

    #[test]
    fn test_keys_len_after_iterating() {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        let mut keys = map.keys();
        keys.next(); // Iterate once
        assert_eq!(keys.len(), 2);
        keys.next(); // Iterate again
        assert_eq!(keys.len(), 1);
        keys.next(); // Iterate again
        assert_eq!(keys.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_70 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_keys_fold() {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let keys = map.keys();
        let result = keys.fold(0, |acc, &key| acc + key);
        
        assert_eq!(result, 6); // 1 + 2 + 3 = 6
    }

    #[test]
    fn test_keys_fold_with_identity() {
        let map: HashMap<u32, &str> = HashMap::new();
        let keys = map.keys();
        
        let result = keys.fold(10, |acc, &key| acc + key);
        
        assert_eq!(result, 10); // no keys, should return the initial value
    }

    #[test]
    fn test_keys_fold_with_no_keys() {
        let map: HashMap<u32, &str> = HashMap::new();
        
        let keys = map.keys();
        let result = keys.fold(100, |acc, &key| acc + key);
        
        assert_eq!(result, 100); // no keys, should return the initial value
    }

    #[test]
    fn test_keys_fold_with_one_key() {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(5, "a");
        
        let keys = map.keys();
        let result = keys.fold(0, |acc, &key| acc + key);
        
        assert_eq!(result, 5); // only one key, the result should be the key itself
    }
}

#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_keys_next() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut keys = map.keys();
        let mut vec = vec![keys.next(), keys.next(), keys.next()];

        // The `Keys` iterator produces keys in arbitrary order, so the
        // keys must be sorted to test them against a sorted array.
        vec.sort_unstable();
        assert_eq!(vec, [Some(&1), Some(&2), Some(&3)]);
    }

    #[test]
    fn test_keys_next_fused() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        let mut keys = map.keys();
        
        // Consuming the keys
        assert_eq!(keys.next(), Some(&1));
        assert_eq!(keys.next(), None);
        assert_eq!(keys.next(), None);
    }

    #[test]
    fn test_keys_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        let mut keys = map.keys();
        
        assert_eq!(keys.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let keys = map.keys();
        let (lower, upper) = keys.size_hint();
        
        assert_eq!(lower, 3);
        assert_eq!(upper, Some(3));
    }

    #[test]
    fn test_size_hint_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        let keys = map.keys();
        let (lower, upper) = keys.size_hint();
        
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn clone_values_test() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        let values = map.values();
        let cloned_values = values.clone();

        assert_eq!(values.len(), cloned_values.len());
        for (v1, v2) in values.zip(cloned_values) {
            assert_eq!(v1, v2);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_82 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_fold() {
        let map: HashMap<i32, i32> = [(1, 1), (2, 2), (3, 3)].iter().cloned().collect();
        let values = map.values();

        let sum = values.fold(0, |acc, &value| acc + value);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold_empty() {
        let map: HashMap<i32, i32> = HashMap::new();
        let values = map.values();

        let sum = values.fold(0, |acc, &value| acc + value);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_fold_with_initial_value() {
        let map: HashMap<i32, i32> = [(1, 1), (2, 2)].iter().cloned().collect();
        let values = map.values();

        let sum = values.fold(10, |acc, &value| acc + value);
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_fold_with_identity() {
        let map: HashMap<i32, i32> = [(1, 1), (2, 2), (3, 3)].iter().cloned().collect();
        let values = map.values();

        let identity = values.fold(1, |acc, &value| acc * value);
        assert_eq!(identity, 6);
    }
}

#[cfg(test)]
mod tests_llm_16_83 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_values_next() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut values_iter = map.values();
        let mut results = vec![values_iter.next(), values_iter.next(), values_iter.next()];

        results.sort_unstable();
        assert_eq!(results, vec![Some(&"a"), Some(&"b"), Some(&"c")]);

        assert_eq!(values_iter.next(), None);
        assert_eq!(values_iter.next(), None);
    }

    #[test]
    fn test_values_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        let mut values_iter = map.values();

        assert_eq!(values_iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_84 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_size_hint_empty() {
        let map: HashMap<i32, &str> = HashMap::new();
        let values = map.values();
        assert_eq!(values.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_non_empty() {
        let map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let values = map.values();
        assert_eq!(values.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_size_hint_after_consuming() {
        let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();
        let mut values = map.values();
        assert_eq!(values.size_hint(), (3, Some(3)));
        values.next(); // Consume one
        assert_eq!(values.size_hint(), (2, Some(2)));
        values.next(); // Consume another
        assert_eq!(values.size_hint(), (1, Some(1)));
        values.next(); // Consume the last
        assert_eq!(values.size_hint(), (0, Some(0)));
    }
}

#[cfg(test)]
mod tests_llm_16_87 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_values_mut_len() {
        let mut map: HashMap<_, _> = [(1, "One"), (2, "Two"), (3, "Three")].iter().cloned().collect();

        let values_mut = map.values_mut();
        assert_eq!(values_mut.len(), 3);

        let mut values_mut_iter = values_mut;
        values_mut_iter.next();
        assert_eq!(values_mut_iter.len(), 2);

        values_mut_iter.next();
        assert_eq!(values_mut_iter.len(), 1);

        values_mut_iter.next();
        assert_eq!(values_mut_iter.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_250 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_and_modify_on_occupied_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("test", 1);

        map.entry("test")
            .and_modify(|e| *e += 1);

        assert_eq!(map["test"], 2);
    }

    #[test]
    fn test_and_modify_on_vacant_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        let entry = map.entry("test")
            .and_modify(|e| *e += 1);

        match entry {
            Entry::Vacant(_) => { /* Correct behavior: Entry is vacant */ }
            _ => panic!("Expected vacant entry, but got occupied."),
        }

        map.entry("test").or_insert(1);

        map.entry("test")
            .and_modify(|e| *e += 1);
        assert_eq!(map["test"], 2);
    }

    #[test]
    fn test_and_modify_multiple_entries() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("first", 1);
        map.insert("second", 2);

        map.entry("first")
            .and_modify(|e| *e += 1);
        map.entry("second")
            .and_modify(|e| *e += 2);

        assert_eq!(map["first"], 2);
        assert_eq!(map["second"], 4);
    }
}

#[cfg(test)]
mod tests_llm_16_251 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_and_replace_entry_with_vacant_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        let entry = map
            .entry("poneyland")
            .and_replace_entry_with(|_k, _v| panic!());

        match entry {
            Entry::Vacant(e) => {
                assert_eq!(e.key(), &"poneyland");
            }
            Entry::Occupied(_) => panic!(),
        }
    }

    #[test]
    fn test_and_replace_entry_with_occupied_entry_replace() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 42);

        let entry = map
            .entry("poneyland")
            .and_replace_entry_with(|k, v| {
                assert_eq!(k, &"poneyland");
                assert_eq!(v, 42);
                Some(v + 1)
            });

        match entry {
            Entry::Occupied(e) => {
                assert_eq!(e.key(), &"poneyland");
                assert_eq!(e.get(), &43);
            }
            Entry::Vacant(_) => panic!(),
        }

        assert_eq!(map["poneyland"], 43);
    }

    #[test]
    fn test_and_replace_entry_with_occupied_entry_remove() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 42);

        let entry = map
            .entry("poneyland")
            .and_replace_entry_with(|_k, _v| None);

        match entry {
            Entry::Vacant(e) => assert_eq!(e.key(), &"poneyland"),
            Entry::Occupied(_) => panic!(),
        }

        assert!(!map.contains_key("poneyland"));
    }
}

#[cfg(test)]
mod tests_llm_16_252 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_insert_existing_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("key1", 10);
        let entry = map.entry("key1");
        let occupied_entry = entry.insert(20);
        assert_eq!(occupied_entry.key(), &"key1");
        assert_eq!(map["key1"], 20);
    }

    #[test]
    fn test_insert_new_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let entry = map.entry("key2");
        let occupied_entry = entry.insert(30);
        assert_eq!(occupied_entry.key(), &"key2");
        assert_eq!(map["key2"], 30);
    }

    #[test]
    fn test_insert_with_multiple_keys() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("key3", 40);
        map.insert("key4", 50);

        let entry1 = map.entry("key3");
        let occupied_entry1 = entry1.insert(60);
        assert_eq!(occupied_entry1.key(), &"key3");
        assert_eq!(map["key3"], 60);

        let entry2 = map.entry("key4");
        let occupied_entry2 = entry2.insert(70);
        assert_eq!(occupied_entry2.key(), &"key4");
        assert_eq!(map["key4"], 70);
    }

    #[test]
    fn test_insert_with_nonexistent_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let entry = map.entry("key5");
        let occupied_entry = entry.insert(80);
        assert_eq!(occupied_entry.key(), &"key5");
        assert_eq!(map["key5"], 80);
    }
}

#[cfg(test)]
mod tests_llm_16_253 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_key_occupied_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 3);
        let entry = map.entry("poneyland");
        match entry {
            Entry::Occupied(ref occupied_entry) => {
                assert_eq!(occupied_entry.key(), &"poneyland");
            }
            _ => panic!("Expected occupied entry"),
        }
    }

    #[test]
    fn test_key_vacant_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let entry = map.entry("horseland");
        match entry {
            Entry::Vacant(ref vacant_entry) => {
                assert_eq!(vacant_entry.key(), &"horseland");
            }
            _ => panic!("Expected vacant entry"),
        }
    }

    #[test]
    fn test_key_with_existing_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("existing_key", 42);
        assert_eq!(map.entry("existing_key").key(), &"existing_key");
    }

    #[test]
    fn test_key_with_non_existing_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        assert_eq!(map.entry("non_existing_key").key(), &"non_existing_key");
    }
}

#[cfg(test)]
mod tests_llm_16_254 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_or_default_nonexistent_key() {
        let mut map: HashMap<&str, Option<u32>> = HashMap::new();
        let value = map.entry("nonexistent").or_default();
        assert_eq!(value, &mut None);
        assert_eq!(map["nonexistent"], None);
    }

    #[test]
    fn test_or_default_existing_key() {
        let mut map: HashMap<&str, Option<u32>> = HashMap::new();
        map.insert("existing", Some(42));
        let value = map.entry("existing").or_default();
        assert_eq!(value, &mut Some(42));
        assert_eq!(map["existing"], Some(42));
    }

    #[test]
    fn test_or_default_default_value() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        let value = map.entry("key").or_default();
        assert_eq!(value, &mut 0);
        assert_eq!(map["key"], 0);
    }

    #[test]
    fn test_or_default_multiple_keys() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.entry("key1").or_default();
        map.entry("key2").or_default();
        assert_eq!(map["key1"], 0);
        assert_eq!(map["key2"], 0);
    }

    #[test]
    fn test_or_default_with_custom_type() {
        #[derive(Default, PartialEq, Debug)]
        struct CustomType {
            value: u32,
        }

        let mut map: HashMap<&str, CustomType> = HashMap::new();
        let value = map.entry("custom").or_default();
        assert_eq!(value, &mut CustomType::default());
        assert_eq!(map["custom"], CustomType { value: 0 });
    }
}

#[cfg(test)]
mod tests_llm_16_255 {
    use crate::HashMap;

    #[test]
    fn test_or_insert_vacant() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        assert_eq!(map.entry("new_key").or_insert(10), &mut 10);
        assert_eq!(map["new_key"], 10);
    }

    #[test]
    fn test_or_insert_occupied() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("existing_key", 20);
        assert_eq!(map.entry("existing_key").or_insert(30), &mut 20);
        assert_eq!(map["existing_key"], 20);
    }

    #[test]
    fn test_or_insert_update() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("update_key", 5);
        *map.entry("update_key").or_insert(10) *= 2;
        assert_eq!(map["update_key"], 10);
    }

    #[test]
    fn test_or_insert_multiple() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        assert_eq!(map.entry("a").or_insert(1), &mut 1);
        assert_eq!(map.entry("b").or_insert(2), &mut 2);
        assert_eq!(map.entry("a").or_insert(3), &mut 1);
        assert_eq!(map.entry("b").or_insert(4), &mut 2);
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
    }
}

#[cfg(test)]
mod tests_llm_16_257 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_or_insert_with_key_nonexistent_key() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.entry("poneyland").or_insert_with_key(|key| key.len());
        assert_eq!(map["poneyland"], 9);
    }

    #[test]
    fn test_or_insert_with_key_existing_key() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.insert("poneyland", 9);
        *map.entry("poneyland").or_insert_with_key(|key| key.len() * 10) *= 2;
        assert_eq!(map["poneyland"], 18);
    }

    #[test]
    fn test_or_insert_with_key_multiple_keys() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.entry("test1").or_insert_with_key(|key| key.chars().count());
        map.entry("test2").or_insert_with_key(|key| key.chars().count());
        assert_eq!(map["test1"], 5);
        assert_eq!(map["test2"], 5);
    }

    #[test]
    fn test_or_insert_with_key_reference_key() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        let key = "test_key";
        map.entry(key).or_insert_with_key(|k| k.chars().count());
        assert_eq!(map[key], 8);
    }

    #[test]
    fn test_or_insert_with_key_key_derivation() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.entry("key1").or_insert_with_key(|key| key.chars().count() * 2);
        assert_eq!(map["key1"], 8);
    }
}

#[cfg(test)]
mod tests_llm_16_265 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_allocation_size() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        assert_eq!(map.allocation_size(), 0);

        map.insert(1, 100);
        assert!(map.allocation_size() > 0);

        map.insert(2, 200);
        assert!(map.allocation_size() >= 2 * std::mem::size_of::<(i32, i32)>());

        for i in 3..100 {
            map.insert(i, i * 10);
        }
        assert!(map.allocation_size() >= 100 * std::mem::size_of::<(i32, i32)>());

        map.clear();
        assert_eq!(map.allocation_size(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_269 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_clear() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        let capacity_before_clear = map.capacity();

        map.clear();

        assert!(map.is_empty());
        assert_eq!(map.capacity(), capacity_before_clear);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_clear_empty_map() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        let capacity_before_clear = map.capacity();

        map.clear();

        assert!(map.is_empty());
        assert_eq!(map.capacity(), capacity_before_clear);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_clear_with_capacity() {
        let mut map: HashMap<i32, &str> = HashMap::with_capacity(5);
        map.insert(1, "a");
        map.insert(2, "b");

        let capacity_before_clear = map.capacity();
        map.clear();

        assert!(map.is_empty());
        assert_eq!(map.capacity(), capacity_before_clear);
        assert_eq!(map.len(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_272 {
    use crate::hash_map::HashMap;

    #[test]
    fn test_entry_insert() {
        let mut map = HashMap::new();
        let entry = map.entry("key1");
        let value = entry.or_insert(10);
        *value += 5;

        assert_eq!(map.len(), 1);
        assert_eq!(map["key1"], 15);
    }

    #[test]
    fn test_entry_or_insert() {
        let mut map = HashMap::new();
        let entry = map.entry("key2");
        assert_eq!(entry.or_insert(20), &mut 20);
        assert_eq!(map.len(), 1);
        assert_eq!(map["key2"], 20);
    }

    #[test]
    fn test_entry_or_insert_with() {
        let mut map = HashMap::new();
        let entry = map.entry("key3");
        assert_eq!(entry.or_insert_with(|| 30), &mut 30);
        assert_eq!(map.len(), 1);
        assert_eq!(map["key3"], 30);
    }

    #[test]
    fn test_entry_or_insert_with_key() {
        let mut map = HashMap::new();
        let entry = map.entry("key4");
        assert_eq!(entry.or_insert_with_key(|key| key.len() as i32), &mut 4);
        assert_eq!(map.len(), 1);
        assert_eq!(map["key4"], 4);
    }

    #[test]
    fn test_entry_and_modify() {
        let mut map = HashMap::new();
        map.insert("key5", 100);
        let entry = map.entry("key5");
        entry.and_modify(|value| *value += 50);
        
        assert_eq!(map["key5"], 150);
    }

    #[test]
    fn test_entry_vacant() {
        let mut map = HashMap::new();
        let entry = map.entry("key6");
        assert!(matches!(entry, crate::hash_map::Entry::Vacant(_)));
        entry.or_insert(60);
        
        assert_eq!(map["key6"], 60);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_entry_occupied() {
        let mut map = HashMap::new();
        map.insert("key7", 70);
        let entry = map.entry("key7");
        
        assert!(matches!(entry, crate::hash_map::Entry::Occupied(_)));
        assert_eq!(entry.or_insert(70), &mut 70);
        assert_eq!(map["key7"], 70);
    }
}

#[cfg(test)]
mod tests_llm_16_280 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_get_key_value_mut_existing_key() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        
        let (k, v) = map.get_key_value_mut(&1).unwrap();
        assert_eq!(k, &1);
        assert_eq!(v, &mut "a");
        *v = "b";
        assert_eq!(map.get_key_value_mut(&1).unwrap(), (&1, &mut "b"));
    }

    #[test]
    fn test_get_key_value_mut_nonexistent_key() {
        let mut map = HashMap::new();
        map.insert(1, "a");

        let result = map.get_key_value_mut(&2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_key_value_mut_multiple_entries() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        {
            let (k, v) = map.get_key_value_mut(&1).unwrap();
            assert_eq!(k, &1);
            assert_eq!(v, &mut "a");
            *v = "c";
        }

        assert_eq!(map.get_key_value_mut(&1).unwrap(), (&1, &mut "c"));
        assert_eq!(map.get_key_value_mut(&2).unwrap(), (&2, &mut "b"));
    }

    #[test]
    fn test_get_key_value_mut_with_different_key_type() {
        let mut map = HashMap::new();
        map.insert(1, "a");

        let (k, v) = map.get_key_value_mut(&1).unwrap();
        assert_eq!(k, &1);
        assert_eq!(v, &mut "a");
    }

    #[test]
    fn test_get_key_value_mut_with_mutable_references() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        
        let (k, v) = map.get_key_value_mut(&1).unwrap();
        assert_eq!(k, &1);
        *v = "changed";
        
        assert_eq!(map.get_key_value_mut(&1).unwrap(), (&1, &mut "changed"));
    }
}

#[cfg(test)]
mod tests_llm_16_287 {
    use crate::HashMap;

    #[test]
    fn test_get_mut() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        
        // Test mutating existing key
        if let Some(x) = map.get_mut(&1) {
            *x = "b";
        }
        assert_eq!(map.get(&1), Some(&"b"));

        // Test getting a key that does not exist
        assert_eq!(map.get_mut(&2), None);
    }

    #[test]
    fn test_get_mut_non_existent() {
        let mut map = HashMap::new();
        map.insert(2, "x");
        
        // Test mutating a non-existent key
        assert_eq!(map.get_mut(&3), None);
    }

    #[test]
    fn test_get_mut_multiple() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        
        // Modify both existing keys
        if let Some(x) = map.get_mut(&1) {
            *x += 5; // 10 + 5 = 15
        }
        if let Some(x) = map.get_mut(&2) {
            *x *= 2; // 20 * 2 = 40
        }

        assert_eq!(map.get(&1), Some(&15));
        assert_eq!(map.get(&2), Some(&40));
    }
}

#[cfg(test)]
mod tests_llm_16_289 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_insert_new_key() {
        let mut map = HashMap::new();
        let result = map.insert(42, "value");
        assert_eq!(result, None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&42), Some(&"value"));
    }

    #[test]
    fn test_insert_update_existing_key() {
        let mut map = HashMap::new();
        map.insert(42, "value1");
        let result = map.insert(42, "value2");
        assert_eq!(result, Some("value1"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&42), Some(&"value2"));
    }

    #[test]
    fn test_insert_multiple_keys() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_insert_zero_keys() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let result = map.insert(0, 0);
        assert_eq!(result, None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&0), Some(&0));
    }

    #[test]
    fn test_insert_overwriting_different_types() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("a", 2);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("a"), Some(&2));
    }

    #[test]
    fn test_insert_and_drop() {
        let mut map = HashMap::new();
        map.insert(1, "value");
        drop(map);
        // map is dropped, the test is to ensure no panic occurs
    }
}

#[cfg(test)]
mod tests_llm_16_293 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_is_empty() {
        let map: HashMap<i32, i32> = HashMap::new();
        assert!(map.is_empty());

        let mut map = HashMap::new();
        map.insert(1, 1);
        assert!(!map.is_empty());

        map.remove(&1);
        assert!(map.is_empty());
    }

    #[test]
    fn test_is_empty_with_capacity() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
        assert!(map.is_empty());

        map.insert(1, 1);
        assert!(!map.is_empty());

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_is_empty_after_operations() {
        let mut map = HashMap::new();
        assert!(map.is_empty());

        map.insert(2, 2);
        assert!(!map.is_empty());

        map.insert(3, 3);
        assert!(!map.is_empty());

        map.remove(&2);
        assert!(!map.is_empty());

        map.remove(&3);
        assert!(map.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_297 {
    use crate::HashMap;

    #[test]
    fn test_len() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0); // Check initial length

        map.insert(1, "a");
        assert_eq!(map.len(), 1); // Check length after one insertion

        map.insert(2, "b");
        map.insert(3, "c");
        assert_eq!(map.len(), 3); // Check length after multiple insertions

        map.remove(&2);
        assert_eq!(map.len(), 2); // Check length after removal

        map.clear();
        assert_eq!(map.len(), 0); // Check length after clearing the map
    }
}

#[cfg(test)]
mod tests_llm_16_298 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_remove_existing_key() {
        let mut map = HashMap::new();
        map.insert(1, "value1");
        map.insert(2, "value2");

        assert_eq!(map.remove(&1), Some("value1"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&1), None);
    }

    #[test]
    fn test_remove_non_existing_key() {
        let mut map = HashMap::new();
        map.insert(1, "value1");

        assert_eq!(map.remove(&2), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_remove_empty_map() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        assert_eq!(map.remove(&1), None);
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove_after_insert() {
        let mut map = HashMap::new();
        map.insert(1, "value");
        assert_eq!(map.remove(&1), Some("value"));
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove_and_insert_same_key() {
        let mut map = HashMap::new();
        map.insert(1, "old_value");

        assert_eq!(map.remove(&1), Some("old_value"));
        assert_eq!(map.len(), 0);

        map.insert(1, "new_value");
        assert_eq!(map.remove(&1), Some("new_value"));
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove_key_when_equivalent() {
        #[derive(Hash, Eq, PartialEq)]
        struct Key {
            id: i32,
        }

        let mut map = HashMap::new();
        map.insert(Key { id: 1 }, "value1");

        assert_eq!(map.remove(&Key { id: 1 }), Some("value1"));
        assert!(map.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_302 {
    use crate::HashMap;

    #[test]
    fn test_shrink_to() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);
        assert!(map.capacity() >= 100);

        map.shrink_to(10);
        assert!(map.capacity() >= 10);

        map.shrink_to(0);
        assert!(map.capacity() >= 2);

        map.shrink_to(10);
        assert!(map.capacity() >= 2);
    }

    #[test]
    fn test_shrink_to_no_effect() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(20);
        map.insert(1, 2);
        map.insert(3, 4);
        assert!(map.capacity() >= 20);

        map.shrink_to(30);
        assert!(map.capacity() >= 20);
    }

    #[test]
    fn test_shrink_to_empty() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        assert!(map.capacity() == 0);

        map.shrink_to(10);
        assert!(map.capacity() == 0);

        map.insert(1, 2);
        map.shrink_to(0);
        assert!(map.capacity() >= 1);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);

        assert!(map.capacity() >= 100);
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
    }
}

#[cfg(test)]
mod tests_llm_16_303 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_shrink_to_fit() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 1);
        map.insert(2, 2);
        assert!(map.capacity() >= 100);
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
    }

    #[test]
    fn test_shrink_to_fit_empty() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        assert!(map.capacity() == 0);
        map.shrink_to_fit();
        assert!(map.capacity() == 0);
    }

    #[test]
    fn test_shrink_to_fit_with_elements() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
        for i in 0..100 {
            map.insert(i, i);
        }
        assert!(map.capacity() > 50);
        map.shrink_to_fit();
        assert!(map.capacity() >= 100); // may not shrink below 100 due to internal resize policy
    }

    #[test]
    fn test_shrink_to_fit_after_removal() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        assert!(map.capacity() >= 3);
        map.remove(&1);
        map.shrink_to_fit();
        assert!(map.capacity() >= 2); // after considering the removal, it should manage size
    }
}

#[cfg(test)]
mod tests_llm_16_304 {
    use super::*;

use crate::*;
    use crate::hash_map::OccupiedError;

    #[test]
    fn test_try_insert_success() {
        let mut map = HashMap::new();
        let value = map.try_insert(1, "first").unwrap();
        assert_eq!(value, &"first");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_try_insert_existing_key() {
        let mut map = HashMap::new();
        let _ = map.try_insert(1, "first").unwrap();

        match map.try_insert(1, "second") {
            Err(OccupiedError { entry, value }) => {
                assert_eq!(entry.key(), &1);
                assert_eq!(entry.get(), &"first");
                assert_eq!(value, "second");
            }
            _ => panic!("Expected an error for existing key"),
        }
    }

    #[test]
    fn test_try_insert_multiple_keys() {
        let mut map = HashMap::new();
        assert_eq!(map.try_insert(1, "one").unwrap(), &"one");
        assert_eq!(map.try_insert(2, "two").unwrap(), &"two");
        assert_eq!(map.len(), 2);
        
        match map.try_insert(1, "one_new") {
            Err(OccupiedError { entry, value }) => {
                assert_eq!(entry.key(), &1);
                assert_eq!(entry.get(), &"one");
                assert_eq!(value, "one_new");
            }
            _ => panic!("Expected an error for existing key"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_312 {
    use super::*;

use crate::*;
    use bumpalo::Bump;

    #[test]
    fn test_new_in_empty() {
        let bump = Bump::new();
        let map: HashMap<i32, i32, DefaultHashBuilder, _> = HashMap::new_in(&bump);
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
    }

    #[test]
    fn test_new_in_insert() {
        let bump = Bump::new();
        let mut map: HashMap<&str, i32, DefaultHashBuilder, _> = HashMap::new_in(&bump);
        map.insert("One", 1);
        assert_eq!(map.len(), 1);
        assert!(map.capacity() > 0);
    }

    #[test]
    fn test_new_in_multiple_inserts() {
        let bump = Bump::new();
        let mut map: HashMap<&str, i32, DefaultHashBuilder, _> = HashMap::new_in(&bump);
        let entries = [("One", 1), ("Two", 2), ("Three", 3)];
        
        for (key, value) in entries.iter() {
            map.insert(*key, *value);
        }

        assert_eq!(map.len(), 3);
        assert!(map.capacity() > 3); // expecting some reallocation to occur
    }

    #[test]
    fn test_new_in_capacity() {
        let bump = Bump::new();
        let mut map: HashMap<i32, i32, DefaultHashBuilder, _> = HashMap::new_in(&bump);
        assert_eq!(map.capacity(), 0);

        for i in 0..10 {
            map.insert(i, i * 2);
        }

        assert_eq!(map.len(), 10);
        assert!(map.capacity() > 10); // expecting some reallocation to occur
    }
}

#[cfg(test)]
mod tests_llm_16_314 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_new_hashmap() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_315 {
    use super::*;

use crate::*;
    use crate::HashMap;

    #[test]
    fn test_with_capacity() {
        let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
        assert_eq!(map.len(), 0);
        assert!(map.capacity() >= 10);
    }

    #[test]
    fn test_with_capacity_zero() {
        let map: HashMap<&str, i32> = HashMap::with_capacity(0);
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
    }

    #[test]
    fn test_with_capacity_below_initial_capacity() {
        let mut map: HashMap<&str, i32> = HashMap::with_capacity(5);
        assert!(map.capacity() >= 5);
        map.insert("One", 1);
        map.insert("Two", 2);
        assert!(map.capacity() >= 5);
    }

    #[test]
    fn test_with_capacity_more_than_initial_insert() {
        let mut map: HashMap<&str, i32> = HashMap::with_capacity(5);
        assert_eq!(map.len(), 0);
        assert!(map.capacity() >= 5);
        
        map.insert("Three", 3);
        map.insert("Four", 4);
        map.insert("Five", 5);
        assert_eq!(map.len(), 3);
        assert!(map.capacity() >= 5);
    }
}

#[cfg(test)]
mod tests_llm_16_319 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_get_mut() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        assert_eq!(map["poneyland"], 12);
        if let Entry::Occupied(mut entry) = map.entry("poneyland") {
            *entry.get_mut() += 10;
            assert_eq!(*entry.get(), 22);
            *entry.get_mut() += 2;
        }

        assert_eq!(map["poneyland"], 24);
    }

    #[test]
    fn test_get_mut_non_existent() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        if let Entry::Occupied(mut entry) = map.entry("poneyland") {
            assert_eq!(entry.get_mut(), &mut 12);
            *entry.get_mut() += 10;
        }

        assert_eq!(map.get_mut("nonexistent"), None);
    }
}

#[cfg(test)]
mod tests_llm_16_321 {
    use super::*;

use crate::*;
    use crate::hash_map::{Entry, HashMap};

    #[test]
    fn test_into_mut() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        match map.entry("poneyland") {
            Entry::Occupied(entry) => {
                let value = entry.into_mut();
                *value += 10;
                assert_eq!(map["poneyland"], 22);
            }
            Entry::Vacant(_) => panic!("Entry should be occupied"),
        }
    }

    #[test]
    fn test_into_mut_non_existent() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        match map.entry("missing") {
            Entry::Occupied(_) => panic!("Entry should be vacant"),
            Entry::Vacant(entry) => {
                entry.insert(10);
                assert_eq!(map["missing"], 10);
            }
        }
        match map.entry("missing") {
            Entry::Occupied(entry) => {
                let value = entry.into_mut();
                *value += 5;
                assert_eq!(map["missing"], 15);
            }
            Entry::Vacant(_) => panic!("Entry should be occupied"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_322 {
    use super::*; // Ensure we are in the correct scope

use crate::*;
    use crate::hash_map::HashMap;

    #[test]
    fn test_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        match map.entry("poneyland") {
            Entry::Vacant(_) => panic!(),
            Entry::Occupied(entry) => assert_eq!(entry.key(), &"poneyland"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_323 {
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_remove() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        if let Entry::Occupied(o) = map.entry("poneyland") {
            assert_eq!(o.remove(), 12);
        }

        assert_eq!(map.contains_key("poneyland"), false);
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove_non_existent_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("example", 34);

        {
            let entry = map.entry("example");
            if let Entry::Occupied(o) = entry {
                assert_eq!(o.remove(), 34);
            }
        }

        assert!(map.contains_key("example") == false);
        assert!(map.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_324 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_remove_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("poneyland", 12);

        if let Entry::Occupied(o) = map.entry("poneyland") {
            assert_eq!(o.remove_entry(), ("poneyland", 12));
        }

        assert_eq!(map.contains_key("poneyland"), false);
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove_entry_empty_map() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        assert!(map.is_empty());

        let entry_result = map.entry("nonexistent");
        assert!(matches!(entry_result, Entry::Vacant(_)));
    }

    #[test]
    fn test_remove_entry_multiple() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);

        if let Entry::Occupied(o) = map.entry("foo") {
            assert_eq!(o.remove_entry(), ("foo", 1));
        }

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("bar"), Some(&2));
    }
}

#[cfg(test)]
mod tests_llm_16_325 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_replace_entry_with() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        
        map.insert("poneyland", 42);
        let entry = map.entry("poneyland").or_insert(0);
        
        let replaced_entry = map.entry("poneyland").and_modify(|v| *v = 0).or_insert(0);
        
        let entry = match map.entry("poneyland") {
            Entry::Occupied(e) => e.replace_entry_with(|k, v| {
                assert_eq!(k, &"poneyland");
                assert_eq!(v, 0);
                Some(v + 1)
            }),
            Entry::Vacant(_) => panic!(),
        };

        match entry {
            Entry::Occupied(e) => {
                assert_eq!(e.key(), &"poneyland");
                assert_eq!(e.get(), &1);
            },
            Entry::Vacant(_) => panic!(),
        }

        assert_eq!(map["poneyland"], 1);
        
        let entry = match map.entry("poneyland") {
            Entry::Occupied(e) => e.replace_entry_with(|_k, _v| None),
            Entry::Vacant(_) => panic!(),
        };

        match entry {
            Entry::Vacant(e) => {
                assert_eq!(e.key(), &"poneyland");
            },
            Entry::Occupied(_) => panic!(),
        }

        assert!(!map.contains_key("poneyland"));
    }
}

#[cfg(test)]
mod tests_llm_16_326 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_insert_vacant_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        if let Entry::Vacant(o) = map.entry("test_key") {
            o.insert(42);
        }

        assert_eq!(map["test_key"], 42);
    }

    #[test]
    fn test_insert_vacant_entry_overwrite() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        if let Entry::Vacant(o) = map.entry("key") {
            o.insert(10);
        }

        if let Entry::Vacant(o) = map.entry("key") {
            o.insert(20);
        }

        assert_eq!(map["key"], 10);
    }

    #[test]
    fn test_insert_multiple_vacant_entries() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        let keys = ["key1", "key2", "key3"];
        let values = [1, 2, 3];

        for (key, value) in keys.iter().zip(values.iter()) {
            if let Entry::Vacant(o) = map.entry(key) {
                o.insert(*value);
            }
        }

        assert_eq!(map["key1"], 1);
        assert_eq!(map["key2"], 2);
        assert_eq!(map["key3"], 3);
    }

    #[test]
    fn test_insert_with_empty_map() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        if let Entry::Vacant(o) = map.entry("new_key") {
            o.insert(100);
        }

        assert_eq!(map["new_key"], 100);
    }
}

#[cfg(test)]
mod tests_llm_16_327 {
    use super::*;

use crate::*;
    use crate::HashMap;
    use crate::hash_map::Entry;

    #[test]
    fn test_insert_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        if let Entry::Vacant(v) = map.entry("test_entry") {
            let occupied_entry = v.insert_entry(42);
            assert_eq!(occupied_entry.get(), &42);
            assert_eq!(map.get("test_entry"), Some(&42));
        }
    }
    
    #[test]
    fn test_insert_entry_existing_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("test_entry", 42);

        if let Entry::Vacant(v) = map.entry("test_entry") {
            let occupied_entry = v.insert_entry(100);
            assert_eq!(occupied_entry.get(), &100);
            assert_eq!(map.get("test_entry"), Some(&100));
        }
    }
    
    #[test]
    fn test_insert_entry_multiple() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        if let Entry::Vacant(v) = map.entry("entry_one") {
            v.insert_entry(1);
        }
        if let Entry::Vacant(v) = map.entry("entry_two") {
            v.insert_entry(2);
        }
        if let Entry::Vacant(v) = map.entry("entry_three") {
            v.insert_entry(3);
        }

        assert_eq!(map.get("entry_one"), Some(&1));
        assert_eq!(map.get("entry_two"), Some(&2));
        assert_eq!(map.get("entry_three"), Some(&3));
    }
}

#[cfg(test)]
mod tests_llm_16_328 {
    use super::*;

use crate::*;
    use crate::hash_map::{Entry, HashMap};

    #[test]
    fn test_into_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        match map.entry("test_key") {
            Entry::Occupied(_) => panic!("Entry should be vacant"), 
            Entry::Vacant(v) => {
                let key = v.into_key(); 
                assert_eq!(key, "test_key");
            },
        }
    }

    #[test]
    fn test_into_key_with_existing_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.insert("existing_key", 42);

        match map.entry("existing_key") {
            Entry::Occupied(_) => {
                panic!("Entry should be occupied, not vacant");
            },
            Entry::Vacant(_) => {
                panic!("Entry should not be vacant since key exists");
            },
        }
    }

    #[test]
    fn test_multiple_into_key() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        assert!(map.is_empty());

        match map.entry("another_key") {
            Entry::Occupied(_) => panic!("Entry should be vacant"),
            Entry::Vacant(v) => {
                let key = v.into_key();
                assert_eq!(key, "another_key");
            },
        }

        assert_eq!(map.len(), 1);
        assert!(map.contains_key("another_key"));
    }
}

#[cfg(test)]
mod tests_llm_16_345 {
    use super::*;

use crate::*;
    use crate::HashMap;

    struct KeyWrapper {
        value: i32,
    }

    impl KeyWrapper {
        fn new(value: i32) -> Self {
            KeyWrapper { value }
        }
    }

    struct KeyEqual {
        value: i32,
    }

    impl KeyEqual {
        fn new(value: i32) -> Self {
            KeyEqual { value }
        }
    }

    impl Equivalent<KeyWrapper> for KeyEqual {
        fn equivalent(&self, other: &KeyWrapper) -> bool {
            self.value == other.value
        }
    }

    #[test]
    fn test_equivalent() {
        let key_equal = KeyEqual::new(42);
        let equivalent_fn = equivalent(&key_equal);

        let key_wrapper_equal = KeyWrapper::new(42);
        let key_wrapper_unequal = KeyWrapper::new(10);
        
        assert!(equivalent_fn(&key_wrapper_equal));
        assert!(!equivalent_fn(&key_wrapper_unequal));
    }
}
