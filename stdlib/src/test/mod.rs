// Built-in test framework

#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            panic!("assertion failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $msg:expr $(,)?) => {
        if !$cond {
            panic!("assertion failed: {} - {}", stringify!($cond), $msg);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        let left = $left;
        let right = $right;
        if left != right {
            panic!("assertion failed: `(left == right)`\n  left: `{:?}`\n right: `{:?}`", left, right);
        }
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        let left = $left;
        let right = $right;
        if left == right {
            panic!("assertion failed: `(left != right)`\n  left: `{:?}`\n right: `{:?}`", left, right);
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Test;

pub fn run_tests() {
    // Test runner
}