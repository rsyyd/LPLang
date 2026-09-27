// I/O: File, Stdio, Path

pub struct File {
    // ...
}

impl File {
    pub fn open(path: &Path) -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn create(path: &Path) -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        Ok(buf.len())
    }
}

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn read_line(&self, buf: &mut String) -> Result<usize, Error> {
        Ok(0)
    }
}

impl Stdout {
    pub fn write(&self, buf: &[u8]) -> Result<usize, Error> {
        Ok(buf.len())
    }
}

pub struct Path {
    path: String,
}

impl Path {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }

    pub fn join(&self, other: &str) -> Self {
        Self { path: format!("{}/{}", self.path, other) }
    }

    pub fn exists(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    InvalidInput,
    Other,
}