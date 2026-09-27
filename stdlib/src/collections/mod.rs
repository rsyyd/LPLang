// Collections: Vec, Map, Set, String

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Self { ptr: std::ptr::null_mut(), len: 0, cap: 0 }
    }

    pub fn push(&mut self, value: T) {
        // TODO
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct Map<K, V> {
    // HashMap implementation
    _marker: std::marker::PhantomData<(K, V)>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        None
    }
}

pub struct Set<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }

    pub fn insert(&mut self, value: T) -> bool {
        false
    }

    pub fn contains(&self, value: &T) -> bool {
        false
    }
}

pub struct String {
    vec: std::vec::Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        Self { vec: std::vec::Vec::new() }
    }

    pub fn from(s: &str) -> Self {
        Self { vec: s.as_bytes().to_vec() }
    }

    pub fn push_str(&mut self, s: &str) {
        self.vec.extend_from_slice(s.as_bytes());
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.vec).unwrap()
    }
}

pub struct Bytes {
    vec: std::vec::Vec<u8>,
}