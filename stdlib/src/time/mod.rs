// Time: Instant, Duration, DateTime, TimeZone

pub struct Instant {
    nanos: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self { nanos: 0 }
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_nanos(0)
    }
}

pub struct Duration {
    nanos: u64,
}

impl Duration {
    pub const fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    pub const fn from_millis(millis: u64) -> Self {
        Self { nanos: millis * 1_000_000 }
    }

    pub const fn from_secs(secs: u64) -> Self {
        Self { nanos: secs * 1_000_000_000 }
    }

    pub fn as_nanos(&self) -> u64 {
        self.nanos
    }

    pub fn as_millis(&self) -> u64 {
        self.nanos / 1_000_000
    }

    pub fn as_secs(&self) -> u64 {
        self.nanos / 1_000_000_000
    }
}

pub struct DateTime {
    timestamp: i64,
}

pub struct TimeZone {
    name: String,
}