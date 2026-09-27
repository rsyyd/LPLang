// Core primitives and traits

pub trait Display {
    fn fmt(&self) -> String;
}

pub trait Debug {
    fn fmt(&self) -> String;
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub trait Copy: Clone {}

pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;
}

pub trait Eq: PartialEq {}

pub trait PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}

pub trait Ord: PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

pub trait Sub<Rhs = Self> {
    type Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
}

pub trait Mul<Rhs = Self> {
    type Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
}

pub trait Div<Rhs = Self> {
    type Output;
    fn div(self, rhs: Rhs) -> Self::Output;
}

pub trait Rem<Rhs = Self> {
    type Output;
    fn rem(self, rhs: Rhs) -> Self::Output;
}

pub trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}

pub trait Not {
    type Output;
    fn not(self) -> Self::Output;
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Simplified Future trait (not using Pin for now)
pub trait Future {
    type Output;
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub struct Pin<P> {
    pointer: P,
}

impl<P> Pin<P> {
    pub fn new(pointer: P) -> Self {
        Self { pointer }
    }
}

pub struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    pub fn new(waker: &'a Waker) -> Self {
        Self { waker }
    }
}

pub struct Waker {
    // ...
}

impl Waker {
    pub fn noop() -> Self {
        Self {}
    }
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Drop {
    fn drop(&mut self);
}

pub struct Option<T> {
    inner: OptionInner<T>,
}

enum OptionInner<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub const fn none() -> Self {
        Self { inner: OptionInner::None }
    }

    pub const fn some(value: T) -> Self {
        Self { inner: OptionInner::Some(value) }
    }

    pub fn is_some(&self) -> bool {
        matches!(self.inner, OptionInner::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self.inner, OptionInner::None)
    }

    pub fn unwrap(self) -> T {
        match self.inner {
            OptionInner::Some(v) => v,
            OptionInner::None => panic!("unwrap on None"),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self.inner {
            OptionInner::Some(v) => Option::some(f(v)),
            OptionInner::None => Option::none(),
        }
    }
}

pub struct Result<T, E> {
    inner: ResultInner<T, E>,
}

enum ResultInner<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub fn ok(value: T) -> Self {
        Self { inner: ResultInner::Ok(value) }
    }

    pub fn err(error: E) -> Self {
        Self { inner: ResultInner::Err(error) }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self.inner, ResultInner::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self.inner, ResultInner::Err(_))
    }

    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self.inner {
            ResultInner::Ok(v) => v,
            ResultInner::Err(e) => panic!("unwrap on Err: {:?}", e),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U, E> {
        match self.inner {
            ResultInner::Ok(v) => Result::ok(f(v)),
            ResultInner::Err(e) => Result::err(e),
        }
    }
}