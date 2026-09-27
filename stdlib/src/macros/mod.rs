// Built-in macros

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        // TODO: compile-time format string validation
        std::format!($($arg)*)
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        std::print!($($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => { std::println!() };
    ($($arg:tt)*) => {{
        std::println!($($arg)*);
    }};
}

#[macro_export]
macro_rules! vec {
    () => { Vec::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($elem);)+
        v
    }};
}

#[macro_export]
macro_rules! map {
    () => { Map::new() };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut m = Map::new();
        $(m.insert($key, $value);)+
        m
    }};
}

#[macro_export]
macro_rules! try_ {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

#[macro_export]
macro_rules! nursery {
    ($($stmt:stmt)*) => {{
        // TODO: structured concurrency nursery
        // spawns all tasks, waits for all, cancels on panic
    }};
}