// Networking: TCP, UDP, HTTP, WebSocket

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub mod http {
    use super::Error;
    

    pub struct Server;
    pub struct Request;
    pub struct Response;

    impl Server {
        pub async fn bind(addr: &str) -> Result<Self, Error> {
            Ok(Self)
        }

        pub async fn serve<F>(self, handler: F) -> Result<(), Error>
        where
            F: Fn(Request) -> Response + Send + Sync + 'static,
        {
            Ok(())
        }
    }

    impl Request {
        pub fn query(&self, key: &str) -> Option<String> {
            None
        }
    }

    impl Response {
        pub fn ok(body: String) -> Self {
            Self
        }

        pub fn json<T: serde::Serialize>(value: &T) -> Self {
            Self
        }
    }
}

pub mod tcp {
    use super::Error;

    pub struct TcpListener;
    pub struct TcpStream;

    impl TcpListener {
        pub async fn bind(addr: &str) -> Result<Self, Error> {
            Ok(Self)
        }

        pub async fn accept(&self) -> Result<TcpStream, Error> {
            Ok(TcpStream)
        }
    }
}

pub mod ws {
    pub struct WebSocket;
}