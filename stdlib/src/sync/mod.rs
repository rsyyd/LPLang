// Synchronization: Channel, Mutex, RwLock, Atomic

pub mod channel {
    use std::fmt;

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let (tx, rx) = crossbeam_channel::unbounded();
        (Sender { inner: tx }, Receiver { inner: rx })
    }

    pub struct Sender<T> {
        inner: crossbeam_channel::Sender<T>,
    }

    pub struct Receiver<T> {
        inner: crossbeam_channel::Receiver<T>,
    }

    impl<T> Sender<T> {
        pub fn send(&self, value: T) -> Result<(), SendError<T>> {
            self.inner.send(value).map_err(|e| SendError(e.0))
        }
    }

    impl<T> Receiver<T> {
        pub fn recv(&self) -> Result<T, RecvError> {
            self.inner.recv().map_err(|_| RecvError)
        }
    }

    #[derive(Debug)]
    pub struct SendError<T>(pub T);

    impl<T> fmt::Display for SendError<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "send error")
        }
    }

    impl<T: fmt::Debug> std::error::Error for SendError<T> {}

    #[derive(Debug)]
    pub struct RecvError;

    impl fmt::Display for RecvError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "recv error")
        }
    }

    impl std::error::Error for RecvError {}
}

pub struct Mutex<T> {
    inner: parking_lot::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self { inner: parking_lot::Mutex::new(value) }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard { inner: self.inner.lock() }
    }
}

pub struct MutexGuard<'a, T> {
    inner: parking_lot::MutexGuard<'a, T>,
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub struct RwLock<T> {
    inner: parking_lot::RwLock<T>,
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self { inner: parking_lot::RwLock::new(value) }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard { inner: self.inner.read() }
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        RwLockWriteGuard { inner: self.inner.write() }
    }
}

pub struct RwLockReadGuard<'a, T> {
    inner: parking_lot::RwLockReadGuard<'a, T>,
}

impl<'a, T> std::ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

pub struct RwLockWriteGuard<'a, T> {
    inner: parking_lot::RwLockWriteGuard<'a, T>,
}

impl<'a, T> std::ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T> std::ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub mod atomic {
    use std::sync::atomic::{AtomicBool as StdAtomicBool, AtomicU32 as StdAtomicU32, 
                           AtomicU64 as StdAtomicU64, AtomicUsize as StdAtomicUsize, Ordering};

    pub struct AtomicBool(StdAtomicBool);
    pub struct AtomicU32(StdAtomicU32);
    pub struct AtomicU64(StdAtomicU64);
    pub struct AtomicUsize(StdAtomicUsize);

    impl AtomicBool {
        pub fn new(v: bool) -> Self { Self(StdAtomicBool::new(v)) }
        pub fn load(&self, order: Ordering) -> bool { self.0.load(order) }
        pub fn store(&self, v: bool, order: Ordering) { self.0.store(v, order) }
    }

    impl AtomicU32 {
        pub fn new(v: u32) -> Self { Self(StdAtomicU32::new(v)) }
        pub fn load(&self, order: Ordering) -> u32 { self.0.load(order) }
        pub fn store(&self, v: u32, order: Ordering) { self.0.store(v, order) }
    }

    impl AtomicU64 {
        pub fn new(v: u64) -> Self { Self(StdAtomicU64::new(v)) }
        pub fn load(&self, order: Ordering) -> u64 { self.0.load(order) }
        pub fn store(&self, v: u64, order: Ordering) { self.0.store(v, order) }
    }

    impl AtomicUsize {
        pub fn new(v: usize) -> Self { Self(StdAtomicUsize::new(v)) }
        pub fn load(&self, order: Ordering) -> usize { self.0.load(order) }
        pub fn store(&self, v: usize, order: Ordering) { self.0.store(v, order) }
    }
}